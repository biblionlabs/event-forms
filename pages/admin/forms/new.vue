<template>
  <div>
    <h1>Nuevo Formulario</h1>
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
