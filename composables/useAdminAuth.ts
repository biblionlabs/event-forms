export function useAdminAuth() {
  const isAuthenticated = useState('admin_auth', () => false)

  async function login(username: string, password: string) {
    const res = await $fetch('/api/admin/login', {
      method: 'POST',
      body: { username, password }
    })
    isAuthenticated.value = true
    return res
  }

  async function checkAuth() {
    try {
      await $fetch('/api/admin/forms', { method: 'GET' })
      isAuthenticated.value = true
      return true
    } catch {
      isAuthenticated.value = false
      return false
    }
  }

  return { isAuthenticated, login, checkAuth }
}
