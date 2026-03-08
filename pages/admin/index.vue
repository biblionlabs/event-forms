<template>
  <div>
    <h2 style="margin-bottom: 1.5rem;">Dashboard</h2>

    <ProgressSpinner v-if="pending" style="display: block; margin: 3rem auto;" />
    <template v-else-if="stats">
      <!-- Overview Cards -->
      <div style="display: grid; grid-template-columns: repeat(auto-fit, minmax(200px, 1fr)); gap: 1rem; margin-bottom: 2rem;">
        <Card>
          <template #content>
            <div class="stat-card">
              <div class="stat-value">{{ stats.overview.totalScans }}</div>
              <div class="stat-label">Total Escaneos</div>
            </div>
          </template>
        </Card>
        <Card>
          <template #content>
            <div class="stat-card">
              <div class="stat-value">{{ stats.overview.uniqueUsers }}</div>
              <div class="stat-label">Usuarios Únicos</div>
            </div>
          </template>
        </Card>
        <Card>
          <template #content>
            <div class="stat-card">
              <div class="stat-value">{{ stats.overview.totalResponses }}</div>
              <div class="stat-label">Respuestas</div>
            </div>
          </template>
        </Card>
        <Card>
          <template #content>
            <div class="stat-card">
              <div class="stat-value">{{ stats.overview.completedResponses }} <small style="font-size: 0.5em; color: var(--p-text-muted-color);">({{ stats.overview.completionRate }}%)</small></div>
              <div class="stat-label">Completados</div>
            </div>
          </template>
        </Card>
      </div>

      <!-- Active Forms Table -->
      <Card style="margin-bottom: 2rem;">
        <template #title>Formularios Activos</template>
        <template #content>
          <DataTable :value="stats.formStats" :rows="10" striped-rows>
            <Column field="name" header="Formulario" />
            <Column field="scan_count" header="Escaneos" sortable />
            <Column field="unique_visitors" header="Visitantes Únicos" sortable />
            <Column field="response_count" header="Respuestas" sortable />
            <Column field="completed_count" header="Completados" sortable />
            <Column header="Acciones">
              <template #body="slotProps">
                <div style="display: flex; gap: 0.5rem;">
                  <Button
                    label="Stats"
                    icon="pi pi-chart-bar"
                    size="small"
                    severity="info"
                    outlined
                    @click="navigateTo(`/admin/stats?form=${slotProps.data.id}`)"
                  />
                  <Button
                    label="Editar"
                    icon="pi pi-pencil"
                    size="small"
                    outlined
                    @click="navigateTo(`/admin/forms/${slotProps.data.id}`)"
                  />
                </div>
              </template>
            </Column>
          </DataTable>
        </template>
      </Card>

      <!-- Device & Country Stats -->
      <div style="display: grid; grid-template-columns: 1fr 1fr; gap: 1rem; margin-bottom: 2rem;">
        <Card>
          <template #title>Dispositivos</template>
          <template #content>
            <div v-for="d in stats.deviceStats" :key="d.device_type" style="display: flex; justify-content: space-between; padding: 0.5rem 0; border-bottom: 1px solid var(--p-surface-200);">
              <span style="text-transform: capitalize;">{{ d.device_type }}</span>
              <Tag :value="String(d.count)" />
            </div>
            <p v-if="!stats.deviceStats?.length" style="color: var(--p-text-muted-color);">Sin datos</p>
          </template>
        </Card>
        <Card>
          <template #title>Países</template>
          <template #content>
            <div v-for="c in stats.countryStats" :key="c.country" style="display: flex; justify-content: space-between; padding: 0.5rem 0; border-bottom: 1px solid var(--p-surface-200);">
              <span>{{ c.country || 'Desconocido' }}</span>
              <Tag :value="String(c.count)" />
            </div>
            <p v-if="!stats.countryStats?.length" style="color: var(--p-text-muted-color);">Sin datos</p>
          </template>
        </Card>
      </div>

      <!-- Recent Scans -->
      <Card>
        <template #title>Escaneos Recientes</template>
        <template #content>
          <DataTable :value="stats.recentScans?.slice(0, 20)" :rows="20" striped-rows>
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

const { data: stats, pending } = await useFetch('/api/admin/stats')
</script>
