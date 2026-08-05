export interface ApiClient {
  get<T>(path: string): Promise<T>
}

export function createApiClient(baseURL: string): ApiClient {
  return {
    get<T>(path: string) {
      return $fetch<T>(path, {
        baseURL,
        credentials: 'include',
      })
    },
  }
}
