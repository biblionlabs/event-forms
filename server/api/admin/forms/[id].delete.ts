export default defineEventHandler(async (event) => {
  const db = getDB(event)
  const id = getRouterParam(event, 'id')

  await db.prepare('DELETE FROM step_responses WHERE form_response_id IN (SELECT id FROM form_responses WHERE form_id = ?)').bind(id).run()
  await db.prepare('DELETE FROM form_responses WHERE form_id = ?').bind(id).run()
  await db.prepare('DELETE FROM scan_events WHERE form_id = ?').bind(id).run()
  await db.prepare('DELETE FROM form_fields WHERE form_id = ?').bind(id).run()
  await db.prepare('DELETE FROM form_steps WHERE form_id = ?').bind(id).run()
  await db.prepare('DELETE FROM forms WHERE id = ?').bind(id).run()

  return { success: true }
})
