export default defineEventHandler(async (event) => {
  const db = getDB(event)
  const forms = await db.prepare(
    `SELECT f.*,
      (SELECT COUNT(*) FROM form_steps WHERE form_id = f.id) as step_count,
      (SELECT COUNT(*) FROM form_responses WHERE form_id = f.id) as response_count
     FROM forms f ORDER BY f.created_at DESC`
  ).all()
  return forms.results
})
