export const useAPI = createUseFetch((callerOptions) => ({
  baseURL: useRuntimeConfig().public.apiBase,
  lazy: true,
  server: false,
  credentials: "include",
  ...callerOptions,
}));

export const $api = <T = unknown>(
  url: Parameters<typeof $fetch>[0],
  callerOptions?: Parameters<typeof $fetch>[1],
): ReturnType<typeof $fetch<T>> => {
  return $fetch<T>(url, {
    baseURL: useRuntimeConfig().public.apiBase,
    credentials: "include",
    ...callerOptions,
  });
};
