<template>
  <form @submit.prevent="save">
    <article>
      <header><h3>Información General</h3></header>
      <label>
        Nombre del formulario *
        <input v-model="form.name" type="text" required />
      </label>
      <label>
        Descripción
        <textarea v-model="form.description" rows="2"></textarea>
      </label>
      <div class="grid">
        <label>
          Título de agradecimiento final
          <input v-model="form.thank_you_title" type="text" />
        </label>
        <label>
          URL de imagen de agradecimiento
          <input v-model="form.thank_you_image_url" type="url" placeholder="https://..." />
        </label>
      </div>
      <label>
        Mensaje de agradecimiento final
        <textarea v-model="form.thank_you_message" rows="2"></textarea>
      </label>
    </article>

    <article>
      <header><h3>Configuración de Sesión e Identificación</h3></header>
      <p><small>Define qué campos se usan para identificar usuarios entre formularios.</small></p>
      <label>
        Campos identificadores (fingerprint)
        <input v-model="identifierFieldsStr" type="text" placeholder="email,phone (separados por coma)" />
      </label>
      <p><small>Combinación de campos que generan un hash único para identificar al usuario.</small></p>
      <label>
        Campos a guardar en cookies
        <input v-model="cookieFieldsStr" type="text" placeholder="email,name (separados por coma)" />
      </label>
      <p><small>Campos que se almacenan en cookies para pre-llenar en futuras visitas.</small></p>
      <label>
        Campos requeridos para sesión completa
        <input v-model="sessionRequiredStr" type="text" placeholder="email (separados por coma)" />
      </label>
      <p><small>Campos que deben estar completos para considerar la sesión válida.</small></p>
    </article>

    <h3>Pasos del Formulario</h3>
    <div v-for="(step, si) in form.steps" :key="si">
      <article>
        <header>
          <div style="display: flex; justify-content: space-between; align-items: center;">
            <h4>Paso {{ si + 1 }}</h4>
            <div>
              <button type="button" class="outline" @click="moveStep(si, -1)" :disabled="si === 0">&#9650;</button>
              <button type="button" class="outline" @click="moveStep(si, 1)" :disabled="si === form.steps.length - 1">&#9660;</button>
              <button type="button" class="outline secondary" @click="removeStep(si)">Eliminar</button>
            </div>
          </div>
        </header>
        <div class="grid">
          <label>
            Título del paso *
            <input v-model="step.title" type="text" required />
          </label>
          <label>
            Descripción
            <input v-model="step.description" type="text" />
          </label>
        </div>
        <details>
          <summary>Mensaje al completar este paso</summary>
          <div class="grid">
            <label>
              Título
              <input v-model="step.thank_you_title" type="text" />
            </label>
            <label>
              URL de imagen
              <input v-model="step.thank_you_image_url" type="url" placeholder="https://..." />
            </label>
          </div>
          <label>
            Mensaje
            <textarea v-model="step.thank_you_message" rows="2"></textarea>
          </label>
        </details>
        <label>
          <input type="checkbox" v-model="step.is_final" />
          Es paso final (completa el formulario)
        </label>

        <h5>Campos</h5>
        <div v-for="(field, fi) in step.fields" :key="fi" style="border: 1px solid var(--pico-muted-border-color); padding: 1rem; margin-bottom: 0.5rem; border-radius: var(--pico-border-radius);">
          <div class="grid">
            <label>
              Clave *
              <input v-model="field.field_key" type="text" required placeholder="email, nombre, etc." />
            </label>
            <label>
              Etiqueta *
              <input v-model="field.label" type="text" required placeholder="Tu correo electrónico" />
            </label>
            <label>
              Tipo
              <select v-model="field.field_type">
                <option value="text">Texto</option>
                <option value="email">Email</option>
                <option value="phone">Teléfono</option>
                <option value="number">Número</option>
                <option value="textarea">Área de texto</option>
                <option value="select">Selección</option>
                <option value="radio">Radio</option>
                <option value="checkbox">Checkbox</option>
                <option value="date">Fecha</option>
                <option value="url">URL</option>
              </select>
            </label>
          </div>
          <div class="grid">
            <label>
              Placeholder
              <input v-model="field.placeholder" type="text" />
            </label>
            <label v-if="['select', 'radio', 'checkbox'].includes(field.field_type)">
              Opciones (una por línea)
              <textarea v-model="field._optionsStr" rows="3" @input="parseOptions(field)"></textarea>
            </label>
          </div>
          <details>
            <summary>Validaciones</summary>
            <div class="grid">
              <label>
                Patrón (regex)
                <input v-model="field._validations.pattern" type="text" />
              </label>
              <label>
                Mín. caracteres
                <input v-model.number="field._validations.minLength" type="number" />
              </label>
              <label>
                Máx. caracteres
                <input v-model.number="field._validations.maxLength" type="number" />
              </label>
            </div>
          </details>
          <div style="display: flex; gap: 1rem; flex-wrap: wrap; align-items: center;">
            <label style="margin: 0;">
              <input type="checkbox" v-model="field.is_required" /> Requerido
            </label>
            <label style="margin: 0;">
              <input type="checkbox" v-model="field.is_identifier" /> Identificador
            </label>
            <label style="margin: 0;">
              <input type="checkbox" v-model="field.is_cookie" /> Guardar en cookie
            </label>
            <button type="button" class="outline secondary" style="margin-left: auto;" @click="removeField(si, fi)">
              Quitar campo
            </button>
          </div>
        </div>
        <button type="button" class="outline" @click="addField(si)">+ Agregar Campo</button>
      </article>
    </div>

    <button type="button" class="secondary" @click="addStep">+ Agregar Paso</button>
    <hr />
    <button type="submit">Guardar Formulario</button>
  </form>
</template>

<script setup lang="ts">
const props = defineProps<{ initialData?: any }>()
const emit = defineEmits<{ save: [data: any] }>()

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

const identifierFieldsStr = ref('')
const cookieFieldsStr = ref('')
const sessionRequiredStr = ref('')

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
  identifierFieldsStr.value = (props.initialData.identifier_fields || []).join(', ')
  cookieFieldsStr.value = (props.initialData.cookie_fields || []).join(', ')
  sessionRequiredStr.value = (props.initialData.session_required_fields || []).join(', ')
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
  const parseList = (s: string) => s.split(',').map(v => v.trim()).filter(Boolean)

  const data = {
    ...toRaw(form),
    identifier_fields: parseList(identifierFieldsStr.value),
    cookie_fields: parseList(cookieFieldsStr.value),
    session_required_fields: parseList(sessionRequiredStr.value),
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
