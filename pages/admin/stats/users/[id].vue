<template>
  <div>
    <NuxtLink to="/admin/stats" style="margin-bottom: 1rem; display: inline-block;">&larr; Volver a estadísticas</NuxtLink>
    <h1>Perfil de Usuario</h1>

    <div v-if="pending" aria-busy="true">Cargando...</div>
    <template v-else-if="data">
      <article>
        <header><h3>Datos del Perfil</h3></header>
        <dl>
          <template v-for="(value, key) in data.user.data" :key="key">
            <dt>{{ key }}</dt>
            <dd>{{ value }}</dd>
          </template>
        </dl>
        <p><small>Fingerprint: <code>{{ data.user.fingerprint?.slice(0, 16) }}...</code></small></p>
        <p><small>Creado: {{ new Date(data.user.created_at).toLocaleString() }}</small></p>
      </article>

      <h3>Respuestas ({{ data.responses.length }})</h3>
      <div class="overflow-auto">
        <table>
          <thead>
            <tr>
              <th>Formulario</th>
              <th>Estado</th>
              <th>Paso</th>
              <th>Inicio</th>
              <th>Datos</th>
            </tr>
          </thead>
          <tbody>
            <tr v-for="r in data.responses" :key="r.id">
              <td>{{ r.form_name }}</td>
              <td>
                <span :style="{ color: r.status === 'completed' ? 'green' : 'orange' }">
                  {{ r.status === 'completed' ? 'Completado' : 'En progreso' }}
                </span>
              </td>
              <td>{{ r.current_step }}</td>
              <td>{{ new Date(r.started_at).toLocaleString() }}</td>
              <td>
                <details v-if="r.step_data?.length">
                  <summary>Ver datos</summary>
                  <pre>{{ JSON.stringify(r.step_data, null, 2) }}</pre>
                </details>
              </td>
            </tr>
          </tbody>
        </table>
      </div>

      <h3>Sesiones ({{ data.sessions.length }})</h3>
      <div class="overflow-auto">
        <table>
          <thead>
            <tr><th>ID</th><th>IP</th><th>País</th><th>Dispositivo</th><th>Último Acceso</th></tr>
          </thead>
          <tbody>
            <tr v-for="s in data.sessions" :key="s.id">
              <td><code>{{ s.id.slice(0, 8) }}</code></td>
              <td>{{ s.ip_address }}</td>
              <td>{{ s.country }}</td>
              <td>{{ s.device_type }}</td>
              <td>{{ new Date(s.last_active_at).toLocaleString() }}</td>
            </tr>
          </tbody>
        </table>
      </div>

      <h3>Historial de Escaneos ({{ data.scans.length }})</h3>
      <div class="overflow-auto">
        <table>
          <thead>
            <tr><th>Formulario</th><th>Paso</th><th>Nuevo</th><th>País</th><th>Dispositivo</th><th>Fecha</th></tr>
          </thead>
          <tbody>
            <tr v-for="scan in data.scans" :key="scan.id">
              <td>{{ scan.form_name }}</td>
              <td>{{ scan.step_number }}</td>
              <td>{{ scan.is_new_user ? 'Sí' : 'No' }}</td>
              <td>{{ scan.country }}</td>
              <td>{{ scan.device_type }}</td>
              <td>{{ new Date(scan.scanned_at).toLocaleString() }}</td>
            </tr>
          </tbody>
        </table>
      </div>
    </template>
  </div>
</template>

<script setup lang="ts">
definePageMeta({ layout: 'admin' })

const route = useRoute()
const { data, pending } = await useFetch(`/api/admin/stats/users/${route.params.id}`)
</script>

<style scoped>
.overflow-auto { overflow-x: auto; }
</style>
