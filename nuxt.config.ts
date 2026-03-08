export default defineNuxtConfig({
  compatibilityDate: '2025-01-01',
  devtools: { enabled: false },

  nitro: {
    preset: 'cloudflare-pages',
    cloudflare: {
      pages: {
        routes: {
          exclude: ['/api/*']
        }
      }
    }
  },

  app: {
    head: {
      title: 'Event Forms',
      meta: [
        { charset: 'utf-8' },
        { name: 'viewport', content: 'width=device-width, initial-scale=1' }
      ],
      link: [
        { rel: 'stylesheet', href: 'https://cdn.jsdelivr.net/npm/@picocss/pico@2/css/pico.min.css' }
      ]
    }
  },

  routeRules: {
    '/admin/**': { ssr: false },
    '/f/**': { ssr: true }
  }
})
