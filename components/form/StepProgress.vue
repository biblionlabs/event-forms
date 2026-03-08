<template>
  <div class="step-progress">
    <div class="progress-bar">
      <div class="progress-fill" :style="{ width: `${(currentStep / totalSteps) * 100}%` }"></div>
    </div>
    <p class="progress-text">Paso {{ currentStep }} de {{ totalSteps }}</p>
  </div>
</template>

<script setup lang="ts">
defineProps<{
  currentStep: number
  totalSteps: number
}>()
</script>

<style scoped>
.step-progress {
  margin-bottom: 1.5rem;
}
.progress-bar {
  width: 100%;
  height: 8px;
  background: var(--pico-muted-border-color);
  border-radius: 4px;
  overflow: hidden;
}
.progress-fill {
  height: 100%;
  background: var(--pico-primary);
  border-radius: 4px;
  transition: width 0.3s ease;
}
.progress-text {
  text-align: center;
  margin-top: 0.5rem;
  font-size: 0.875rem;
  color: var(--pico-muted-color);
}
</style>
