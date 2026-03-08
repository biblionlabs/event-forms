<template>
  <div style="max-width: 400px; margin: 4rem auto;">
    <article>
      <header>
        <h2>Iniciar Sesión</h2>
      </header>
      <form @submit.prevent="handleLogin">
        <label>
          Usuario
          <input v-model="username" type="text" required autocomplete="username" />
        </label>
        <label>
          Contraseña
          <input v-model="password" type="password" required autocomplete="current-password" />
        </label>
        <p v-if="error" style="color: var(--pico-del-color);">{{ error }}</p>
        <button type="submit" :disabled="loading" :aria-busy="loading">
          Entrar
        </button>
      </form>
    </article>
  </div>
</template>

<script setup lang="ts">
definePageMeta({ layout: 'default' })

const router = useRouter()
const { login } = useAdminAuth()
const username = ref('')
const password = ref('')
const error = ref('')
const loading = ref(false)

async function handleLogin() {
  loading.value = true
  error.value = ''
  try {
    await login(username.value, password.value)
    router.push('/admin')
  } catch {
    error.value = 'Credenciales inválidas'
  } finally {
    loading.value = false
  }
}
</script>
