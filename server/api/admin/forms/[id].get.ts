export default defineEventHandler(async (event) => {
  const db = getDB(event)
  const id = getRouterParam(event, 'id')

  const form = await db.prepare('SELECT * FROM forms WHERE id = ?').bind(id).first()
  if (!form) throw createError({ statusCode: 404, statusMessage: 'Form not found' })

  const steps = await db.prepare(
    'SELECT * FROM form_steps WHERE form_id = ? ORDER BY step_number'
  ).bind(id).all()

  const fields = await db.prepare(
    'SELECT * FROM form_fields WHERE form_id = ? ORDER BY sort_order'
  ).bind(id).all()

  const stepsWithFields = steps.results.map((step: any) => ({
    ...step,
    fields: fields.results.filter((f: any) => f.step_id === step.id)
  }))

  return {
    ...form,
    identifier_fields: JSON.parse((form.identifier_fields as string) || '[]'),
    cookie_fields: JSON.parse((form.cookie_fields as string) || '[]'),
    session_required_fields: JSON.parse((form.session_required_fields as string) || '[]'),
    steps: stepsWithFields
  }
})
