import type { ApiClient } from './api'
import type { CurrentUser } from '~/types/auth'

const authRoutes = {
  currentUser: '/auth/me',
  logout: '/auth/logout',
  googleLogin: '/auth/google',
} as const

export function createAuthService(api: ApiClient, apiBase: string) {
  return {
    getCurrentUser() {
      return api.get<CurrentUser>(authRoutes.currentUser)
    },

    logout() {
      return api.post(authRoutes.logout)
    },

    getGoogleLoginUrl() {
      return `${apiBase}${authRoutes.googleLogin}`
    },
  }
}
