export default defineEventHandler(async (event) => {
  const db = getDB(event)
  const id = getRouterParam(event, 'id')

  const [form, steps, scansByStep, responsesByStep, scanTimeline, recentResponses] = await Promise.all([
    db.prepare('SELECT * FROM forms WHERE id = ?').bind(id).first(),
    db.prepare('SELECT * FROM form_steps WHERE form_id = ? ORDER BY step_number').bind(id).all(),
    db.prepare(
      `SELECT step_number, COUNT(*) as total,
        COUNT(DISTINCT session_id) as unique_visitors,
        SUM(CASE WHEN is_new_user = 1 THEN 1 ELSE 0 END) as new_users
       FROM scan_events WHERE form_id = ?
       GROUP BY step_number ORDER BY step_number`
    ).bind(id).all(),
    db.prepare(
      `SELECT sr.step_number, COUNT(*) as count
       FROM step_responses sr
       JOIN form_responses fr ON fr.id = sr.form_response_id
       WHERE fr.form_id = ?
       GROUP BY sr.step_number ORDER BY sr.step_number`
    ).bind(id).all(),
    db.prepare(
      `SELECT DATE(scanned_at) as date, COUNT(*) as count
       FROM scan_events WHERE form_id = ?
       GROUP BY DATE(scanned_at) ORDER BY date DESC LIMIT 30`
    ).bind(id).all(),
    db.prepare(
      `SELECT fr.*, up.data as user_data
       FROM form_responses fr
       LEFT JOIN user_profiles up ON up.id = fr.user_profile_id
       WHERE fr.form_id = ?
       ORDER BY fr.started_at DESC LIMIT 50`
    ).bind(id).all()
  ])

  if (!form) throw createError({ statusCode: 404, statusMessage: 'Form not found' })

  const funnel = steps.results.map((step: any) => {
    const scanData = scansByStep.results.find((s: any) => s.step_number === step.step_number) as any
    const responseData = responsesByStep.results.find((r: any) => r.step_number === step.step_number) as any
    return {
      step_number: step.step_number,
      title: step.title,
      scans: scanData?.total || 0,
      unique_visitors: scanData?.unique_visitors || 0,
      new_users: scanData?.new_users || 0,
      completions: responseData?.count || 0
    }
  })

  return {
    form,
    funnel,
    scanTimeline: scanTimeline.results,
    recentResponses: recentResponses.results
  }
})
