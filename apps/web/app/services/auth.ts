import type { ApiClient } from './api'
import type { CurrentUser } from '~/types/auth'

const authRoutes = {
  currentUser: '/auth/me',
  googleLogin: '/auth/google',
} as const

export function createAuthService(api: ApiClient, apiBase: string) {
  return {
    getCurrentUser() {
      return api.get<CurrentUser>(authRoutes.currentUser)
    },

    getGoogleLoginUrl() {
      return `${apiBase}${authRoutes.googleLogin}`
    },
  }
}
