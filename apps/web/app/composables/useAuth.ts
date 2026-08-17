import { createApiClient } from '~/services/api'
import { createAuthService } from '~/services/auth'
import type { CurrentUser } from '~/types/auth'

export function useAuth() {
  const config = useRuntimeConfig()
  const api = createApiClient(config.public.apiBase)
  const auth = createAuthService(api, config.public.apiBase)

  const user = ref<CurrentUser | null>(null)
  const loading = ref(true)
  const loggingOut = ref(false)
  let requestVersion = 0

  async function loadUser(): Promise<CurrentUser | null> {
    const version = ++requestVersion
    loading.value = true

    try {
      const currentUser = await auth.getCurrentUser()
      if (version === requestVersion) user.value = currentUser

      return currentUser
    } catch {
      if (version === requestVersion) user.value = null

      return null
    } finally {
      if (version === requestVersion) loading.value = false
    }
  }

  function signInWithGoogle(): void {
    window.location.assign(auth.getGoogleLoginUrl())
  }

  async function logout(): Promise<void> {
    const version = ++requestVersion
    loggingOut.value = true

    try {
      await auth.logout()
      if (version === requestVersion) user.value = null
    } finally {
      if (version === requestVersion) loggingOut.value = false
    }
  }

  onMounted(() => void loadUser())

  return {
    user: readonly(user),
    loading: readonly(loading),
    loggingOut: readonly(loggingOut),
    loadUser,
    logout,
    signInWithGoogle,
  }
}
