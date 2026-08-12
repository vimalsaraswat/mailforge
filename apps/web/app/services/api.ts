export interface ApiClient {
  get<T>(path: string): Promise<T>
  post<T, B = unknown>(path: string, body?: B): Promise<T>
  put<T, B = unknown>(path: string, body: B): Promise<T>
  delete<T>(path: string): Promise<T>
}

// TODO: make more typesafe
export function createApiClient(baseURL: string): ApiClient {
  return {
    get<T>(path: string) {
      return $fetch<T>(path, {
        baseURL,
        credentials: 'include',
      })
    },

    post<T, B extends BodyInit | Record<string, any> | null | undefined>(path: string, body?: B) {
      return $fetch<T>(path, {
        method: 'POST',
        baseURL,
        credentials: 'include',
        body,
      })
    },

    put<T, B extends BodyInit | Record<string, any> | null | undefined>(path: string, body: B) {
      return $fetch<T>(path, {
        method: 'PUT',
        baseURL,
        credentials: 'include',
        body,
      })
    },

    delete<T>(path: string) {
      return $fetch<T>(path, {
        method: 'DELETE',
        baseURL,
        credentials: 'include',
      })
    },
  }
}
