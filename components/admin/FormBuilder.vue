<template>
  <form @submit.prevent="save">
    <!-- General Info -->
    <Card style="margin-bottom: 1.5rem;">
      <template #title>Información General</template>
      <template #content>
        <div style="display: flex; flex-direction: column; gap: 1rem;">
          <div>
            <label class="field-label">Nombre del formulario *</label>
            <InputText v-model="form.name" required style="width: 100%;" />
          </div>
          <div>
            <label class="field-label">Descripción</label>
            <Textarea v-model="form.description" rows="2" style="width: 100%;" />
          </div>
          <div style="display: grid; grid-template-columns: 1fr 1fr; gap: 1rem;">
            <div>
              <label class="field-label">Título de agradecimiento final</label>
              <InputText v-model="form.thank_you_title" style="width: 100%;" />
            </div>
            <div>
              <label class="field-label">Mensaje de agradecimiento final</label>
              <Textarea v-model="form.thank_you_message" rows="2" style="width: 100%;" />
            </div>
          </div>
          <AdminImageUpload
            v-model="form.thank_you_image_url"
            label="Imagen de agradecimiento final (opcional)"
          />
        </div>
      </template>
    </Card>

    <!-- Info banner about field checkboxes -->
    <Message severity="info" :closable="false" style="margin-bottom: 1rem;">
      <span style="font-size: 0.85rem;">
        <strong>Identificador</strong> = genera fingerprint para matching entre formularios.
        <strong>Cookie</strong> = se guarda en el navegador.
        <strong>Sesión</strong> = requerido para sesión válida.
        Marca estos en cada campo abajo.
      </span>
    </Message>

    <!-- Steps -->
    <div style="display: flex; justify-content: space-between; align-items: center; margin-bottom: 1rem;">
      <h3 style="margin: 0;">Pasos del Formulario</h3>
      <Button type="button" label="Agregar Paso" icon="pi pi-plus" severity="secondary" @click="addStep" />
    </div>

    <div ref="stepsContainer">
      <Card v-for="(step, si) in form.steps" :key="step._key" class="step-card" :data-index="si">
        <template #title>
          <div style="display: flex; justify-content: space-between; align-items: center;">
            <div style="display: flex; align-items: center; gap: 0.5rem;">
              <i class="pi pi-bars drag-handle" title="Arrastrar para reordenar" />
              <span>Paso {{ si + 1 }}</span>
              <Tag v-if="step.is_final" value="Final" severity="success" />
            </div>
            <div style="display: flex; gap: 0.5rem;">
              <Button type="button" icon="pi pi-trash" size="small" severity="danger" text @click="removeStep(si)" :disabled="form.steps.length <= 1" />
            </div>
          </div>
        </template>
        <template #content>
          <div style="display: flex; flex-direction: column; gap: 1rem;">
            <div style="display: grid; grid-template-columns: 1fr 1fr; gap: 1rem;">
              <div>
                <label class="field-label">Título del paso *</label>
                <InputText v-model="step.title" required style="width: 100%;" />
              </div>
              <div>
                <label class="field-label">Descripción</label>
                <InputText v-model="step.description" style="width: 100%;" />
              </div>
            </div>

            <!-- Step thank you (collapsible) -->
            <Panel header="Mensaje al completar este paso" toggleable collapsed>
              <div style="display: grid; grid-template-columns: 1fr 1fr; gap: 1rem;">
                <div>
                  <label class="field-label">Título</label>
                  <InputText v-model="step.thank_you_title" style="width: 100%;" />
                </div>
                <div>
                  <label class="field-label">Mensaje</label>
                  <Textarea v-model="step.thank_you_message" rows="2" style="width: 100%;" />
                </div>
              </div>
              <AdminImageUpload
                v-model="step.thank_you_image_url"
                label="Imagen (opcional)"
                style="margin-top: 0.75rem;"
              />
            </Panel>

            <div style="display: flex; align-items: center; gap: 0.5rem;">
              <Checkbox v-model="step.is_final" :binary="true" :input-id="`final-${si}`" />
              <label :for="`final-${si}`" style="font-size: 0.875rem;">Es paso final (completa el formulario)</label>
            </div>

            <!-- Fields -->
            <Divider />
            <div style="display: flex; justify-content: space-between; align-items: center;">
              <h4 style="margin: 0;">Campos</h4>
              <Button type="button" label="Agregar Campo" icon="pi pi-plus" size="small" severity="secondary" outlined @click="addField(si)" />
            </div>

            <div v-for="(field, fi) in step.fields" :key="fi" class="field-card">
              <div style="display: grid; grid-template-columns: 1fr 1fr 1fr; gap: 0.75rem;">
                <div>
                  <label class="field-label-sm">Clave *</label>
                  <InputText v-model="field.field_key" required placeholder="email, nombre..." size="small" style="width: 100%;" />
                </div>
                <div>
                  <label class="field-label-sm">Etiqueta *</label>
                  <InputText v-model="field.label" required placeholder="Tu correo" size="small" style="width: 100%;" />
                </div>
                <div>
                  <label class="field-label-sm">Tipo</label>
                  <Select v-model="field.field_type" :options="fieldTypes" option-label="label" option-value="value" size="small" style="width: 100%;" />
                </div>
              </div>

              <div style="display: grid; grid-template-columns: 1fr 1fr; gap: 0.75rem; margin-top: 0.75rem;">
                <div>
                  <label class="field-label-sm">Placeholder</label>
                  <InputText v-model="field.placeholder" size="small" style="width: 100%;" />
                </div>
                <div v-if="['select', 'radio', 'checkbox'].includes(field.field_type)">
                  <label class="field-label-sm">Opciones (una por línea)</label>
                  <Textarea v-model="field._optionsStr" rows="3" style="width: 100%;" @input="parseOptions(field)" />
                </div>
              </div>

              <!-- Validations (collapsible) -->
              <Panel header="Validaciones" toggleable collapsed style="margin-top: 0.75rem;">
                <div style="display: grid; grid-template-columns: 1fr 1fr 1fr; gap: 0.75rem;">
                  <div>
                    <label class="field-label-sm">Patrón (regex)</label>
                    <InputText v-model="field._validations.pattern" size="small" style="width: 100%;" />
                  </div>
                  <div>
                    <label class="field-label-sm">Mín. caracteres</label>
                    <InputNumber v-model="field._validations.minLength" size="small" style="width: 100%;" />
                  </div>
                  <div>
                    <label class="field-label-sm">Máx. caracteres</label>
                    <InputNumber v-model="field._validations.maxLength" size="small" style="width: 100%;" />
                  </div>
                </div>
              </Panel>

              <!-- Field flags & delete -->
              <div style="display: flex; gap: 1.25rem; align-items: center; margin-top: 0.75rem; flex-wrap: wrap;">
                <div class="flag-check">
                  <Checkbox v-model="field.is_required" :binary="true" :input-id="`req-${si}-${fi}`" />
                  <label :for="`req-${si}-${fi}`">Requerido</label>
                </div>
                <div class="flag-check">
                  <Checkbox v-model="field.is_identifier" :binary="true" :input-id="`id-${si}-${fi}`" />
                  <label :for="`id-${si}-${fi}`">
                    <i class="pi pi-fingerprint" style="font-size: 0.75rem;" /> Identificador
                  </label>
                </div>
                <div class="flag-check">
                  <Checkbox v-model="field.is_cookie" :binary="true" :input-id="`ck-${si}-${fi}`" />
                  <label :for="`ck-${si}-${fi}`">
                    <i class="pi pi-database" style="font-size: 0.75rem;" /> Cookie
                  </label>
                </div>
                <div class="flag-check">
                  <Checkbox v-model="field.is_session_required" :binary="true" :input-id="`sr-${si}-${fi}`" />
                  <label :for="`sr-${si}-${fi}`">
                    <i class="pi pi-verified" style="font-size: 0.75rem;" /> Sesión
                  </label>
                </div>
                <Button type="button" icon="pi pi-times" size="small" severity="danger" text style="margin-left: auto;" @click="removeField(si, fi)" />
              </div>
            </div>
          </div>
        </template>
      </Card>
    </div>

    <!-- Summary of configured flags -->
    <Card v-if="identifierSummary.length || cookieSummary.length || sessionRequiredSummary.length" style="margin-top: 1rem; margin-bottom: 1rem; background: var(--p-surface-50);">
      <template #content>
        <div style="display: flex; gap: 2rem; flex-wrap: wrap; font-size: 0.85rem;">
          <div v-if="identifierSummary.length">
            <strong><i class="pi pi-fingerprint" /> Fingerprint:</strong>
            <Tag v-for="k in identifierSummary" :key="k" :value="k" severity="info" style="margin-left: 0.25rem;" />
          </div>
          <div v-if="cookieSummary.length">
            <strong><i class="pi pi-database" /> Cookies:</strong>
            <Tag v-for="k in cookieSummary" :key="k" :value="k" severity="warn" style="margin-left: 0.25rem;" />
          </div>
          <div v-if="sessionRequiredSummary.length">
            <strong><i class="pi pi-verified" /> Sesión:</strong>
            <Tag v-for="k in sessionRequiredSummary" :key="k" :value="k" severity="success" style="margin-left: 0.25rem;" />
          </div>
        </div>
      </template>
    </Card>

    <div style="display: flex; justify-content: flex-end; gap: 1rem; margin-top: 1.5rem;">
      <Button type="submit" label="Guardar Formulario" icon="pi pi-check" />
    </div>
  </form>
