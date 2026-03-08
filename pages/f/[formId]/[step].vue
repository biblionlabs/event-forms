<template>
  <div>
    <ProgressSpinner v-if="pending" style="display: block; margin: 3rem auto;" />

    <template v-else-if="formError">
      <Card style="text-align: center;">
        <template #title>Formulario no disponible</template>
        <template #content>
          <p style="color: var(--p-text-muted-color);">{{ formError.statusMessage || 'Este formulario no existe o no está activo.' }}</p>
        </template>
      </Card>
    </template>

    <!-- Thank You Screen -->
    <template v-else-if="submitted && thankYouData">
      <Card style="text-align: center; padding: 1.5rem;">
        <template #content>
          <img
            v-if="thankYouData.image_url"
            :src="thankYouData.image_url"
            :alt="thankYouData.title"
            style="max-width: 280px; margin: 0 auto 1.5rem; display: block; border-radius: var(--p-border-radius);"
          />
          <i v-else class="pi pi-check-circle" style="font-size: 3rem; color: var(--p-green-500); display: block; margin-bottom: 1rem;" />
          <h2>{{ thankYouData.title }}</h2>
          <p style="color: var(--p-text-muted-color);">{{ thankYouData.message }}</p>
          <Button
            v-if="nextStep"
            label="Continuar al siguiente paso"
            icon="pi pi-arrow-right"
            @click="navigateTo(`/f/${formId}/${nextStep}`)"
            style="margin-top: 1rem;"
          />
        </template>
      </Card>
    </template>

    <!-- Form Step -->
    <template v-else-if="data">
      <Card>
        <template #title>{{ data.form.name }}</template>
        <template #subtitle>{{ data.form.description }}</template>
        <template #content>
          <FormStepProgress
            :current-step="data.step.step_number"
            :total-steps="data.totalSteps"
          />

          <h3 style="margin-bottom: 0.25rem;">{{ data.step.title }}</h3>
          <p v-if="data.step.description" style="color: var(--p-text-muted-color); margin-bottom: 1.5rem;">{{ data.step.description }}</p>

          <Message v-if="data.isReturningUser && Object.keys(data.prefillData).length > 0" severity="info" :closable="false" style="margin-bottom: 1.5rem;">
            Hemos encontrado tus datos anteriores. Los campos ya conocidos están pre-llenados.
          </Message>

          <form @submit.prevent="submitStep">
            <template v-for="field in visibleFields" :key="field.field_key">
              <FormDynamicField
                :field="field"
                v-model="formData[field.field_key]"
                :error="errors[field.field_key]"
              />
            </template>

            <Message v-if="submitError" severity="error" :closable="false" style="margin-bottom: 1rem;">{{ submitError }}</Message>

            <Button
              type="submit"
              :label="data.step.is_final ? 'Enviar' : 'Siguiente'"
              :icon="data.step.is_final ? 'pi pi-check' : 'pi pi-arrow-right'"
              :loading="submitting"
              style="width: 100%; margin-top: 0.5rem;"
            />
          </form>
        </template>
      </Card>
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
        errors[field.field_key] = 'Formato inválido'
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
