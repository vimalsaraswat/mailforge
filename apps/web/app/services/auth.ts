import type { ApiClient } from './api'
import type { CurrentUser } from '~/types/auth'

const authRoutes = {
  currentUser: '/auth/me',
  logout: '/auth/logout',
  googleLogin: '/auth/google',
} as const

export interface AuthService {
  getCurrentUser(): Promise<CurrentUser>
  logout(): Promise<void>
  getGoogleLoginUrl(connectGmail?: boolean): string
}

export function createAuthService(api: ApiClient, apiBase: string): AuthService {
  return {
    getCurrentUser() {
      return api.get<CurrentUser>(authRoutes.currentUser)
    },

    logout() {
      return api.post(authRoutes.logout)
    },

    getGoogleLoginUrl(connectGmail?: boolean) {
      const url = new URL(authRoutes.googleLogin, apiBase)
      if (connectGmail) url.searchParams.set('connect', 'true')

      return url.toString()
    },
  }
}
