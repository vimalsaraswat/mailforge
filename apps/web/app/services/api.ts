export type ApiRequestBody = BodyInit | object | null

export interface ApiClient {
  get<T>(path: string): Promise<T>
  post<T>(path: string, body?: ApiRequestBody): Promise<T>
  put<T>(path: string, body: ApiRequestBody): Promise<T>
  delete<T = void>(path: string): Promise<T>
}

export interface ApiErrorData {
  message?: string
}

interface FetchErrorLike {
  data?: unknown
}

function isRecord(value: unknown): value is Record<string, unknown> {
  return typeof value === 'object' && value !== null
}

/** Returns the API's human-readable error response when one is available. */
export function getApiErrorMessage(error: unknown): string | null {
  if (!isRecord(error)) return null

  const { data } = error as FetchErrorLike
  if (typeof data === 'string' && data.trim()) return data

  if (isRecord(data) && typeof data.message === 'string' && data.message.trim()) {
    return data.message
  }

  return null
}

export function createApiClient(baseURL: string): ApiClient {
  return {
    get<T>(path: string) {
      return $fetch<T>(path, {
        baseURL,
        credentials: 'include',
      })
    },

    post<T>(path: string, body?: ApiRequestBody) {
      return $fetch<T>(path, {
        method: 'POST',
        baseURL,
        credentials: 'include',
        body,
      })
    },

    put<T>(path: string, body: ApiRequestBody) {
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
