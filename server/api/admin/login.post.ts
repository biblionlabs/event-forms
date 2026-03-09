export default defineEventHandler(async (event) => {
  const body = await readBody(event)
  const { username, password } = body

  const config = getAppConfig(event)
  if (username === config.adminUsername && password === config.adminPassword) {
    const token = btoa(`${username}:${password}`)
    setCookie(event, 'admin_session', token, {
      httpOnly: true,
      secure: true,
      sameSite: 'lax',
      maxAge: 60 * 60 * 24 * 7,
      path: '/'
    })
    return { success: true }
  }

  throw createError({ statusCode: 401, statusMessage: 'Invalid credentials' })
})
