<template>
  <form @submit.prevent="save">
    <!-- General Info -->
    <Card style="margin-bottom: 1.5rem;">
      <template #title>Información General</template>
      <template #content>
        <div style="display: flex; flex-direction: column; gap: 1rem;">
          <div>
            <label style="display: block; margin-bottom: 0.5rem; font-weight: 500; font-size: 0.875rem;">Nombre del formulario *</label>
            <InputText v-model="form.name" required style="width: 100%;" />
          </div>
          <div>
            <label style="display: block; margin-bottom: 0.5rem; font-weight: 500; font-size: 0.875rem;">Descripción</label>
            <Textarea v-model="form.description" rows="2" style="width: 100%;" />
          </div>
          <div style="display: grid; grid-template-columns: 1fr 1fr; gap: 1rem;">
            <div>
              <label style="display: block; margin-bottom: 0.5rem; font-weight: 500; font-size: 0.875rem;">Título de agradecimiento final</label>
              <InputText v-model="form.thank_you_title" style="width: 100%;" />
            </div>
            <div>
              <label style="display: block; margin-bottom: 0.5rem; font-weight: 500; font-size: 0.875rem;">URL de imagen de agradecimiento</label>
              <InputText v-model="form.thank_you_image_url" placeholder="https://..." style="width: 100%;" />
            </div>
          </div>
          <div>
            <label style="display: block; margin-bottom: 0.5rem; font-weight: 500; font-size: 0.875rem;">Mensaje de agradecimiento final</label>
            <Textarea v-model="form.thank_you_message" rows="2" style="width: 100%;" />
          </div>
        </div>
      </template>
    </Card>

    <!-- Session & Identification Config -->
    <Card style="margin-bottom: 1.5rem;">
      <template #title>Configuración de Sesión e Identificación</template>
      <template #subtitle>Define qué campos se usan para identificar usuarios entre formularios.</template>
      <template #content>
        <div style="display: flex; flex-direction: column; gap: 1.25rem;">
          <div>
            <label style="display: block; margin-bottom: 0.5rem; font-weight: 500; font-size: 0.875rem;">Campos identificadores (fingerprint)</label>
            <Chips v-model="identifierChips" separator="," placeholder="email, phone..." style="width: 100%;" />
            <small style="color: var(--p-text-muted-color);">Combinación de campos que generan un hash único para identificar al usuario.</small>
          </div>
          <div>
            <label style="display: block; margin-bottom: 0.5rem; font-weight: 500; font-size: 0.875rem;">Campos a guardar en cookies</label>
            <Chips v-model="cookieChips" separator="," placeholder="email, name..." style="width: 100%;" />
            <small style="color: var(--p-text-muted-color);">Campos que se almacenan en cookies para pre-llenar en futuras visitas.</small>
          </div>
          <div>
            <label style="display: block; margin-bottom: 0.5rem; font-weight: 500; font-size: 0.875rem;">Campos requeridos para sesión completa</label>
            <Chips v-model="sessionRequiredChips" separator="," placeholder="email..." style="width: 100%;" />
            <small style="color: var(--p-text-muted-color);">Campos que deben estar completos para considerar la sesión válida.</small>
          </div>
        </div>
      </template>
    </Card>

    <!-- Steps -->
    <div style="display: flex; justify-content: space-between; align-items: center; margin-bottom: 1rem;">
      <h3 style="margin: 0;">Pasos del Formulario</h3>
      <Button type="button" label="Agregar Paso" icon="pi pi-plus" severity="secondary" @click="addStep" />
    </div>

    <Card v-for="(step, si) in form.steps" :key="si" style="margin-bottom: 1rem;">
      <template #title>
        <div style="display: flex; justify-content: space-between; align-items: center;">
          <span>Paso {{ si + 1 }}</span>
          <div style="display: flex; gap: 0.5rem;">
            <Button type="button" icon="pi pi-arrow-up" size="small" severity="secondary" text :disabled="si === 0" @click="moveStep(si, -1)" />
            <Button type="button" icon="pi pi-arrow-down" size="small" severity="secondary" text :disabled="si === form.steps.length - 1" @click="moveStep(si, 1)" />
            <Button type="button" icon="pi pi-trash" size="small" severity="danger" text @click="removeStep(si)" :disabled="form.steps.length <= 1" />
          </div>
        </div>
      </template>
      <template #content>
        <div style="display: flex; flex-direction: column; gap: 1rem;">
          <div style="display: grid; grid-template-columns: 1fr 1fr; gap: 1rem;">
            <div>
              <label style="display: block; margin-bottom: 0.5rem; font-weight: 500; font-size: 0.875rem;">Título del paso *</label>
              <InputText v-model="step.title" required style="width: 100%;" />
            </div>
            <div>
              <label style="display: block; margin-bottom: 0.5rem; font-weight: 500; font-size: 0.875rem;">Descripción</label>
              <InputText v-model="step.description" style="width: 100%;" />
            </div>
          </div>

          <!-- Step thank you (collapsible) -->
          <Panel header="Mensaje al completar este paso" toggleable collapsed>
            <div style="display: grid; grid-template-columns: 1fr 1fr; gap: 1rem;">
              <div>
                <label style="display: block; margin-bottom: 0.5rem; font-weight: 500; font-size: 0.875rem;">Título</label>
                <InputText v-model="step.thank_you_title" style="width: 100%;" />
              </div>
              <div>
                <label style="display: block; margin-bottom: 0.5rem; font-weight: 500; font-size: 0.875rem;">URL de imagen</label>
                <InputText v-model="step.thank_you_image_url" placeholder="https://..." style="width: 100%;" />
              </div>
            </div>
            <div style="margin-top: 0.75rem;">
              <label style="display: block; margin-bottom: 0.5rem; font-weight: 500; font-size: 0.875rem;">Mensaje</label>
              <Textarea v-model="step.thank_you_message" rows="2" style="width: 100%;" />
            </div>
          </Panel>

          <div>
            <Checkbox v-model="step.is_final" :binary="true" input-id="final" />
            <label for="final" style="margin-left: 0.5rem; font-size: 0.875rem;">Es paso final (completa el formulario)</label>
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
                <label style="display: block; margin-bottom: 0.25rem; font-size: 0.8rem; color: var(--p-text-muted-color);">Clave *</label>
                <InputText v-model="field.field_key" required placeholder="email, nombre..." size="small" style="width: 100%;" />
              </div>
              <div>
                <label style="display: block; margin-bottom: 0.25rem; font-size: 0.8rem; color: var(--p-text-muted-color);">Etiqueta *</label>
                <InputText v-model="field.label" required placeholder="Tu correo" size="small" style="width: 100%;" />
              </div>
              <div>
                <label style="display: block; margin-bottom: 0.25rem; font-size: 0.8rem; color: var(--p-text-muted-color);">Tipo</label>
                <Select v-model="field.field_type" :options="fieldTypes" option-label="label" option-value="value" size="small" style="width: 100%;" />
              </div>
            </div>

            <div style="display: grid; grid-template-columns: 1fr 1fr; gap: 0.75rem; margin-top: 0.75rem;">
              <div>
                <label style="display: block; margin-bottom: 0.25rem; font-size: 0.8rem; color: var(--p-text-muted-color);">Placeholder</label>
                <InputText v-model="field.placeholder" size="small" style="width: 100%;" />
              </div>
              <div v-if="['select', 'radio', 'checkbox'].includes(field.field_type)">
                <label style="display: block; margin-bottom: 0.25rem; font-size: 0.8rem; color: var(--p-text-muted-color);">Opciones (una por línea)</label>
                <Textarea v-model="field._optionsStr" rows="3" style="width: 100%;" @input="parseOptions(field)" />
              </div>
            </div>

            <!-- Validations (collapsible) -->
            <Panel header="Validaciones" toggleable collapsed style="margin-top: 0.75rem;">
              <div style="display: grid; grid-template-columns: 1fr 1fr 1fr; gap: 0.75rem;">
                <div>
                  <label style="display: block; margin-bottom: 0.25rem; font-size: 0.8rem; color: var(--p-text-muted-color);">Patrón (regex)</label>
                  <InputText v-model="field._validations.pattern" size="small" style="width: 100%;" />
                </div>
                <div>
                  <label style="display: block; margin-bottom: 0.25rem; font-size: 0.8rem; color: var(--p-text-muted-color);">Mín. caracteres</label>
                  <InputNumber v-model="field._validations.minLength" size="small" style="width: 100%;" />
                </div>
                <div>
                  <label style="display: block; margin-bottom: 0.25rem; font-size: 0.8rem; color: var(--p-text-muted-color);">Máx. caracteres</label>
                  <InputNumber v-model="field._validations.maxLength" size="small" style="width: 100%;" />
                </div>
              </div>
            </Panel>

            <!-- Field flags & delete -->
            <div style="display: flex; gap: 1.5rem; align-items: center; margin-top: 0.75rem; flex-wrap: wrap;">
              <div style="display: flex; align-items: center; gap: 0.4rem;">
                <Checkbox v-model="field.is_required" :binary="true" :input-id="`req-${si}-${fi}`" />
                <label :for="`req-${si}-${fi}`" style="font-size: 0.85rem;">Requerido</label>
              </div>
              <div style="display: flex; align-items: center; gap: 0.4rem;">
                <Checkbox v-model="field.is_identifier" :binary="true" :input-id="`id-${si}-${fi}`" />
                <label :for="`id-${si}-${fi}`" style="font-size: 0.85rem;">Identificador</label>
              </div>
              <div style="display: flex; align-items: center; gap: 0.4rem;">
                <Checkbox v-model="field.is_cookie" :binary="true" :input-id="`ck-${si}-${fi}`" />
                <label :for="`ck-${si}-${fi}`" style="font-size: 0.85rem;">Cookie</label>
              </div>
              <Button type="button" icon="pi pi-times" size="small" severity="danger" text style="margin-left: auto;" @click="removeField(si, fi)" />
            </div>
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
const props = defineProps<{ initialData?: any }>()
const emit = defineEmits<{ save: [data: any] }>()

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
  sort_order: number
  _optionsStr: string
  _validations: { pattern?: string; minLength?: number; maxLength?: number }
}

