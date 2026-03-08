<template>
  <div style="margin-bottom: 1.25rem;">
    <label v-if="field.field_type !== 'checkbox' || field.options?.length" style="display: block; margin-bottom: 0.5rem; font-weight: 500; font-size: 0.875rem;">
      {{ field.label }}
      <span v-if="field.is_required" style="color: var(--p-red-500);">*</span>
    </label>

    <!-- Text / URL -->
    <InputText
      v-if="field.field_type === 'text' || field.field_type === 'url'"
      :model-value="modelValue"
      @update:model-value="emit('update:modelValue', $event)"
      :placeholder="field.placeholder"
      :invalid="!!error"
      style="width: 100%;"
    />

    <!-- Email -->
    <InputText
      v-else-if="field.field_type === 'email'"
      :model-value="modelValue"
      @update:model-value="emit('update:modelValue', $event)"
      type="email"
      :placeholder="field.placeholder || 'correo@ejemplo.com'"
      :invalid="!!error"
      style="width: 100%;"
    />

    <!-- Phone -->
    <InputText
      v-else-if="field.field_type === 'phone'"
      :model-value="modelValue"
      @update:model-value="emit('update:modelValue', $event)"
      :placeholder="field.placeholder || '+1234567890'"
      :invalid="!!error"
      style="width: 100%;"
    />

    <!-- Number -->
    <InputNumber
      v-else-if="field.field_type === 'number'"
      :model-value="modelValue ? Number(modelValue) : null"
      @update:model-value="emit('update:modelValue', $event != null ? String($event) : '')"
      :placeholder="field.placeholder"
      :invalid="!!error"
      style="width: 100%;"
    />

    <!-- Date -->
    <DatePicker
      v-else-if="field.field_type === 'date'"
      :model-value="modelValue ? new Date(modelValue) : null"
      @update:model-value="emit('update:modelValue', $event ? formatDate($event as Date) : '')"
      date-format="yy-mm-dd"
      :invalid="!!error"
      style="width: 100%;"
    />

    <!-- Textarea -->
    <Textarea
      v-else-if="field.field_type === 'textarea'"
      :model-value="modelValue"
      @update:model-value="emit('update:modelValue', $event)"
      :placeholder="field.placeholder"
      rows="3"
      :invalid="!!error"
      style="width: 100%;"
    />

    <!-- Select -->
    <Select
      v-else-if="field.field_type === 'select'"
      :model-value="modelValue"
      @update:model-value="emit('update:modelValue', $event)"
      :options="field.options"
      :placeholder="field.placeholder || 'Selecciona una opción'"
      :invalid="!!error"
      style="width: 100%;"
    />

    <!-- Radio -->
    <div v-else-if="field.field_type === 'radio'" style="display: flex; flex-direction: column; gap: 0.5rem;">
      <div v-for="opt in field.options" :key="opt" style="display: flex; align-items: center; gap: 0.5rem;">
        <RadioButton
          :model-value="modelValue"
          @update:model-value="emit('update:modelValue', $event)"
          :input-id="`${field.field_key}-${opt}`"
          :name="field.field_key"
          :value="opt"
        />
        <label :for="`${field.field_key}-${opt}`">{{ opt }}</label>
      </div>
    </div>

    <!-- Checkbox (multiple) -->
    <template v-else-if="field.field_type === 'checkbox'">
      <div v-if="field.options?.length" style="display: flex; flex-direction: column; gap: 0.5rem;">
        <div v-for="opt in field.options" :key="opt" style="display: flex; align-items: center; gap: 0.5rem;">
          <Checkbox
            :model-value="(modelValue || '').split(',').filter(Boolean)"
            @update:model-value="emit('update:modelValue', ($event as string[]).join(','))"
            :input-id="`${field.field_key}-${opt}`"
            :value="opt"
          />
          <label :for="`${field.field_key}-${opt}`">{{ opt }}</label>
        </div>
      </div>
      <div v-else style="display: flex; align-items: center; gap: 0.5rem;">
        <Checkbox
          :model-value="modelValue === 'true' || modelValue === true"
          @update:model-value="emit('update:modelValue', $event ? 'true' : '')"
          :binary="true"
          :input-id="field.field_key"
        />
        <label :for="field.field_key">
          {{ field.label }}
          <span v-if="field.is_required" style="color: var(--p-red-500);">*</span>
        </label>
      </div>
    </template>

    <small v-if="error" style="color: var(--p-red-500); display: block; margin-top: 0.25rem;">{{ error }}</small>
  </div>
</template>

<script setup lang="ts">
defineProps<{
  field: {
    field_key: string
    label: string
    field_type: string
    placeholder?: string
    options?: string[]
    validations?: { pattern?: string; minLength?: number; maxLength?: number }
    is_required?: boolean
  }
  modelValue: any
  error?: string
}>()

const emit = defineEmits<{
  'update:modelValue': [value: any]
}>()

function formatDate(date: Date): string {
  return date.toISOString().split('T')[0]
}
</script>
