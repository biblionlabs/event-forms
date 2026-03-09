import type { H3Event } from 'h3'

export interface GeoData {
  ip: string
  country: string
  city: string
  region: string
  timezone: string
  deviceType: string
  userAgent: string
}

export function extractGeoData(event: H3Event): GeoData {
  const headers = getHeaders(event)
  const userAgent = headers['user-agent'] || ''

  let deviceType = 'desktop'
  if (/mobile|android|iphone|ipad/i.test(userAgent)) {
    deviceType = /ipad|tablet/i.test(userAgent) ? 'tablet' : 'mobile'
  }

  return {
    ip: headers['cf-connecting-ip'] || headers['x-forwarded-for'] || '0.0.0.0',
    country: headers['cf-ipcountry'] || '',
    city: (headers as Record<string, string>)['cf-ipcity'] || '',
    region: (headers as Record<string, string>)['cf-ipregion'] || '',
    timezone: (headers as Record<string, string>)['cf-timezone'] || '',
    deviceType,
    userAgent
  }
}

export async function getOrCreateSession(event: H3Event): Promise<{ sessionId: string; isNew: boolean; profileId: string | null; fingerprint: string | null }> {
  const db = getDB(event)
  let sessionId = getCookie(event, 'ef_session')
  let isNew = false
  let profileId: string | null = null
  let fingerprint: string | null = null

  if (sessionId) {
    const session = await db.prepare('SELECT id, user_profile_id, fingerprint FROM sessions WHERE id = ?').bind(sessionId).first()
    if (session) {
      profileId = session.user_profile_id as string | null
      fingerprint = session.fingerprint as string | null
      await db.prepare('UPDATE sessions SET last_active_at = datetime(\'now\') WHERE id = ?').bind(sessionId).run()
      return { sessionId, isNew: false, profileId, fingerprint }
    }
  }

  sessionId = generateId()
  isNew = true
  const geo = extractGeoData(event)

  const fpCookie = getCookie(event, 'ef_fingerprint')
  const secret = getAppConfig(event).cookieSecret
  if (fpCookie) {
    fingerprint = await decryptCookieValue(fpCookie, secret)
    if (fingerprint) {
      const profile = await db.prepare('SELECT id FROM user_profiles WHERE fingerprint = ?').bind(fingerprint).first()
      if (profile) {
        profileId = profile.id as string
      }
    }
  }

  await db.prepare(
    `INSERT INTO sessions (id, user_profile_id, fingerprint, ip_address, user_agent, country, city, region, device_type)
     VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?)`
  ).bind(sessionId, profileId, fingerprint, geo.ip, geo.userAgent, geo.country, geo.city, geo.region, geo.deviceType).run()

  setCookie(event, 'ef_session', sessionId, {
    httpOnly: true,
    secure: true,
    sameSite: 'lax',
    maxAge: 60 * 60 * 24 * 365,
    path: '/'
  })

  return { sessionId, isNew, profileId, fingerprint }
}
