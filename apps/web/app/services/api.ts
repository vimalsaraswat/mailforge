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
  const client = $fetch.create({
    baseURL,
    credentials: 'include',
    async onResponseError({ response }) {
      if (response.status === 401) {
        // Handle unauthorized access globally
      }
    },
  })

  return {
    get<T>(path: string) {
      return client<T>(path)
    },

    post<T>(path: string, body?: ApiRequestBody) {
      return client<T>(path, { method: 'POST', body })
    },

    put<T>(path: string, body: ApiRequestBody) {
      return client<T>(path, { method: 'PUT', body })
    },

    delete<T>(path: string) {
      return client<T>(path, { method: 'DELETE' })
    },
  }
}
