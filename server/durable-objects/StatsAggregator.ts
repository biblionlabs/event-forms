export class StatsAggregator {
  private state: DurableObjectState
  private cache: Map<string, { data: any; expiry: number }>

  constructor(state: DurableObjectState) {
    this.state = state
    this.cache = new Map()
  }

  async fetch(request: Request): Promise<Response> {
    const url = new URL(request.url)
    const path = url.pathname

    if (request.method === 'POST' && path === '/increment') {
      const body = await request.json() as { key: string; amount?: number }
      const current = (await this.state.storage.get<number>(body.key)) || 0
      const newVal = current + (body.amount || 1)
      await this.state.storage.put(body.key, newVal)
      return Response.json({ value: newVal })
    }

    if (request.method === 'POST' && path === '/cache') {
      const body = await request.json() as { key: string; data: any; ttl?: number }
      const expiry = Date.now() + (body.ttl || 60) * 1000
      this.cache.set(body.key, { data: body.data, expiry })
      return Response.json({ success: true })
    }

    if (request.method === 'GET' && path === '/cache') {
      const key = url.searchParams.get('key')
      if (key && this.cache.has(key)) {
        const entry = this.cache.get(key)!
        if (entry.expiry > Date.now()) {
          return Response.json({ data: entry.data, hit: true })
        }
        this.cache.delete(key)
      }
      return Response.json({ data: null, hit: false })
    }

    if (request.method === 'GET' && path === '/counters') {
      const allEntries = await this.state.storage.list()
      const counters: Record<string, any> = {}
      for (const [key, value] of allEntries) {
        counters[key] = value
      }
      return Response.json(counters)
    }

    if (request.method === 'POST' && path === '/track-scan') {
      const body = await request.json() as { formId: string; stepNumber: number }
      const dayKey = new Date().toISOString().split('T')[0]
      const scanKey = `scans:${body.formId}:${dayKey}`
      const totalKey = `scans:total:${dayKey}`
      const stepKey = `scans:${body.formId}:step:${body.stepNumber}:${dayKey}`

      const [scanCount, totalCount, stepCount] = await Promise.all([
        this.state.storage.get<number>(scanKey) || 0,
        this.state.storage.get<number>(totalKey) || 0,
        this.state.storage.get<number>(stepKey) || 0
      ])

      await Promise.all([
        this.state.storage.put(scanKey, (scanCount as number) + 1),
        this.state.storage.put(totalKey, (totalCount as number) + 1),
        this.state.storage.put(stepKey, (stepCount as number) + 1)
      ])

      return Response.json({
        formScans: (scanCount as number) + 1,
        totalScans: (totalCount as number) + 1,
        stepScans: (stepCount as number) + 1
      })
    }

    if (request.method === 'GET' && path === '/realtime') {
      const dayKey = new Date().toISOString().split('T')[0]
      const allEntries = await this.state.storage.list({ prefix: `scans:` })
      const todayStats: Record<string, any> = {}
      for (const [key, value] of allEntries) {
        if (key.includes(dayKey)) {
          todayStats[key] = value
        }
      }
      return Response.json(todayStats)
    }

    return new Response('Not found', { status: 404 })
  }
}
