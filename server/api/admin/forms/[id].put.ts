export default defineEventHandler(async (event) => {
  const db = getDB(event)
  const id = getRouterParam(event, 'id')
  const body = await readBody(event)

  await db.prepare(
    `UPDATE forms SET name = ?, description = ?, thank_you_title = ?, thank_you_message = ?, thank_you_image_url = ?,
     identifier_fields = ?, cookie_fields = ?, session_required_fields = ?, is_active = ?, updated_at = datetime('now')
     WHERE id = ?`
  ).bind(
    body.name, body.description || '',
    body.thank_you_title || 'Gracias',
    body.thank_you_message || 'Tu respuesta ha sido registrada.',
    body.thank_you_image_url || '',
    JSON.stringify(body.identifier_fields || []),
    JSON.stringify(body.cookie_fields || []),
    JSON.stringify(body.session_required_fields || []),
    body.is_active !== undefined ? (body.is_active ? 1 : 0) : 1,
    id
  ).run()

  if (body.steps && Array.isArray(body.steps)) {
    await db.prepare('DELETE FROM form_fields WHERE form_id = ?').bind(id).run()
    await db.prepare('DELETE FROM form_steps WHERE form_id = ?').bind(id).run()

    for (const step of body.steps) {
      const stepId = step.id || generateId()
      await db.prepare(
        `INSERT INTO form_steps (id, form_id, step_number, title, description, thank_you_title, thank_you_message, thank_you_image_url, is_final)
         VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?)`
      ).bind(
        stepId, id, step.step_number, step.title,
        step.description || '', step.thank_you_title || '', step.thank_you_message || '',
        step.thank_you_image_url || '', step.is_final ? 1 : 0
      ).run()

      if (step.fields && Array.isArray(step.fields)) {
        for (const field of step.fields) {
          await db.prepare(
            `INSERT INTO form_fields (id, step_id, form_id, field_key, label, field_type, placeholder, options, validations, is_required, is_identifier, is_cookie, is_session_required, sort_order)
             VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)`
          ).bind(
            field.id || generateId(), stepId, id, field.field_key, field.label,
            field.field_type || 'text', field.placeholder || '',
            JSON.stringify(field.options || []),
            JSON.stringify(field.validations || {}),
            field.is_required ? 1 : 0,
            field.is_identifier ? 1 : 0,
            field.is_cookie ? 1 : 0,
            field.is_session_required ? 1 : 0,
            field.sort_order || 0
          ).run()
        }
      }
    }
  }

  return { success: true }
})