</template>

<script setup lang="ts">
import Sortable from 'sortablejs'

const props = defineProps<{ initialData?: any }>()
const emit = defineEmits<{ save: [data: any] }>()

const stepsContainer = ref<HTMLElement>()
let sortableInstance: Sortable | null = null

const fieldTypes = [
  { label: 'Texto', value: 'text' },
  { label: 'Email', value: 'email' },
  { label: 'Teléfono', value: 'phone' },
  { label: 'Número', value: 'number' },
  { label: 'Área de texto', value: 'textarea' },
  { label: 'Selección', value: 'select' },
  { label: 'Radio', value: 'radio' },
  { label: 'Checkbox', value: 'checkbox' },
  { label: 'Fecha', value: 'date' },
  { label: 'URL', value: 'url' }
]

let keyCounter = 0

interface FieldData {
  id?: string
  field_key: string
  label: string
  field_type: string
  placeholder: string
  options: string[]
  is_required: boolean
  is_identifier: boolean
  is_cookie: boolean
  is_session_required: boolean
  sort_order: number
  _optionsStr: string
  _validations: { pattern?: string; minLength?: number; maxLength?: number }
}

interface StepData {
  id?: string
  _key: number
  step_number: number
  title: string
  description: string
  thank_you_title: string
  thank_you_message: string
  thank_you_image_url: string
  is_final: boolean
  fields: FieldData[]
}

