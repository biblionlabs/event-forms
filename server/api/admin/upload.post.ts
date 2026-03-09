export default defineEventHandler(async (event) => {
  const r2 = getR2(event)
  const formData = await readMultipartFormData(event)

  if (!formData || formData.length === 0) {
    throw createError({ statusCode: 400, statusMessage: 'No se envió ningún archivo' })
  }

  const file = formData[0]
  if (!file.type?.startsWith('image/')) {
    throw createError({ statusCode: 400, statusMessage: 'Solo se permiten imágenes' })
  }

  const maxSize = 5 * 1024 * 1024 // 5MB
  if (file.data.length > maxSize) {
    throw createError({ statusCode: 400, statusMessage: 'La imagen no puede superar 5MB' })
  }

  const ext = file.filename?.split('.').pop() || 'jpg'
  const key = `images/${generateId()}.${ext}`

  await r2.put(key, file.data, {
    httpMetadata: {
      contentType: file.type
    }
  })

  return {
    key,
    url: `/api/files/${key}`
  }
})
