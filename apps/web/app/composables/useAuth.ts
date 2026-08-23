import { authRoutes } from "~/services/auth";
import type { CurrentUser } from "~/types/auth";

export function useAuth() {
  const config = useRuntimeConfig();
  const {
    data: user,
    refresh: loadUser,
    pending: loading,
    clear: clearUser,
  } = useAPI<CurrentUser>(authRoutes.currentUser);
  const { execute: logoutUser, pending: loggingOut } = useAPI(authRoutes.logout, {
    immediate: false,
  });

  function signInWithGoogle(connectGmail?: boolean): void {
    const url = new URL("/auth/google", config.public.apiBase);
    if (connectGmail == true) url.searchParams.set("connect", "true");
    window.location.assign(url.toString());
  }

  async function logout(): Promise<void> {
    clearUser();
    await logoutUser();
  }

  return {
    user: readonly(user),
    loading: readonly(loading),
    loggingOut: readonly(loggingOut),
    loadUser,
    logout,
    signInWithGoogle,
  };
}