function createField(): FieldData {
  return {
    field_key: '',
    label: '',
    field_type: 'text',
    placeholder: '',
    options: [],
    is_required: false,
    is_identifier: false,
    is_cookie: false,
    is_session_required: false,
    sort_order: 0,
    _optionsStr: '',
    _validations: {}
  }
}

function createStep(num: number): StepData {
  return {
    _key: keyCounter++,
    step_number: num,
    title: '',
    description: '',
    thank_you_title: '',
    thank_you_message: '',
    thank_you_image_url: '',
    is_final: false,
    fields: [createField()]
  }
}

const form = reactive({
  name: '',
  description: '',
  thank_you_title: 'Gracias',
  thank_you_message: 'Tu respuesta ha sido registrada.',
  thank_you_image_url: '',
  is_active: true,
  steps: [createStep(1)] as StepData[]
})

// Computed summaries from field checkboxes
const identifierSummary = computed(() => {
  const keys: string[] = []
  form.steps.forEach(s => s.fields.forEach(f => {
    if (f.is_identifier && f.field_key) keys.push(f.field_key)
  }))
  return [...new Set(keys)]
})

const cookieSummary = computed(() => {
  const keys: string[] = []
  form.steps.forEach(s => s.fields.forEach(f => {
    if (f.is_cookie && f.field_key) keys.push(f.field_key)
  }))
  return [...new Set(keys)]
})

