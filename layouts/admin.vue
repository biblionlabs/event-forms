<template>
  <div class="admin-layout">
    <aside class="admin-sidebar">
      <div class="brand">
        <i class="pi pi-bolt" />
        Event Forms
      </div>
      <nav>
        <NuxtLink to="/admin">
          <i class="pi pi-home" />
          Dashboard
        </NuxtLink>
        <NuxtLink to="/admin/forms">
          <i class="pi pi-file-edit" />
          Formularios
        </NuxtLink>
        <NuxtLink to="/admin/stats">
          <i class="pi pi-chart-bar" />
          Estadísticas
        </NuxtLink>
      </nav>
      <div class="sidebar-footer">
        <Button
          label="Cerrar sesión"
          icon="pi pi-sign-out"
          severity="secondary"
          text
          size="small"
          @click="logout"
          style="color: var(--p-surface-400); width: 100%; justify-content: flex-start;"
        />
      </div>
    </aside>
    <div class="admin-main">
      <div class="admin-content">
        <slot />
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
const router = useRouter()

function logout() {
  document.cookie = 'admin_session=; Max-Age=0; path=/'
  router.push('/admin/login')
}
</script>
