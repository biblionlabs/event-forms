<template>
  <div>
    <div style="display: flex; justify-content: space-between; align-items: center;">
      <h1>Editar Formulario</h1>
      <NuxtLink to="/admin/forms" role="button" class="outline">Volver</NuxtLink>
    </div>

    <div v-if="pending" aria-busy="true">Cargando...</div>
    <template v-else-if="form">
      <AdminFormBuilder :initial-data="form" @save="updateForm" />

      <hr />
      <h3>Enlaces QR</h3>
      <p>Cada paso tiene una URL única que puedes convertir en código QR:</p>
      <div class="overflow-auto">
        <table>
          <thead>
            <tr>
              <th>Paso</th>
              <th>Título</th>
              <th>URL</th>
              <th>QR</th>
            </tr>
          </thead>
          <tbody>
            <tr v-for="step in form.steps" :key="step.id">
              <td>{{ step.step_number }}</td>
              <td>{{ step.title }}</td>
              <td>
                <code>{{ getStepUrl(step.step_number) }}</code>
              </td>
              <td>
                <button class="outline" @click="showQR(step.step_number)">Ver QR</button>
              </td>
            </tr>
          </tbody>
        </table>
      </div>

      <dialog :open="qrDialog">
        <article style="max-width: 400px;">
          <header>
            <button aria-label="Close" rel="prev" @click="qrDialog = false"></button>
            <h3>Código QR - Paso {{ qrStep }}</h3>
          </header>
          <div style="text-align: center;">
            <canvas ref="qrCanvas"></canvas>
            <p><code>{{ getStepUrl(qrStep) }}</code></p>
          </div>
          <footer>
            <button @click="downloadQR">Descargar QR</button>
          </footer>
        </article>
      </dialog>
    </template>
  </div>
</template>

<script setup lang="ts">
definePageMeta({ layout: 'admin' })

const route = useRoute()
const id = route.params.id as string

const { data: form, pending } = await useFetch(`/api/admin/forms/${id}`)

const qrDialog = ref(false)
const qrStep = ref(1)
const qrCanvas = ref<HTMLCanvasElement>()

function getStepUrl(stepNumber: number): string {
  const origin = typeof window !== 'undefined' ? window.location.origin : ''
  return `${origin}/f/${id}/${stepNumber}`
}

async function showQR(stepNumber: number) {
  qrStep.value = stepNumber
  qrDialog.value = true
  await nextTick()
  if (qrCanvas.value) {
    const QRCode = (await import('qrcode')).default
    QRCode.toCanvas(qrCanvas.value, getStepUrl(stepNumber), { width: 300, margin: 2 })
  }
}

function downloadQR() {
  if (!qrCanvas.value) return
  const link = document.createElement('a')
  link.download = `qr-form-${id}-step-${qrStep.value}.png`
  link.href = qrCanvas.value.toDataURL()
  link.click()
}

async function updateForm(formData: any) {
  await $fetch(`/api/admin/forms/${id}`, {
    method: 'PUT',
    body: formData
  })
  alert('Formulario actualizado correctamente')
}
</script>

<style scoped>
.overflow-auto { overflow-x: auto; }
</style>