const sessionRequiredSummary = computed(() => {
  const keys: string[] = []
  form.steps.forEach(s => s.fields.forEach(f => {
    if (f.is_session_required && f.field_key) keys.push(f.field_key)
  }))
  return [...new Set(keys)]
})

if (props.initialData) {
  Object.assign(form, {
    ...props.initialData,
    steps: props.initialData.steps?.map((s: any, i: number) => ({
      ...s,
      _key: keyCounter++,
      step_number: s.step_number || i + 1,
      is_final: !!s.is_final,
      fields: s.fields?.map((f: any) => ({
        ...f,
        is_required: !!f.is_required,
        is_identifier: !!f.is_identifier,
        is_cookie: !!f.is_cookie,
        is_session_required: !!f.is_session_required,
        options: typeof f.options === 'string' ? JSON.parse(f.options) : (f.options || []),
        _optionsStr: (typeof f.options === 'string' ? JSON.parse(f.options) : (f.options || [])).join('\n'),
        _validations: typeof f.validations === 'string' ? JSON.parse(f.validations) : (f.validations || {})
      })) || [createField()]
    })) || [createStep(1)]
  })
}

function parseOptions(field: FieldData) {
  field.options = field._optionsStr.split('\n').map(s => s.trim()).filter(Boolean)
}

function addStep() {
  form.steps.push(createStep(form.steps.length + 1))
}

function removeStep(index: number) {
  if (form.steps.length <= 1) return
  form.steps.splice(index, 1)
  form.steps.forEach((s, i) => { s.step_number = i + 1 })
}

function addField(stepIndex: number) {
  form.steps[stepIndex].fields.push(createField())
}

function removeField(stepIndex: number, fieldIndex: number) {
  form.steps[stepIndex].fields.splice(fieldIndex, 1)
}

// Initialize SortableJS for step drag and drop
onMounted(() => {
  if (stepsContainer.value) {
    sortableInstance = new Sortable(stepsContainer.value, {
      animation: 200,
      handle: '.drag-handle',
      ghostClass: 'sortable-ghost',
      dragClass: 'sortable-drag',
      onEnd(evt) {
        if (evt.oldIndex == null || evt.newIndex == null) return
        const item = form.steps.splice(evt.oldIndex, 1)[0]
        form.steps.splice(evt.newIndex, 0, item)
        form.steps.forEach((s, i) => { s.step_number = i + 1 })
      }
    })
  }
})

onUnmounted(() => {
  sortableInstance?.destroy()
})

function save() {
  const data = {
    ...toRaw(form),
    identifier_fields: identifierSummary.value,
    cookie_fields: cookieSummary.value,
    session_required_fields: sessionRequiredSummary.value,
    steps: form.steps.map((step, si) => ({
      ...toRaw(step),
      step_number: si + 1,
      is_final: si === form.steps.length - 1 || step.is_final,
      fields: step.fields.map((field, fi) => ({
        field_key: field.field_key,
        label: field.label,
        field_type: field.field_type,
        placeholder: field.placeholder,
        options: field.options,
        validations: field._validations,
        is_required: field.is_required,
        is_identifier: field.is_identifier,
        is_cookie: field.is_cookie,
        is_session_required: field.is_session_required,
        sort_order: fi
      }))
    }))
  }

  emit('save', data)
}
</script>

<style scoped>
.field-label {
  display: block;
  margin-bottom: 0.5rem;
  font-weight: 500;
  font-size: 0.875rem;
}

.field-label-sm {
  display: block;
  margin-bottom: 0.25rem;
  font-size: 0.8rem;
  color: var(--p-text-muted-color);
}

.flag-check {
  display: flex;
  align-items: center;
  gap: 0.4rem;
}

.flag-check label {
  font-size: 0.85rem;
  display: flex;
  align-items: center;
  gap: 0.2rem;
}
</style>
