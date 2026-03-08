<template>
  <div style="margin-bottom: 1.5rem;">
    <ProgressBar :value="(currentStep / totalSteps) * 100" :show-value="false" style="height: 8px;" />
    <p style="text-align: center; margin-top: 0.5rem; font-size: 0.875rem; color: var(--p-text-muted-color);">
      Paso {{ currentStep }} de {{ totalSteps }}
    </p>
  </div>
</template>

<script setup lang="ts">
defineProps<{
  currentStep: number
  totalSteps: number
}>()
</script>
