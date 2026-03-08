<template>
  <div>
    <h1>Estadísticas</h1>

    <div v-if="selectedFormId">
      <button class="outline" @click="selectedFormId = ''">Ver Todas</button>
      <div v-if="formStatsPending" aria-busy="true">Cargando...</div>
      <template v-else-if="formStatsData">
        <h2>{{ formStatsData.form.name }}</h2>

        <h3>Embudo de Conversión</h3>
        <div class="overflow-auto">
          <table>
            <thead>
              <tr>
                <th>Paso</th>
                <th>Título</th>
                <th>Escaneos</th>
                <th>Visitantes Únicos</th>
                <th>Nuevos</th>
                <th>Completados</th>
                <th>Tasa</th>
              </tr>
            </thead>
            <tbody>
              <tr v-for="step in formStatsData.funnel" :key="step.step_number">
                <td>{{ step.step_number }}</td>
                <td>{{ step.title }}</td>
                <td>{{ step.scans }}</td>
                <td>{{ step.unique_visitors }}</td>
                <td>{{ step.new_users }}</td>
                <td>{{ step.completions }}</td>
                <td>
                  {{ step.unique_visitors > 0 ? Math.round((step.completions / step.unique_visitors) * 100) : 0 }}%
                </td>
              </tr>
            </tbody>
          </table>
        </div>

        <h3>Escaneos por Día</h3>
        <div class="overflow-auto">
          <table>
            <thead><tr><th>Fecha</th><th>Escaneos</th></tr></thead>
            <tbody>
              <tr v-for="day in formStatsData.scanTimeline" :key="day.date">
                <td>{{ day.date }}</td>
                <td>{{ day.count }}</td>
              </tr>
            </tbody>
          </table>
        </div>

        <h3>Respuestas Recientes</h3>
        <div class="overflow-auto">
          <table>
            <thead>
              <tr>
                <th>Estado</th>
                <th>Paso Actual</th>
                <th>Inicio</th>
                <th>Datos del Usuario</th>
                <th>Acciones</th>
              </tr>
            </thead>
            <tbody>
              <tr v-for="r in formStatsData.recentResponses" :key="r.id">
                <td>
                  <span :style="{ color: r.status === 'completed' ? 'green' : 'orange' }">
                    {{ r.status === 'completed' ? 'Completado' : 'En progreso' }}
                  </span>
                </td>
                <td>{{ r.current_step }}</td>
                <td>{{ new Date(r.started_at).toLocaleString() }}</td>
                <td>{{ r.user_data ? summarizeUserData(r.user_data) : '-' }}</td>
                <td>
                  <NuxtLink v-if="r.user_profile_id" :to="`/admin/stats/users/${r.user_profile_id}`">
                    Ver Usuario
                  </NuxtLink>
                </td>
              </tr>
            </tbody>
          </table>
        </div>
      </template>
    </div>

    <template v-else>
      <div v-if="pending" aria-busy="true">Cargando...</div>
      <template v-else-if="stats">
        <div class="grid">
          <article>
            <header>Total Escaneos</header>
            <h2>{{ stats.overview.totalScans }}</h2>
          </article>
          <article>
            <header>Usuarios Únicos</header>
            <h2>{{ stats.overview.uniqueUsers }}</h2>
          </article>
          <article>
            <header>Tasa de Completitud</header>
            <h2>{{ stats.overview.completionRate }}%</h2>
          </article>
        </div>

        <h3>Estadísticas por Formulario</h3>
        <div class="overflow-auto">
          <table>
            <thead>
              <tr>
                <th>Formulario</th>
                <th>Escaneos</th>
                <th>Visitantes</th>
                <th>Respuestas</th>
                <th>Completados</th>
                <th></th>
              </tr>
            </thead>
            <tbody>
              <tr v-for="form in stats.formStats" :key="form.id">
                <td>{{ form.name }}</td>
                <td>{{ form.scan_count }}</td>
                <td>{{ form.unique_visitors }}</td>
                <td>{{ form.response_count }}</td>
                <td>{{ form.completed_count }}</td>
                <td>
                  <button class="outline" @click="selectedFormId = form.id">
                    Detalle
                  </button>
                </td>
              </tr>
            </tbody>
          </table>
        </div>

        <h3>Usuarios</h3>
        <NuxtLink to="/admin/stats" @click.prevent="loadUsers">Ver todos los usuarios</NuxtLink>
        <div v-if="usersData" class="overflow-auto" style="margin-top: 1rem;">
          <table>
            <thead>
              <tr>
                <th>ID</th>
                <th>Datos</th>
                <th>Formularios</th>
                <th>Sesiones</th>
                <th>Último Acceso</th>
                <th>País</th>
                <th></th>
              </tr>
            </thead>
            <tbody>
              <tr v-for="user in usersData.users" :key="user.id">
                <td><code>{{ user.id.slice(0, 8) }}</code></td>
                <td>{{ summarizeUserData(JSON.stringify(user.data)) }}</td>
                <td>{{ user.forms_answered }}</td>
                <td>{{ user.total_sessions }}</td>
                <td>{{ user.last_seen ? new Date(user.last_seen).toLocaleString() : '-' }}</td>
                <td>{{ user.last_country || '-' }}</td>
                <td>
                  <NuxtLink :to="`/admin/stats/users/${user.id}`">Ver</NuxtLink>
                </td>
              </tr>
            </tbody>
          </table>
        </div>
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

watch(selectedFormId, (val) => {
  if (val) fetchFormStats()
}, { immediate: true })

async function loadUsers() {
  usersData.value = await $fetch('/api/admin/stats/users')
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

<style scoped>
.overflow-auto { overflow-x: auto; }
</style>
