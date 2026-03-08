export default defineEventHandler(async (event) => {
  const db = getDB(event)
  const id = getRouterParam(event, 'id')

  const [user, responses, sessions, scans] = await Promise.all([
    db.prepare('SELECT * FROM user_profiles WHERE id = ?').bind(id).first(),
    db.prepare(
      `SELECT fr.*, f.name as form_name,
        (SELECT GROUP_CONCAT(sr.data, '|||') FROM step_responses sr WHERE sr.form_response_id = fr.id ORDER BY sr.step_number) as step_data
       FROM form_responses fr
       LEFT JOIN forms f ON f.id = fr.form_id
       WHERE fr.user_profile_id = ?
       ORDER BY fr.started_at DESC`
    ).bind(id).all(),
    db.prepare(
      'SELECT * FROM sessions WHERE user_profile_id = ? ORDER BY last_active_at DESC'
    ).bind(id).all(),
    db.prepare(
      `SELECT se.*, f.name as form_name FROM scan_events se
       LEFT JOIN forms f ON f.id = se.form_id
       WHERE se.user_profile_id = ?
       ORDER BY se.scanned_at DESC LIMIT 100`
    ).bind(id).all()
  ])

  if (!user) throw createError({ statusCode: 404, statusMessage: 'User not found' })

  return {
    user: { ...user, data: JSON.parse((user.data as string) || '{}') },
    responses: responses.results.map((r: any) => ({
      ...r,
      step_data: r.step_data ? r.step_data.split('|||').map((d: string) => {
        try { return JSON.parse(d) } catch { return {} }
      }) : []
    })),
    sessions: sessions.results,
    scans: scans.results
  }
})
