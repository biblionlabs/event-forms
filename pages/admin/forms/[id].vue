<template>
  <div>
    <div style="display: flex; justify-content: space-between; align-items: center; margin-bottom: 1.5rem;">
      <div style="display: flex; align-items: center; gap: 1rem;">
        <Button icon="pi pi-arrow-left" severity="secondary" text @click="navigateTo('/admin/forms')" />
        <h2 style="margin: 0;">Editar Formulario</h2>
      </div>
    </div>

    <ProgressSpinner v-if="pending" style="display: block; margin: 3rem auto;" />
    <template v-else-if="form">
      <AdminFormBuilder :initial-data="form" @save="updateForm" />

      <Divider />

      <!-- QR Code Section -->
      <Card style="margin-top: 1.5rem;">
        <template #title>
          <div style="display: flex; align-items: center; gap: 0.5rem;">
            <i class="pi pi-qrcode" />
            Enlaces QR
          </div>
        </template>
        <template #subtitle>
          Cada paso tiene una URL única que puedes convertir en código QR imprimible.
        </template>
        <template #content>
          <DataTable :value="form.steps" :rows="20">
            <Column field="step_number" header="Paso" style="width: 60px;" />
            <Column field="title" header="Título" />
            <Column header="URL">
              <template #body="slotProps">
                <code style="font-size: 0.8rem; background: var(--p-surface-100); padding: 0.25rem 0.5rem; border-radius: 4px;">
                  {{ getStepUrl(slotProps.data.step_number) }}
                </code>
              </template>
            </Column>
            <Column header="QR" style="width: 120px;">
              <template #body="slotProps">
                <Button label="Ver QR" icon="pi pi-qrcode" size="small" outlined @click="showQR(slotProps.data.step_number)" />
              </template>
            </Column>
          </DataTable>
        </template>
      </Card>
    </template>

    <!-- QR Dialog -->
    <Dialog v-model:visible="qrDialogVisible" :header="`Código QR - Paso ${qrStep}`" :modal="true" style="width: 420px;">
      <div style="text-align: center; padding: 1rem;">
        <canvas ref="qrCanvas" style="margin: 0 auto; display: block;" />
        <p style="margin-top: 1rem;">
          <code style="font-size: 0.8rem; background: var(--p-surface-100); padding: 0.25rem 0.5rem; border-radius: 4px;">
            {{ getStepUrl(qrStep) }}
          </code>
        </p>
      </div>
      <template #footer>
        <Button label="Descargar PNG" icon="pi pi-download" @click="downloadQR" />
      </template>
    </Dialog>

    <Toast />
  </div>
</template>

<script setup lang="ts">
import { useToast } from 'primevue/usetoast'

definePageMeta({ layout: 'admin' })

const route = useRoute()
const id = route.params.id as string
const toast = useToast()

const { data: form, pending } = await useFetch(`/api/admin/forms/${id}`)

const qrDialogVisible = ref(false)
const qrStep = ref(1)
const qrCanvas = ref<HTMLCanvasElement>()

function getStepUrl(stepNumber: number): string {
  const origin = typeof window !== 'undefined' ? window.location.origin : ''
  return `${origin}/f/${id}/${stepNumber}`
}

async function showQR(stepNumber: number) {
  qrStep.value = stepNumber
  qrDialogVisible.value = true
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
  await $fetch(`/api/admin/forms/${id}`, { method: 'PUT', body: formData })
  toast.add({ severity: 'success', summary: 'Guardado', detail: 'Formulario actualizado correctamente', life: 3000 })
}
</script>
