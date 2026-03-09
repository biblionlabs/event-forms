<template>
  <div>
    <label v-if="label" style="display: block; margin-bottom: 0.5rem; font-weight: 500; font-size: 0.875rem;">{{ label }}</label>
    <div
      class="image-upload-zone"
      :class="{ 'has-image': modelValue }"
      @click="triggerUpload"
      @dragover.prevent
      @drop.prevent="handleDrop"
    >
      <template v-if="modelValue">
        <img :src="modelValue" alt="Imagen subida" />
        <div style="margin-top: 0.5rem;">
          <Button type="button" icon="pi pi-trash" size="small" severity="danger" text label="Quitar" @click.stop="removeImage" />
        </div>
      </template>
      <template v-else-if="uploading">
        <ProgressSpinner style="width: 40px; height: 40px;" />
        <p style="color: var(--p-text-muted-color); margin-top: 0.5rem; font-size: 0.85rem;">Subiendo...</p>
      </template>
      <template v-else>
        <i class="pi pi-image" style="font-size: 2rem; color: var(--p-surface-400);" />
        <p style="color: var(--p-text-muted-color); margin-top: 0.5rem; font-size: 0.85rem;">
          Arrastra una imagen o haz clic para seleccionar
        </p>
        <small style="color: var(--p-surface-400);">PNG, JPG, WebP. Máx 5MB</small>
      </template>
    </div>
    <input ref="fileInput" type="file" accept="image/*" style="display: none;" @change="handleFileSelect" />
  </div>
</template>

<script setup lang="ts">
const props = defineProps<{
  modelValue?: string
  label?: string
}>()

const emit = defineEmits<{
  'update:modelValue': [value: string]
}>()

const fileInput = ref<HTMLInputElement>()
const uploading = ref(false)

function triggerUpload() {
  if (!props.modelValue) {
    fileInput.value?.click()
  }
}

function handleDrop(e: DragEvent) {
  const file = e.dataTransfer?.files[0]
  if (file && file.type.startsWith('image/')) {
    uploadFile(file)
  }
}

function handleFileSelect(e: Event) {
  const file = (e.target as HTMLInputElement).files?.[0]
  if (file) uploadFile(file)
}

async function uploadFile(file: File) {
  uploading.value = true
  try {
    const formData = new FormData()
    formData.append('file', file)
    const res = await $fetch<{ url: string }>('/api/admin/upload', {
      method: 'POST',
      body: formData
    })
    emit('update:modelValue', res.url)
  } catch (err: any) {
    console.error('Upload error:', err)
  } finally {
    uploading.value = false
    if (fileInput.value) fileInput.value.value = ''
  }
}

function removeImage() {
  emit('update:modelValue', '')
}
</script>
