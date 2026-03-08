<template>
  <div>
    <nav class="container-fluid" style="background: var(--pico-primary-background); padding: 0.75rem 1.5rem; margin-bottom: 1.5rem;">
      <ul>
        <li><strong style="color: white;">Event Forms Admin</strong></li>
      </ul>
      <ul>
        <li><NuxtLink to="/admin" style="color: white;">Dashboard</NuxtLink></li>
        <li><NuxtLink to="/admin/forms" style="color: white;">Formularios</NuxtLink></li>
        <li><NuxtLink to="/admin/stats" style="color: white;">Estadísticas</NuxtLink></li>
        <li><a href="#" @click.prevent="logout" style="color: white;">Salir</a></li>
      </ul>
    </nav>
    <main class="container">
      <slot />
    </main>
  </div>
</template>

<script setup lang="ts">
const router = useRouter()

function logout() {
  document.cookie = 'admin_session=; Max-Age=0; path=/'
  router.push('/admin/login')
}
</script>
