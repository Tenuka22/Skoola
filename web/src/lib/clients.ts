import { createClient } from './api/client/index'
import {
  SessionSchema,
  addSessionServer,
  clearAuthServer,
  getActiveSessionServer,
} from './auth/session'
import { postAuthRefresh6Aadba1Bf11B4320428155Ff0462660D as postAuthRefreshApi } from './api/sdk.gen'
import { reloginNeeded } from './auth/actions'
import type { ClientOptions, RequestOptions } from './api/client/index'
import { env } from '@/lib/env'

const baseConfig: ClientOptions = {
  baseUrl: env.VITE_API_URL,
}

export const publicClient = createClient(baseConfig)

export const authClient = createClient(baseConfig)

let isRefreshing = false
let failedQueue: Array<{
  resolve: (value: Response) => void
  reject: (reason?: unknown) => void
  options: RequestOptions
}> = []

const processQueue = (
  newAccessToken: string | null = null,
  error: unknown | null = null,
) => {
  failedQueue.forEach(async (prom) => {
    if (error) {
      prom.reject(error)
    } else if (newAccessToken) {
      try {
        const options = prom.options
        const result = await authClient.request({
          ...options,
          method: options.method ?? 'GET',
        })

        prom.resolve(result.response)
      } catch (retryError) {
        prom.reject(retryError)
      }
    } else {
      prom.reject(
        new Error('Failed to refresh token and no new access token provided.'),
      )
    }
  })
  failedQueue = []
}

authClient.interceptors.request.use(async (request) => {
  const session = await getActiveSessionServer()
  if (session?.tokens) {
    request.headers.set('Authorization', `Bearer ${session.tokens.token}`)
  }
  return request
})
authClient.interceptors.response.use(
  // Success handler
  async (response: Response, request: Request, options: RequestOptions) => {
    const session = await getActiveSessionServer()

    if (
      response?.status === 401 &&
      session?.tokens?.refresh_token &&
      request.url !== `${env.VITE_API_URL}/auth/refresh`
    ) {
      if (isRefreshing) {
        return new Promise<Response>((resolve, reject) => {
          failedQueue.push({ resolve, reject, options })
        })
      }

      isRefreshing = true

      try {
        const refreshResponse = await postAuthRefreshApi({
          body: {
            refresh_token: session.tokens.refresh_token,
          },
          client: publicClient,
        })

        if (refreshResponse.data?.token) {
          const newSession = SessionSchema.parse({
            tokens: refreshResponse.data,
            user: session.user,
          })
          await addSessionServer({ data: newSession })

          processQueue(newSession.tokens.token)
          const result = await authClient.request({
            ...options,
            method: options.method ?? 'GET',
          })

          return result.response
        } else {
          throw new Error('Refresh token invalid or expired.')
        }
      } catch (refreshError) {
        console.error('Token refresh failed:', refreshError)
        processQueue(null, refreshError)
        await clearAuthServer()
        reloginNeeded()
        return Promise.reject(refreshError)
      } finally {
        isRefreshing = false
      }
    }
    return response
  },
)

authClient.interceptors.error.use(
  async (error: unknown, response: Response | undefined) => {
    if (response?.status === 401) {
      await clearAuthServer()
      reloginNeeded()
    }
    return Promise.reject(error)
  },
)

export const createServerClient = async () => {
  const client = createClient(baseConfig)

  const session = await getActiveSessionServer()

  if (session?.tokens) {
    client.interceptors.request.use((request) => {
      request.headers.set('Authorization', `Bearer ${session.tokens.token}`)
      return request
    })
  }

  return client
}

export const isServer = typeof window === 'undefined'

export const getUniversalClient = () => {
  if (isServer) {
    return createServerClient()
  }
  return authClient
}
