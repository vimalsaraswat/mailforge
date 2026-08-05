import { createApiClient } from '~/services/api'
import { createAuthService } from '~/services/auth'
import type { CurrentUser } from '~/types/auth'

export function useAuth() {
  const config = useRuntimeConfig()
  const api = createApiClient(config.public.apiBase)
  const auth = createAuthService(api, config.public.apiBase)

  const user = ref<CurrentUser | null>(null)
  const loading = ref(true)

  async function loadUser() {
    try {
      user.value = await auth.getCurrentUser()
    } catch {
      user.value = null
    } finally {
      loading.value = false
    }
  }

  function signInWithGoogle() {
    window.location.assign(auth.getGoogleLoginUrl())
  }

  onMounted(loadUser)

  return {
    user: readonly(user),
    loading: readonly(loading),
    loadUser,
    signInWithGoogle,
  }
}
