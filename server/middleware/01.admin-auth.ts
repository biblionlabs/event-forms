export default defineEventHandler((event) => {
  const path = getRequestURL(event).pathname
  if (path.startsWith('/api/admin') && !path.endsWith('/login')) {
    if (!verifyAdminAuth(event)) {
      throw createError({ statusCode: 401, statusMessage: 'Unauthorized' })
    }
  }
})
