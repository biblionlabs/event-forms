<template>
  <div>
    <h1>Dashboard</h1>
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
          <header>Respuestas</header>
          <h2>{{ stats.overview.totalResponses }}</h2>
        </article>
        <article>
          <header>Completados</header>
          <h2>{{ stats.overview.completedResponses }} ({{ stats.overview.completionRate }}%)</h2>
        </article>
      </div>

      <h3>Formularios Activos</h3>
      <div class="overflow-auto">
        <table>
          <thead>
            <tr>
              <th>Formulario</th>
              <th>Escaneos</th>
              <th>Visitantes Únicos</th>
              <th>Respuestas</th>
              <th>Completados</th>
              <th>Acciones</th>
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
                <NuxtLink :to="`/admin/stats?form=${form.id}`">Ver Stats</NuxtLink> |
                <NuxtLink :to="`/admin/forms/${form.id}`">Editar</NuxtLink>
              </td>
            </tr>
          </tbody>
        </table>
      </div>

      <div class="grid">
        <article>
          <header>Dispositivos</header>
          <ul>
            <li v-for="d in stats.deviceStats" :key="d.device_type">
              {{ d.device_type }}: {{ d.count }}
            </li>
          </ul>
        </article>
        <article>
          <header>Países</header>
          <ul>
            <li v-for="c in stats.countryStats" :key="c.country">
              {{ c.country }}: {{ c.count }}
            </li>
          </ul>
        </article>
      </div>

      <h3>Escaneos Recientes</h3>
      <div class="overflow-auto">
        <table>
          <thead>
            <tr>
              <th>Formulario</th>
              <th>Paso</th>
              <th>Nuevo</th>
              <th>País</th>
              <th>Dispositivo</th>
              <th>Fecha</th>
            </tr>
          </thead>
          <tbody>
            <tr v-for="scan in stats.recentScans?.slice(0, 20)" :key="scan.id">
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

const { data: stats, pending } = await useFetch('/api/admin/stats')
</script>

<style scoped>
.overflow-auto { overflow-x: auto; }
</style>
