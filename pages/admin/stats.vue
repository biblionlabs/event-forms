<template>
  <div>
    <h2 style="margin-bottom: 1.5rem;">Estadísticas</h2>

    <!-- Form Detail View -->
    <template v-if="selectedFormId">
      <Button label="Ver Todas" icon="pi pi-arrow-left" severity="secondary" text @click="selectedFormId = ''" style="margin-bottom: 1rem;" />

      <ProgressSpinner v-if="formStatsPending" style="display: block; margin: 3rem auto;" />
      <template v-else-if="formStatsData">
        <h3>{{ formStatsData.form.name }}</h3>

        <!-- Funnel -->
        <Card style="margin-bottom: 1.5rem;">
          <template #title>Embudo de Conversión</template>
          <template #content>
            <DataTable :value="formStatsData.funnel" striped-rows>
              <Column field="step_number" header="Paso" style="width: 60px;" />
              <Column field="title" header="Título" />
              <Column field="scans" header="Escaneos" sortable />
              <Column field="unique_visitors" header="Visitantes Únicos" sortable />
              <Column field="new_users" header="Nuevos" sortable />
              <Column field="completions" header="Completados" sortable />
              <Column header="Tasa">
                <template #body="slotProps">
                  <Tag
                    :value="`${slotProps.data.unique_visitors > 0 ? Math.round((slotProps.data.completions / slotProps.data.unique_visitors) * 100) : 0}%`"
                    :severity="slotProps.data.unique_visitors > 0 && (slotProps.data.completions / slotProps.data.unique_visitors) >= 0.5 ? 'success' : 'warn'"
                  />
                </template>
              </Column>
            </DataTable>
          </template>
        </Card>

        <!-- Timeline -->
        <Card style="margin-bottom: 1.5rem;">
          <template #title>Escaneos por Día</template>
          <template #content>
            <DataTable :value="formStatsData.scanTimeline" :rows="15" paginator striped-rows>
              <Column field="date" header="Fecha" sortable />
              <Column field="count" header="Escaneos" sortable />
            </DataTable>
          </template>
        </Card>

        <!-- Recent Responses -->
        <Card>
          <template #title>Respuestas Recientes</template>
          <template #content>
            <DataTable :value="formStatsData.recentResponses" :rows="15" paginator striped-rows>
              <Column header="Estado">
                <template #body="slotProps">
                  <Tag
                    :value="slotProps.data.status === 'completed' ? 'Completado' : 'En progreso'"
                    :severity="slotProps.data.status === 'completed' ? 'success' : 'warn'"
                  />
                </template>
              </Column>
              <Column field="current_step" header="Paso Actual" />
              <Column header="Inicio">
                <template #body="slotProps">
                  {{ new Date(slotProps.data.started_at).toLocaleString() }}
                </template>
              </Column>
              <Column header="Datos del Usuario">
                <template #body="slotProps">
                  {{ slotProps.data.user_data ? summarizeUserData(slotProps.data.user_data) : '-' }}
                </template>
              </Column>
              <Column header="Acciones" style="width: 100px;">
                <template #body="slotProps">
                  <Button
                    v-if="slotProps.data.user_profile_id"
                    label="Ver"
                    size="small"
                    outlined
                    @click="navigateTo(`/admin/stats/users/${slotProps.data.user_profile_id}`)"
                  />
                </template>
              </Column>
            </DataTable>
          </template>
        </Card>
      </template>
    </template>

    <!-- Overview -->
    <template v-else>
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
                <div class="stat-value">{{ stats.overview.completionRate }}%</div>
                <div class="stat-label">Tasa de Completitud</div>
              </div>
            </template>
          </Card>
        </div>

        <!-- Stats by Form -->
        <Card style="margin-bottom: 1.5rem;">
          <template #title>Estadísticas por Formulario</template>
          <template #content>
            <DataTable :value="stats.formStats" striped-rows>
              <Column field="name" header="Formulario" />
              <Column field="scan_count" header="Escaneos" sortable />
              <Column field="unique_visitors" header="Visitantes" sortable />
              <Column field="response_count" header="Respuestas" sortable />
              <Column field="completed_count" header="Completados" sortable />
              <Column header="" style="width: 100px;">
                <template #body="slotProps">
                  <Button label="Detalle" size="small" outlined @click="selectedFormId = slotProps.data.id" />
                </template>
              </Column>
            </DataTable>
          </template>
        </Card>

        <!-- Users -->
        <Card>
          <template #title>
            <div style="display: flex; justify-content: space-between; align-items: center;">
              <span>Usuarios</span>
              <Button label="Cargar usuarios" icon="pi pi-users" size="small" severity="secondary" outlined @click="loadUsers" :loading="loadingUsers" />
            </div>
          </template>
          <template #content>
            <DataTable v-if="usersData" :value="usersData.users" :rows="15" paginator striped-rows>
              <Column header="ID" style="width: 100px;">
                <template #body="slotProps">
                  <code style="font-size: 0.8rem;">{{ slotProps.data.id.slice(0, 8) }}</code>
                </template>
              </Column>
              <Column header="Datos">
                <template #body="slotProps">
                  {{ summarizeUserData(JSON.stringify(slotProps.data.data)) }}
                </template>
              </Column>
              <Column field="forms_answered" header="Formularios" sortable />
              <Column field="total_sessions" header="Sesiones" sortable />
              <Column header="Último Acceso">
                <template #body="slotProps">
                  {{ slotProps.data.last_seen ? new Date(slotProps.data.last_seen).toLocaleString() : '-' }}
                </template>
              </Column>
              <Column field="last_country" header="País" />
              <Column header="" style="width: 80px;">
                <template #body="slotProps">
                  <Button icon="pi pi-eye" size="small" text @click="navigateTo(`/admin/stats/users/${slotProps.data.id}`)" />
                </template>
              </Column>
            </DataTable>
            <p v-else style="color: var(--p-text-muted-color); text-align: center;">Haz clic en "Cargar usuarios" para ver la lista.</p>
          </template>
        </Card>
      </template>
    </template>
  </div>
</template>

<script setup lang="ts">
definePageMeta({ layout: 'admin' })

const route = useRoute()
const selectedFormId = ref((route.query.form as string) || '')

const { data: stats, pending } = await useFetch('/api/admin/stats')

const { data: formStatsData, pending: formStatsPending, execute: fetchFormStats } = await useFetch(
  () => `/api/admin/stats/forms/${selectedFormId.value}`,
  { immediate: false, watch: false }
)

const usersData = ref<any>(null)
const loadingUsers = ref(false)

watch(selectedFormId, (val) => {
  if (val) fetchFormStats()
}, { immediate: true })

async function loadUsers() {
  loadingUsers.value = true
  try {
    usersData.value = await $fetch('/api/admin/stats/users')
  } finally {
    loadingUsers.value = false
  }
}

function summarizeUserData(data: string): string {
  try {
    const obj = typeof data === 'string' ? JSON.parse(data) : data
    const entries = Object.entries(obj).slice(0, 3)
    return entries.map(([k, v]) => `${k}: ${v}`).join(', ')
  } catch {
    return '-'
  }
}
</script>
