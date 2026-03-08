<template>
  <div>
    <div style="display: flex; align-items: center; gap: 1rem; margin-bottom: 1.5rem;">
      <Button icon="pi pi-arrow-left" severity="secondary" text @click="navigateTo('/admin/forms')" />
      <h2 style="margin: 0;">Nuevo Formulario</h2>
    </div>
    <AdminFormBuilder @save="createForm" />
  </div>
</template>

<script setup lang="ts">
definePageMeta({ layout: 'admin' })

const router = useRouter()

async function createForm(formData: any) {
  const res = await $fetch<{ id: string }>('/api/admin/forms', {
    method: 'POST',
    body: formData
  })
  router.push(`/admin/forms/${res.id}`)
}
</script>
