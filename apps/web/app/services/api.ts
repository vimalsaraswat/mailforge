export interface ApiClient {
  get<T>(path: string): Promise<T>
  post(path: string): Promise<unknown>
}

export function createApiClient(baseURL: string): ApiClient {
  return {
    get<T>(path: string) {
      return $fetch<T>(path, {
        baseURL,
        credentials: 'include',
      })
    },

    post(path: string) {
      return $fetch<unknown>(path, {
        method: 'POST',
        baseURL,
        credentials: 'include',
      })
    },
  }
}
