import Aura from '@primevue/themes/aura'

export default defineNuxtConfig({
  compatibilityDate: '2025-01-01',
  devtools: { enabled: false },

  modules: ['@primevue/nuxt-module'],

  primevue: {
    options: {
      theme: {
        preset: Aura,
        options: {
          darkModeSelector: '.dark-mode'
        }
      },
      ripple: true
    }
  },

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
      ]
    }
  },

  css: ['~/assets/css/global.css'],

  routeRules: {
    '/admin/**': { ssr: false },
    '/f/**': { ssr: true }
  }
})
