export default defineEventHandler(async (event) => {
  const db = getDB(event)

  const [totalScans, uniqueUsers, totalResponses, completedResponses, activeForms, recentScans, formStats, deviceStats, countryStats, funnelData] = await Promise.all([
    db.prepare('SELECT COUNT(*) as count FROM scan_events').first(),
    db.prepare('SELECT COUNT(*) as count FROM user_profiles').first(),
    db.prepare('SELECT COUNT(*) as count FROM form_responses').first(),
    db.prepare('SELECT COUNT(*) as count FROM form_responses WHERE status = ?').bind('completed').first(),
    db.prepare('SELECT COUNT(*) as count FROM forms WHERE is_active = 1').first(),
    db.prepare(
      `SELECT se.*, f.name as form_name FROM scan_events se
       LEFT JOIN forms f ON f.id = se.form_id
       ORDER BY se.scanned_at DESC LIMIT 50`
    ).all(),
    db.prepare(
      `SELECT f.id, f.name,
        COUNT(DISTINCT se.id) as scan_count,
        COUNT(DISTINCT fr.id) as response_count,
        COUNT(DISTINCT CASE WHEN fr.status = 'completed' THEN fr.id END) as completed_count,
        COUNT(DISTINCT se.session_id) as unique_visitors
       FROM forms f
       LEFT JOIN scan_events se ON se.form_id = f.id
       LEFT JOIN form_responses fr ON fr.form_id = f.id
       GROUP BY f.id ORDER BY scan_count DESC`
    ).all(),
    db.prepare(
      `SELECT device_type, COUNT(*) as count FROM scan_events
       WHERE device_type IS NOT NULL AND device_type != ''
       GROUP BY device_type ORDER BY count DESC`
    ).all(),
    db.prepare(
      `SELECT country, COUNT(*) as count FROM scan_events
       WHERE country IS NOT NULL AND country != ''
       GROUP BY country ORDER BY count DESC LIMIT 20`
    ).all(),
    db.prepare(
      `SELECT form_id, step_number, COUNT(DISTINCT session_id) as visitors
       FROM scan_events GROUP BY form_id, step_number
       ORDER BY form_id, step_number`
    ).all()
  ])

  return {
    overview: {
      totalScans: (totalScans as any)?.count || 0,
      uniqueUsers: (uniqueUsers as any)?.count || 0,
      totalResponses: (totalResponses as any)?.count || 0,
      completedResponses: (completedResponses as any)?.count || 0,
      activeForms: (activeForms as any)?.count || 0,
      completionRate: (totalResponses as any)?.count > 0
        ? Math.round(((completedResponses as any)?.count / (totalResponses as any)?.count) * 100)
        : 0
    },
    recentScans: recentScans.results,
    formStats: formStats.results,
    deviceStats: deviceStats.results,
    countryStats: countryStats.results,
    funnelData: funnelData.results
  }
})
