export default defineEventHandler(async (event) => {
  const r2 = getR2(event)
  const path = getRouterParam(event, 'path')

  if (!path) {
    throw createError({ statusCode: 400, statusMessage: 'Ruta no especificada' })
  }

  const object = await r2.get(path)
  if (!object) {
    throw createError({ statusCode: 404, statusMessage: 'Archivo no encontrado' })
  }

  const headers: Record<string, string> = {
    'Cache-Control': 'public, max-age=31536000, immutable'
  }

  if (object.httpMetadata?.contentType) {
    headers['Content-Type'] = object.httpMetadata.contentType
  }

  setHeaders(event, headers)
  return object.body
})
