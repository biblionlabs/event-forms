export default defineEventHandler(async (event) => {
  const db = getDB(event)
  const formId = getRouterParam(event, 'formId')
  const stepNum = parseInt(getRouterParam(event, 'step') || '1')
  const body = await readBody(event)
  const fieldData: Record<string, any> = body.data || {}

  const form = await db.prepare('SELECT * FROM forms WHERE id = ? AND is_active = 1').bind(formId).first()
  if (!form) throw createError({ statusCode: 404, statusMessage: 'Formulario no encontrado' })

  const step = await db.prepare(
    'SELECT * FROM form_steps WHERE form_id = ? AND step_number = ?'
  ).bind(formId, stepNum).first()
  if (!step) throw createError({ statusCode: 404, statusMessage: 'Paso no encontrado' })

  const fields = await db.prepare(
    'SELECT * FROM form_fields WHERE step_id = ? ORDER BY sort_order'
  ).bind(step.id).all()

  for (const field of fields.results as any[]) {
    if (field.is_required && !fieldData[field.field_key]) {
      throw createError({ statusCode: 400, statusMessage: `El campo "${field.label}" es requerido` })
    }
    const validations = JSON.parse(field.validations || '{}')
    const value = fieldData[field.field_key]
    if (value && validations.pattern) {
      const re = new RegExp(validations.pattern)
      if (!re.test(value)) {
        throw createError({ statusCode: 400, statusMessage: `El campo "${field.label}" no tiene un formato válido` })
      }
    }
    if (value && validations.minLength && value.length < validations.minLength) {
      throw createError({ statusCode: 400, statusMessage: `El campo "${field.label}" debe tener al menos ${validations.minLength} caracteres` })
    }
    if (value && validations.maxLength && value.length > validations.maxLength) {
      throw createError({ statusCode: 400, statusMessage: `El campo "${field.label}" no puede tener más de ${validations.maxLength} caracteres` })
    }
  }

  const session = await getOrCreateSession(event)
  const secret = getAppConfig(event).cookieSecret
  const identifierFields = JSON.parse((form.identifier_fields as string) || '[]') as string[]
  const cookieFieldKeys = JSON.parse((form.cookie_fields as string) || '[]') as string[]

  let profileId = session.profileId
  let fingerprint = session.fingerprint

  const identifierValues: Record<string, string> = {}
  let hasAllIdentifiers = true
  for (const key of identifierFields) {
    if (fieldData[key]) {
      identifierValues[key] = fieldData[key]
    }
  }

  let existingProfile = null
  if (Object.keys(identifierValues).length > 0 && Object.keys(identifierValues).length === identifierFields.length) {
    fingerprint = await generateFingerprint(identifierValues, identifierFields)

    existingProfile = await db.prepare(
      'SELECT * FROM user_profiles WHERE fingerprint = ?'
    ).bind(fingerprint).first()

    if (existingProfile) {
      profileId = existingProfile.id as string
      const existingData = JSON.parse((existingProfile.data as string) || '{}')
      const mergedData = { ...existingData, ...fieldData }
      await db.prepare(
        `UPDATE user_profiles SET data = ?, updated_at = datetime('now') WHERE id = ?`
      ).bind(JSON.stringify(mergedData), profileId).run()
    } else {
      profileId = generateId()
      await db.prepare(
        'INSERT INTO user_profiles (id, fingerprint, data) VALUES (?, ?, ?)'
      ).bind(profileId, fingerprint, JSON.stringify(fieldData)).run()
    }

    await db.prepare(
      'UPDATE sessions SET user_profile_id = ?, fingerprint = ? WHERE id = ?'
    ).bind(profileId, fingerprint, session.sessionId).run()

    const encryptedFp = await encryptCookieValue(fingerprint, secret)
    setCookie(event, 'ef_fingerprint', encryptedFp, {
      httpOnly: true,
      secure: true,
      sameSite: 'lax',
      maxAge: 60 * 60 * 24 * 365,
      path: '/'
    })
  } else if (profileId) {
    const profile = await db.prepare('SELECT data FROM user_profiles WHERE id = ?').bind(profileId).first()
    if (profile) {
      const existingData = JSON.parse((profile.data as string) || '{}')
      const mergedData = { ...existingData, ...fieldData }
      await db.prepare(
        `UPDATE user_profiles SET data = ?, updated_at = datetime('now') WHERE id = ?`
      ).bind(JSON.stringify(mergedData), profileId).run()
    }
  }

  for (const key of cookieFieldKeys) {
    if (fieldData[key]) {
      setCookie(event, `ef_${key}`, fieldData[key], {
        httpOnly: false,
        secure: true,
        sameSite: 'lax',
        maxAge: 60 * 60 * 24 * 365,
        path: '/'
      })
    }
  }

  let formResponse = await db.prepare(
    'SELECT id, current_step FROM form_responses WHERE form_id = ? AND session_id = ? ORDER BY started_at DESC LIMIT 1'
  ).bind(formId, session.sessionId).first()

  if (!formResponse) {
    const frId = generateId()
    await db.prepare(
      'INSERT INTO form_responses (id, form_id, session_id, user_profile_id, current_step) VALUES (?, ?, ?, ?, ?)'
    ).bind(frId, formId, session.sessionId, profileId, stepNum).run()
    formResponse = { id: frId, current_step: stepNum }
  }

  await db.prepare(
    'INSERT INTO step_responses (id, form_response_id, step_id, step_number, data) VALUES (?, ?, ?, ?, ?)'
  ).bind(generateId(), formResponse.id, step.id, stepNum, JSON.stringify(fieldData)).run()

  const totalSteps = await db.prepare(
    'SELECT MAX(step_number) as max_step FROM form_steps WHERE form_id = ?'
  ).bind(formId).first()
  const maxStep = (totalSteps as any)?.max_step || 1
  const isCompleted = stepNum >= maxStep || step.is_final

  if (isCompleted) {
    await db.prepare(
      `UPDATE form_responses SET status = 'completed', completed_at = datetime('now'), current_step = ? WHERE id = ?`
    ).bind(stepNum, formResponse.id).run()
  } else {
    await db.prepare(
      'UPDATE form_responses SET current_step = ? WHERE id = ?'
    ).bind(stepNum + 1, formResponse.id).run()
  }

  const nextStep = isCompleted ? null : stepNum + 1

  // Always return a thankYou - use step-level if configured, else defaults
  let thankYou
  if (isCompleted) {
    thankYou = {
      title: (form.thank_you_title as string) || 'Gracias',
      message: (form.thank_you_message as string) || 'Tu respuesta ha sido registrada exitosamente.',
      image_url: (form.thank_you_image_url as string) || ''
    }
  } else if (step.thank_you_title) {
    thankYou = {
      title: step.thank_you_title as string,
      message: (step.thank_you_message as string) || '',
      image_url: (step.thank_you_image_url as string) || ''
    }
  } else {
    thankYou = {
      title: 'Paso completado',
      message: `Has completado el paso ${stepNum}. Continúa con el siguiente.`,
      image_url: ''
    }
  }

  return {
    success: true,
    isCompleted,
    nextStep,
    thankYou
  }
})
