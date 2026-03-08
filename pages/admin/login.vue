<template>
  <div style="display: flex; align-items: center; justify-content: center; min-height: 100vh; background: var(--p-surface-50);">
    <Card style="width: 380px;">
      <template #title>
        <div style="text-align: center;">
          <i class="pi pi-bolt" style="font-size: 2rem; color: var(--p-primary-color); display: block; margin-bottom: 0.5rem;" />
          Iniciar Sesión
        </div>
      </template>
      <template #content>
        <form @submit.prevent="handleLogin">
          <div style="display: flex; flex-direction: column; gap: 1.25rem;">
            <div>
              <label for="username" style="display: block; margin-bottom: 0.5rem; font-weight: 500; font-size: 0.875rem;">Usuario</label>
              <InputText
                id="username"
                v-model="username"
                placeholder="admin"
                autocomplete="username"
                style="width: 100%;"
              />
            </div>
            <div>
              <label for="password" style="display: block; margin-bottom: 0.5rem; font-weight: 500; font-size: 0.875rem;">Contraseña</label>
              <Password
                id="password"
                v-model="password"
                :feedback="false"
                toggle-mask
                input-style="width: 100%"
                style="width: 100%;"
              />
            </div>
            <Message v-if="error" severity="error" :closable="false">{{ error }}</Message>
            <Button type="submit" label="Entrar" icon="pi pi-sign-in" :loading="loading" style="width: 100%;" />
          </div>
        </form>
      </template>
    </Card>
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
