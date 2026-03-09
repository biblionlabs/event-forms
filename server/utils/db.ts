import type { H3Event } from 'h3'

interface AppConfig {
  adminUsername: string
  adminPassword: string
  cookieSecret: string
}

export function getAppConfig(event: H3Event): AppConfig {
  const cf = event.context.cloudflare?.env
  const rc = useRuntimeConfig(event)
  return {
    adminUsername: cf?.ADMIN_USERNAME || rc.adminUsername || 'admin',
    adminPassword: cf?.ADMIN_PASSWORD || rc.adminPassword || 'changeme123',
    cookieSecret: cf?.COOKIE_SECRET || rc.cookieSecret || 'dev-secret-change-in-production-32ch'
  }
}

export function getDB(event: H3Event): D1Database {
  const cf = event.context.cloudflare?.env
  if (!cf?.DB) {
    throw createError({
      statusCode: 503,
      statusMessage: 'Base de datos no disponible. Usa "wrangler pages dev dist/" para probar con D1 localmente.'
    })
  }
  return cf.DB
}

export function getR2(event: H3Event): R2Bucket {
  const cf = event.context.cloudflare?.env
  if (!cf?.R2) {
    throw createError({
      statusCode: 503,
      statusMessage: 'R2 no disponible. Usa "wrangler pages dev dist/" para probar con R2 localmente.'
    })
  }
  return cf.R2
}

export function generateId(): string {
  return crypto.randomUUID().replace(/-/g, '').slice(0, 16)
}
