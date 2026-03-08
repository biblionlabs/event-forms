<template>
  <div>
    <div style="display: flex; align-items: center; gap: 1rem; margin-bottom: 1.5rem;">
      <Button icon="pi pi-arrow-left" severity="secondary" text @click="navigateTo('/admin/stats')" />
      <h2 style="margin: 0;">Perfil de Usuario</h2>
    </div>

    <ProgressSpinner v-if="pending" style="display: block; margin: 3rem auto;" />
    <template v-else-if="data">
      <!-- Profile -->
      <Card style="margin-bottom: 1.5rem;">
        <template #title>Datos del Perfil</template>
        <template #content>
          <div style="display: grid; grid-template-columns: repeat(auto-fit, minmax(200px, 1fr)); gap: 0.75rem;">
            <div v-for="(value, key) in data.user.data" :key="key" style="background: var(--p-surface-50); padding: 0.75rem; border-radius: var(--p-border-radius);">
              <div style="font-size: 0.75rem; color: var(--p-text-muted-color); text-transform: uppercase; margin-bottom: 0.25rem;">{{ key }}</div>
              <div style="font-weight: 500;">{{ value }}</div>
            </div>
          </div>
          <div style="margin-top: 1rem; display: flex; gap: 2rem; font-size: 0.85rem; color: var(--p-text-muted-color);">
            <span>Fingerprint: <code>{{ data.user.fingerprint?.slice(0, 16) }}...</code></span>
            <span>Creado: {{ new Date(data.user.created_at).toLocaleString() }}</span>
          </div>
        </template>
      </Card>

      <!-- Responses -->
      <Card style="margin-bottom: 1.5rem;">
        <template #title>Respuestas ({{ data.responses.length }})</template>
        <template #content>
          <DataTable :value="data.responses" :rows="10" paginator striped-rows>
            <Column field="form_name" header="Formulario" />
            <Column header="Estado">
              <template #body="slotProps">
                <Tag
                  :value="slotProps.data.status === 'completed' ? 'Completado' : 'En progreso'"
                  :severity="slotProps.data.status === 'completed' ? 'success' : 'warn'"
                />
              </template>
            </Column>
            <Column field="current_step" header="Paso" />
            <Column header="Inicio">
              <template #body="slotProps">
                {{ new Date(slotProps.data.started_at).toLocaleString() }}
              </template>
            </Column>
            <Column header="Datos">
              <template #body="slotProps">
                <Button
                  v-if="slotProps.data.step_data?.length"
                  label="Ver datos"
                  size="small"
                  text
                  @click="toggleStepData(slotProps.data.id)"
                />
                <pre v-if="expandedRows[slotProps.data.id]" style="font-size: 0.75rem; background: var(--p-surface-50); padding: 0.5rem; border-radius: 4px; margin-top: 0.5rem; white-space: pre-wrap;">{{ JSON.stringify(slotProps.data.step_data, null, 2) }}</pre>
              </template>
            </Column>
          </DataTable>
        </template>
      </Card>

      <!-- Sessions -->
      <Card style="margin-bottom: 1.5rem;">
        <template #title>Sesiones ({{ data.sessions.length }})</template>
        <template #content>
          <DataTable :value="data.sessions" :rows="10" paginator striped-rows>
            <Column header="ID" style="width: 100px;">
              <template #body="slotProps">
                <code style="font-size: 0.8rem;">{{ slotProps.data.id.slice(0, 8) }}</code>
              </template>
            </Column>
            <Column field="ip_address" header="IP" />
            <Column field="country" header="País" />
            <Column field="device_type" header="Dispositivo" />
            <Column header="Último Acceso">
              <template #body="slotProps">
                {{ new Date(slotProps.data.last_active_at).toLocaleString() }}
              </template>
            </Column>
          </DataTable>
        </template>
      </Card>

      <!-- Scan History -->
      <Card>
        <template #title>Historial de Escaneos ({{ data.scans.length }})</template>
        <template #content>
          <DataTable :value="data.scans" :rows="15" paginator striped-rows>
            <Column field="form_name" header="Formulario" />
            <Column field="step_number" header="Paso" />
            <Column header="Nuevo">
              <template #body="slotProps">
                <Tag :value="slotProps.data.is_new_user ? 'Sí' : 'No'" :severity="slotProps.data.is_new_user ? 'success' : 'secondary'" />
              </template>
            </Column>
            <Column field="country" header="País" />
            <Column field="device_type" header="Dispositivo" />
            <Column header="Fecha">
              <template #body="slotProps">
                {{ new Date(slotProps.data.scanned_at).toLocaleString() }}
              </template>
            </Column>
          </DataTable>
        </template>
      </Card>
    </template>
  </div>
</template>

<script setup lang="ts">
definePageMeta({ layout: 'admin' })

const route = useRoute()
const { data, pending } = await useFetch(`/api/admin/stats/users/${route.params.id}`)

const expandedRows = reactive<Record<string, boolean>>({})

function toggleStepData(id: string) {
  expandedRows[id] = !expandedRows[id]
}
</script>
