import type { H3Event } from 'h3'

export function verifyAdminAuth(event: H3Event): boolean {
  const config = getAppConfig(event)

  const cookie = getCookie(event, 'admin_session')
  if (cookie) {
    try {
      const decoded = atob(cookie)
      const [user, pass] = decoded.split(':')
      if (user === config.adminUsername && pass === config.adminPassword) {
        return true
      }
    } catch {}
  }

  const auth = getHeader(event, 'authorization')
  if (!auth) return false

  if (auth.startsWith('Basic ')) {
    const decoded = atob(auth.slice(6))
    const [user, pass] = decoded.split(':')
    return user === config.adminUsername && pass === config.adminPassword
  }

  return false
}
