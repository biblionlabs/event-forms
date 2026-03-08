<template>
  <div>
    <label v-if="field.field_type !== 'checkbox'" :for="field.field_key">
      {{ field.label }}
      <span v-if="field.is_required" style="color: var(--pico-del-color);">*</span>
    </label>

    <input
      v-if="field.field_type === 'text' || field.field_type === 'url'"
      :id="field.field_key"
      :type="field.field_type"
      :placeholder="field.placeholder"
      :required="field.is_required"
      :value="modelValue"
      @input="emit('update:modelValue', ($event.target as HTMLInputElement).value)"
      :aria-invalid="error ? true : undefined"
    />

    <input
      v-else-if="field.field_type === 'email'"
      :id="field.field_key"
      type="email"
      :placeholder="field.placeholder || 'correo@ejemplo.com'"
      :required="field.is_required"
      :value="modelValue"
      @input="emit('update:modelValue', ($event.target as HTMLInputElement).value)"
      :aria-invalid="error ? true : undefined"
    />

    <input
      v-else-if="field.field_type === 'phone'"
      :id="field.field_key"
      type="tel"
      :placeholder="field.placeholder || '+1234567890'"
      :required="field.is_required"
      :value="modelValue"
      @input="emit('update:modelValue', ($event.target as HTMLInputElement).value)"
      :aria-invalid="error ? true : undefined"
    />

    <input
      v-else-if="field.field_type === 'number'"
      :id="field.field_key"
      type="number"
      :placeholder="field.placeholder"
      :required="field.is_required"
      :value="modelValue"
      @input="emit('update:modelValue', ($event.target as HTMLInputElement).value)"
      :aria-invalid="error ? true : undefined"
    />

    <input
      v-else-if="field.field_type === 'date'"
      :id="field.field_key"
      type="date"
      :required="field.is_required"
      :value="modelValue"
      @input="emit('update:modelValue', ($event.target as HTMLInputElement).value)"
      :aria-invalid="error ? true : undefined"
    />

    <textarea
      v-else-if="field.field_type === 'textarea'"
      :id="field.field_key"
      :placeholder="field.placeholder"
      :required="field.is_required"
      :value="modelValue"
      @input="emit('update:modelValue', ($event.target as HTMLTextAreaElement).value)"
      rows="3"
      :aria-invalid="error ? true : undefined"
    ></textarea>

    <select
      v-else-if="field.field_type === 'select'"
      :id="field.field_key"
      :required="field.is_required"
      :value="modelValue"
      @change="emit('update:modelValue', ($event.target as HTMLSelectElement).value)"
      :aria-invalid="error ? true : undefined"
    >
      <option value="" disabled>{{ field.placeholder || 'Selecciona una opción' }}</option>
      <option v-for="opt in field.options" :key="opt" :value="opt">{{ opt }}</option>
    </select>

    <fieldset v-else-if="field.field_type === 'radio'">
      <label v-for="opt in field.options" :key="opt">
        <input
          type="radio"
          :name="field.field_key"
          :value="opt"
          :checked="modelValue === opt"
          @change="emit('update:modelValue', opt)"
        />
        {{ opt }}
      </label>
    </fieldset>

    <template v-else-if="field.field_type === 'checkbox'">
      <label v-if="field.options?.length">
        {{ field.label }}
        <span v-if="field.is_required" style="color: var(--pico-del-color);">*</span>
      </label>
      <fieldset v-if="field.options?.length">
        <label v-for="opt in field.options" :key="opt">
          <input
            type="checkbox"
            :value="opt"
            :checked="(modelValue || '').split(',').includes(opt)"
            @change="toggleCheckbox(opt)"
          />
          {{ opt }}
        </label>
      </fieldset>
      <label v-else>
        <input
          type="checkbox"
          :checked="modelValue === 'true' || modelValue === true"
          @change="emit('update:modelValue', ($event.target as HTMLInputElement).checked ? 'true' : '')"
        />
        {{ field.label }}
        <span v-if="field.is_required" style="color: var(--pico-del-color);">*</span>
      </label>
    </template>

    <small v-if="error" style="color: var(--pico-del-color);">{{ error }}</small>
  </div>
</template>

<script setup lang="ts">
const props = defineProps<{
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

function toggleCheckbox(opt: string) {
  const current = (props.modelValue || '').split(',').filter(Boolean) as string[]
  const idx = current.indexOf(opt)
  if (idx >= 0) current.splice(idx, 1)
  else current.push(opt)
  emit('update:modelValue', current.join(','))
}
</script>
