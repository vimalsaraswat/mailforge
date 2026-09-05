/** Options shared by every request to the Mailforge API. */
function useApiDefaults() {
  return {
    baseURL: useRuntimeConfig().public.apiBase,
    credentials: "include" as const,
  };
}

/**
 * Reactive API requests. These only run in the browser because the session is
 * stored in a browser cookie.
 */
export const useAPI = createUseFetch((options) => ({
  ...useApiDefaults(),
  lazy: true,
  server: false,
  ...options,
}));

export const $api = <T = unknown>(
  url: Parameters<typeof $fetch>[0],
  callerOptions?: Parameters<typeof $fetch>[1],
): ReturnType<typeof $fetch<T>> => {
  return $fetch<T>(url, {
    ...useApiDefaults(),
    ...callerOptions,
  });
};
