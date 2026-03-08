export async function generateFingerprint(fields: Record<string, string>, identifierKeys: string[]): Promise<string> {
  const sortedKeys = identifierKeys.sort()
  const values = sortedKeys.map(k => (fields[k] || '').trim().toLowerCase())
  const raw = values.join('|')

  const encoder = new TextEncoder()
  const data = encoder.encode(raw)
  const hash = await crypto.subtle.digest('SHA-256', data)
  const array = new Uint8Array(hash)
  return Array.from(array).map(b => b.toString(16).padStart(2, '0')).join('')
}

export async function encryptCookieValue(value: string, secret: string): Promise<string> {
  const encoder = new TextEncoder()
  const keyData = encoder.encode(secret.padEnd(32, '0').slice(0, 32))
  const iv = crypto.getRandomValues(new Uint8Array(12))

  const key = await crypto.subtle.importKey('raw', keyData, 'AES-GCM', false, ['encrypt'])
  const encrypted = await crypto.subtle.encrypt({ name: 'AES-GCM', iv }, key, encoder.encode(value))

  const combined = new Uint8Array(iv.length + new Uint8Array(encrypted).length)
  combined.set(iv)
  combined.set(new Uint8Array(encrypted), iv.length)

  return btoa(String.fromCharCode(...combined))
}

export async function decryptCookieValue(encrypted: string, secret: string): Promise<string | null> {
  try {
    const encoder = new TextEncoder()
    const keyData = encoder.encode(secret.padEnd(32, '0').slice(0, 32))
    const combined = Uint8Array.from(atob(encrypted), c => c.charCodeAt(0))

    const iv = combined.slice(0, 12)
    const data = combined.slice(12)

    const key = await crypto.subtle.importKey('raw', keyData, 'AES-GCM', false, ['decrypt'])
    const decrypted = await crypto.subtle.decrypt({ name: 'AES-GCM', iv }, key, data)

    return new TextDecoder().decode(decrypted)
  } catch {
    return null
  }
}
