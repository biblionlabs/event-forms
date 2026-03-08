export default defineEventHandler(async (event) => {
  const body = await readBody(event)
  const { username, password } = body

  const env = event.context.cloudflare.env
  if (username === env.ADMIN_USERNAME && password === env.ADMIN_PASSWORD) {
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
