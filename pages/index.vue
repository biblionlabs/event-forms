<template>
  <div style="display: flex; align-items: center; justify-content: center; min-height: 100vh; background: var(--p-surface-50);">
    <Card style="width: 400px; text-align: center;">
      <template #title>
        <i class="pi pi-bolt" style="font-size: 2.5rem; color: var(--p-primary-color); display: block; margin-bottom: 0.5rem;" />
        Event Forms
      </template>
      <template #subtitle>
        Sistema de formularios multi-paso con códigos QR
      </template>
      <template #content>
        <p style="color: var(--p-text-muted-color); font-size: 0.9rem;">
          Crea formularios dinámicos, genera QR por paso, y rastrea respuestas con matching inteligente de usuarios.
        </p>
      </template>
      <template #footer>
        <Button label="Acceder al Panel" icon="pi pi-sign-in" @click="navigateTo('/admin/login')" style="width: 100%;" />
      </template>
    </Card>
  </div>
</template>
