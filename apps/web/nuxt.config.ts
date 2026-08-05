import tailwindcss from '@tailwindcss/vite'

// https://nuxt.com/docs/api/configuration/nuxt-config
export default defineNuxtConfig({
  modules: ['@nuxt/ui', '@nuxt/icon', '@nuxt/eslint'],
  devtools: { enabled: true },
  css: ['~/assets/css/main.css'],
  compatibilityDate: '2025-07-15',
  devServer: {
    port: 8000,
  },
  runtimeConfig: {
    public: {
      apiBase: 'http://localhost:3000',
    },
  },
  vite: {
    plugins: [tailwindcss()],
  },
})
