<template>
  <div>
    <div style="display: flex; justify-content: space-between; align-items: center; margin-bottom: 1.5rem;">
      <h2>Formularios</h2>
      <Button label="Nuevo Formulario" icon="pi pi-plus" @click="navigateTo('/admin/forms/new')" />
    </div>

    <ProgressSpinner v-if="pending" style="display: block; margin: 3rem auto;" />
    <template v-else>
      <Card v-if="!forms?.length">
        <template #content>
          <div style="text-align: center; padding: 2rem;">
            <i class="pi pi-inbox" style="font-size: 3rem; color: var(--p-text-muted-color); display: block; margin-bottom: 1rem;" />
            <p style="color: var(--p-text-muted-color);">No hay formularios creados aún.</p>
            <Button label="Crear primer formulario" icon="pi pi-plus" @click="navigateTo('/admin/forms/new')" style="margin-top: 1rem;" />
          </div>
        </template>
      </Card>

      <Card v-else>
        <template #content>
          <DataTable :value="forms" :rows="20" striped-rows>
            <Column field="name" header="Nombre" sortable />
            <Column field="step_count" header="Pasos" sortable />
            <Column field="response_count" header="Respuestas" sortable />
            <Column header="Estado">
              <template #body="slotProps">
                <Tag
                  :value="slotProps.data.is_active ? 'Activo' : 'Inactivo'"
                  :severity="slotProps.data.is_active ? 'success' : 'danger'"
                />
              </template>
            </Column>
            <Column header="Creado">
              <template #body="slotProps">
                {{ new Date(slotProps.data.created_at).toLocaleDateString() }}
              </template>
            </Column>
            <Column header="Acciones">
              <template #body="slotProps">
                <div style="display: flex; gap: 0.5rem;">
                  <Button icon="pi pi-pencil" size="small" outlined @click="navigateTo(`/admin/forms/${slotProps.data.id}`)" />
                  <Button icon="pi pi-trash" size="small" severity="danger" outlined @click="confirmDelete(slotProps.data)" />
                </div>
              </template>
            </Column>
          </DataTable>
        </template>
      </Card>
    </template>

    <Dialog v-model:visible="deleteDialog" header="Confirmar eliminación" :modal="true" style="width: 400px;">
      <p>¿Estás seguro de eliminar el formulario <strong>{{ deleteTarget?.name }}</strong>?</p>
      <p style="color: var(--p-text-muted-color); font-size: 0.875rem;">Esta acción eliminará todos los pasos, campos y respuestas asociados.</p>
      <template #footer>
        <Button label="Cancelar" severity="secondary" @click="deleteDialog = false" />
        <Button label="Eliminar" severity="danger" icon="pi pi-trash" @click="doDelete" />
      </template>
    </Dialog>
  </div>
</template>

<script setup lang="ts">
definePageMeta({ layout: 'admin' })

const { data: forms, pending, refresh } = await useFetch<any[]>('/api/admin/forms')

const deleteDialog = ref(false)
const deleteTarget = ref<any>(null)

function confirmDelete(form: any) {
  deleteTarget.value = form
  deleteDialog.value = true
}

async function doDelete() {
  if (!deleteTarget.value) return
  await $fetch(`/api/admin/forms/${deleteTarget.value.id}`, { method: 'DELETE' })
  deleteDialog.value = false
  deleteTarget.value = null
  refresh()
}
</script>
