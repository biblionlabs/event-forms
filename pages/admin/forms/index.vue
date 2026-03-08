<template>
  <div>
    <div style="display: flex; justify-content: space-between; align-items: center;">
      <h1>Formularios</h1>
      <NuxtLink to="/admin/forms/new" role="button">+ Nuevo Formulario</NuxtLink>
    </div>

    <div v-if="pending" aria-busy="true">Cargando...</div>
    <template v-else>
      <p v-if="!forms?.length">No hay formularios creados aún.</p>
      <div class="overflow-auto">
        <table v-if="forms?.length">
          <thead>
            <tr>
              <th>Nombre</th>
              <th>Pasos</th>
              <th>Respuestas</th>
              <th>Estado</th>
              <th>Creado</th>
              <th>Acciones</th>
            </tr>
          </thead>
          <tbody>
            <tr v-for="form in forms" :key="form.id">
              <td>{{ form.name }}</td>
              <td>{{ form.step_count }}</td>
              <td>{{ form.response_count }}</td>
              <td>
                <span :style="{ color: form.is_active ? 'green' : 'red' }">
                  {{ form.is_active ? 'Activo' : 'Inactivo' }}
                </span>
              </td>
              <td>{{ new Date(form.created_at).toLocaleDateString() }}</td>
              <td>
                <NuxtLink :to="`/admin/forms/${form.id}`">Editar</NuxtLink> |
                <a href="#" @click.prevent="deleteForm(form.id)">Eliminar</a>
              </td>
            </tr>
          </tbody>
        </table>
      </div>
    </template>
  </div>
</template>

<script setup lang="ts">
definePageMeta({ layout: 'admin' })

const { data: forms, pending, refresh } = await useFetch<any[]>('/api/admin/forms')

async function deleteForm(id: string) {
  if (!confirm('¿Estás seguro de eliminar este formulario?')) return
  await $fetch(`/api/admin/forms/${id}`, { method: 'DELETE' })
  refresh()
}
</script>

<style scoped>
.overflow-auto { overflow-x: auto; }
</style>
