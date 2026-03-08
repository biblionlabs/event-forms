export default defineEventHandler(async (event) => {
  const db = getDB(event)
  const formId = getRouterParam(event, 'formId')
  const stepNum = parseInt(getRouterParam(event, 'step') || '1')

  const form = await db.prepare('SELECT * FROM forms WHERE id = ? AND is_active = 1').bind(formId).first()
  if (!form) throw createError({ statusCode: 404, statusMessage: 'Formulario no encontrado' })

  const step = await db.prepare(
    'SELECT * FROM form_steps WHERE form_id = ? AND step_number = ?'
  ).bind(formId, stepNum).first()
  if (!step) throw createError({ statusCode: 404, statusMessage: 'Paso no encontrado' })

  const fields = await db.prepare(
    'SELECT * FROM form_fields WHERE step_id = ? ORDER BY sort_order'
  ).bind(step.id).all()

  const totalSteps = await db.prepare(
    'SELECT COUNT(*) as count FROM form_steps WHERE form_id = ?'
  ).bind(formId).first()

  const session = await getOrCreateSession(event)
  const geo = extractGeoData(event)

  await db.prepare(
    `INSERT INTO scan_events (id, form_id, step_number, session_id, user_profile_id, is_new_user, ip_address, user_agent, country, city, region, timezone, device_type, referrer)
     VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)`
  ).bind(
    generateId(), formId, stepNum, session.sessionId, session.profileId,
    session.isNew ? 1 : 0, geo.ip, geo.userAgent, geo.country, geo.city,
    geo.region, geo.timezone, geo.deviceType,
    getHeader(event, 'referer') || ''
  ).run()

  let prefillData: Record<string, any> = {}
  const fieldsToSkip: string[] = []

  if (session.profileId) {
    const profile = await db.prepare('SELECT data FROM user_profiles WHERE id = ?').bind(session.profileId).first()
    if (profile && profile.data) {
      const userData = JSON.parse(profile.data as string)
      for (const field of fields.results as any[]) {
        if (userData[field.field_key] !== undefined) {
          prefillData[field.field_key] = userData[field.field_key]
          if (!field.is_required || userData[field.field_key]) {
            fieldsToSkip.push(field.field_key)
          }
        }
      }
    }
  } else {
    const cookieFields = JSON.parse((form.cookie_fields as string) || '[]') as string[]
    for (const key of cookieFields) {
      const val = getCookie(event, `ef_${key}`)
      if (val) prefillData[key] = val
    }
  }

  let existingResponse = null
  if (session.sessionId) {
    existingResponse = await db.prepare(
      'SELECT id, current_step, status FROM form_responses WHERE form_id = ? AND session_id = ? ORDER BY started_at DESC LIMIT 1'
    ).bind(formId, session.sessionId).first()
  }

  const parsedFields = (fields.results as any[]).map(f => ({
    ...f,
    options: JSON.parse(f.options || '[]'),
    validations: JSON.parse(f.validations || '{}')
  }))

  return {
    form: {
      id: form.id,
      name: form.name,
      description: form.description,
      thank_you_title: form.thank_you_title,
      thank_you_message: form.thank_you_message,
      thank_you_image_url: form.thank_you_image_url
    },
    step: {
      id: step.id,
      step_number: step.step_number,
      title: step.title,
      description: step.description,
      thank_you_title: step.thank_you_title,
      thank_you_message: step.thank_you_message,
      thank_you_image_url: step.thank_you_image_url,
      is_final: step.is_final
    },
    fields: parsedFields,
    totalSteps: (totalSteps as any)?.count || 1,
    prefillData,
    fieldsToSkip,
    sessionId: session.sessionId,
    existingResponse,
    isReturningUser: !session.isNew
  }
})
