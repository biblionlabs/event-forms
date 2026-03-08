import type { H3Event } from 'h3'

export function getDB(event: H3Event): D1Database {
  const { DB } = event.context.cloudflare.env
  return DB
}

export function generateId(): string {
  return crypto.randomUUID().replace(/-/g, '').slice(0, 16)
}