interface StepData {
  id?: string
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
    sort_order: 0,
    _optionsStr: '',
    _validations: {}
  }
}

function createStep(num: number): StepData {
  return {
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
  identifier_fields: [] as string[],
  cookie_fields: [] as string[],
  session_required_fields: [] as string[],
  is_active: true,
  steps: [createStep(1)] as StepData[]
})

const identifierChips = ref<string[]>([])
const cookieChips = ref<string[]>([])
const sessionRequiredChips = ref<string[]>([])

if (props.initialData) {
  Object.assign(form, {
    ...props.initialData,
    steps: props.initialData.steps?.map((s: any, i: number) => ({
      ...s,
      step_number: s.step_number || i + 1,
      is_final: !!s.is_final,
      fields: s.fields?.map((f: any) => ({
        ...f,
        is_required: !!f.is_required,
        is_identifier: !!f.is_identifier,
        is_cookie: !!f.is_cookie,
        options: typeof f.options === 'string' ? JSON.parse(f.options) : (f.options || []),
        _optionsStr: (typeof f.options === 'string' ? JSON.parse(f.options) : (f.options || [])).join('\n'),
        _validations: typeof f.validations === 'string' ? JSON.parse(f.validations) : (f.validations || {})
      })) || [createField()]
    })) || [createStep(1)]
  })
  identifierChips.value = props.initialData.identifier_fields || []
  cookieChips.value = props.initialData.cookie_fields || []
  sessionRequiredChips.value = props.initialData.session_required_fields || []
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

function moveStep(index: number, direction: number) {
  const newIndex = index + direction
  if (newIndex < 0 || newIndex >= form.steps.length) return
  const temp = form.steps[index]
  form.steps[index] = form.steps[newIndex]
  form.steps[newIndex] = temp
  form.steps.forEach((s, i) => { s.step_number = i + 1 })
}

function addField(stepIndex: number) {
  form.steps[stepIndex].fields.push(createField())
}

function removeField(stepIndex: number, fieldIndex: number) {
  form.steps[stepIndex].fields.splice(fieldIndex, 1)
}

function save() {
  const data = {
    ...toRaw(form),
    identifier_fields: identifierChips.value,
    cookie_fields: cookieChips.value,
    session_required_fields: sessionRequiredChips.value,
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
        sort_order: fi
      }))
    }))
  }

  emit('save', data)
}
</script>
