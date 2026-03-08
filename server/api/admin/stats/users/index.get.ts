export default defineEventHandler(async (event) => {
  const db = getDB(event)
  const query = getQuery(event)
  const page = parseInt(query.page as string) || 1
  const limit = parseInt(query.limit as string) || 50
  const offset = (page - 1) * limit

  const [users, total] = await Promise.all([
    db.prepare(
      `SELECT up.*,
        COUNT(DISTINCT fr.id) as total_responses,
        COUNT(DISTINCT fr.form_id) as forms_answered,
        COUNT(DISTINCT s.id) as total_sessions,
        MAX(s.last_active_at) as last_seen,
        (SELECT country FROM sessions WHERE user_profile_id = up.id ORDER BY last_active_at DESC LIMIT 1) as last_country,
        (SELECT device_type FROM sessions WHERE user_profile_id = up.id ORDER BY last_active_at DESC LIMIT 1) as last_device
       FROM user_profiles up
       LEFT JOIN form_responses fr ON fr.user_profile_id = up.id
       LEFT JOIN sessions s ON s.user_profile_id = up.id
       GROUP BY up.id
       ORDER BY up.updated_at DESC
       LIMIT ? OFFSET ?`
    ).bind(limit, offset).all(),
    db.prepare('SELECT COUNT(*) as count FROM user_profiles').first()
  ])

  return {
    users: users.results.map((u: any) => ({
      ...u,
      data: JSON.parse(u.data || '{}')
    })),
    total: (total as any)?.count || 0,
    page,
    limit
  }
})
