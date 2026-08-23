export const useAPI = createUseFetch((callerOptions) => ({
  baseURL: useRuntimeConfig().public.apiBase,
  lazy: true,
  server: false,
  credentials: "include",
  ...callerOptions,
}));
