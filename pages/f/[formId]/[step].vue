<template>
  <div>
    <div v-if="pending" aria-busy="true" style="text-align: center; padding: 3rem;">
      Cargando formulario...
    </div>

    <template v-else-if="formError">
      <article style="text-align: center;">
        <h2>Formulario no disponible</h2>
        <p>{{ formError.statusMessage || 'Este formulario no existe o no está activo.' }}</p>
      </article>
    </template>

    <template v-else-if="submitted && thankYouData">
      <article style="text-align: center; padding: 2rem;">
        <img
          v-if="thankYouData.image_url"
          :src="thankYouData.image_url"
          :alt="thankYouData.title"
          style="max-width: 300px; margin: 0 auto 1.5rem; display: block; border-radius: var(--pico-border-radius);"
        />
        <h2>{{ thankYouData.title }}</h2>
        <p>{{ thankYouData.message }}</p>
        <NuxtLink
          v-if="nextStep"
          :to="`/f/${formId}/${nextStep}`"
          role="button"
          style="margin-top: 1rem;"
        >
          Continuar al siguiente paso
        </NuxtLink>
      </article>
    </template>

    <template v-else-if="data">
      <article>
        <header>
          <h2>{{ data.form.name }}</h2>
          <p v-if="data.form.description">{{ data.form.description }}</p>
        </header>

        <FormStepProgress
          :current-step="data.step.step_number"
          :total-steps="data.totalSteps"
        />

        <h3>{{ data.step.title }}</h3>
        <p v-if="data.step.description">{{ data.step.description }}</p>

        <div v-if="data.isReturningUser && Object.keys(data.prefillData).length > 0" style="background: var(--pico-card-background-color); padding: 1rem; border-radius: var(--pico-border-radius); margin-bottom: 1rem; border-left: 3px solid var(--pico-primary);">
          <small>Hemos encontrado tus datos anteriores. Los campos ya conocidos están pre-llenados.</small>
        </div>

        <form @submit.prevent="submitStep">
          <template v-for="field in visibleFields" :key="field.field_key">
            <FormDynamicField
              :field="field"
              v-model="formData[field.field_key]"
              :error="errors[field.field_key]"
            />
          </template>

          <p v-if="submitError" style="color: var(--pico-del-color);">{{ submitError }}</p>

          <button type="submit" :disabled="submitting" :aria-busy="submitting">
            {{ data.step.is_final ? 'Enviar' : 'Siguiente' }}
          </button>
        </form>
      </article>
    </template>
  </div>
</template>

<script setup lang="ts">
definePageMeta({ layout: 'form' })

const route = useRoute()
const formId = route.params.formId as string
const stepNum = route.params.step as string

const { data, pending, error: formError } = await useFetch(`/api/f/${formId}/${stepNum}`)

const formData = reactive<Record<string, any>>({})
const errors = reactive<Record<string, string>>({})
const submitting = ref(false)
const submitted = ref(false)
const submitError = ref('')
const thankYouData = ref<{ title: string; message: string; image_url: string } | null>(null)
const nextStep = ref<number | null>(null)

if (data.value) {
  for (const field of data.value.fields) {
    formData[field.field_key] = data.value.prefillData?.[field.field_key] || ''
  }
}

const visibleFields = computed(() => {
  if (!data.value) return []
  return data.value.fields.filter((f: any) => {
    if (data.value!.fieldsToSkip.includes(f.field_key) && formData[f.field_key]) {
      return false
    }
    return true
  })
})

function validate(): boolean {
  let valid = true
  Object.keys(errors).forEach(k => delete errors[k])

  for (const field of data.value?.fields || []) {
    const value = formData[field.field_key]
    if (field.is_required && !value) {
      errors[field.field_key] = `${field.label} es requerido`
      valid = false
      continue
    }
    if (value && field.validations?.pattern) {
      const re = new RegExp(field.validations.pattern)
      if (!re.test(value)) {
        errors[field.field_key] = `Formato inválido`
        valid = false
      }
    }
    if (value && field.validations?.minLength && value.length < field.validations.minLength) {
      errors[field.field_key] = `Mínimo ${field.validations.minLength} caracteres`
      valid = false
    }
    if (value && field.validations?.maxLength && value.length > field.validations.maxLength) {
      errors[field.field_key] = `Máximo ${field.validations.maxLength} caracteres`
      valid = false
    }
  }
  return valid
}

async function submitStep() {
  if (!validate()) return

  submitting.value = true
  submitError.value = ''

  try {
    const cleanData: Record<string, any> = {}
    for (const field of data.value?.fields || []) {
      if (formData[field.field_key] !== undefined && formData[field.field_key] !== '') {
        cleanData[field.field_key] = formData[field.field_key]
      }
    }

    const result = await $fetch<{
      success: boolean
      isCompleted: boolean
      nextStep: number | null
      thankYou: { title: string; message: string; image_url: string } | null
    }>(`/api/f/${formId}/${stepNum}`, {
      method: 'POST',
      body: { data: cleanData }
    })

    submitted.value = true
    nextStep.value = result.nextStep

    if (result.thankYou) {
      thankYouData.value = result.thankYou
    } else if (result.nextStep) {
      navigateTo(`/f/${formId}/${result.nextStep}`)
    } else {
      thankYouData.value = {
        title: data.value?.form.thank_you_title || 'Gracias',
        message: data.value?.form.thank_you_message || 'Tu respuesta ha sido registrada.',
        image_url: data.value?.form.thank_you_image_url || ''
      }
    }
  } catch (err: any) {
    submitError.value = err?.data?.statusMessage || err?.message || 'Error al enviar el formulario'
  } finally {
    submitting.value = false
  }
}
</script>
