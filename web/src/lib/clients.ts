import {
  
  
  createClient
} from './api/client/index'
import {
  SessionSchema,
  addSessionServer,
  clearAuthServer,
  getActiveSessionServer,
} from './auth/session'
import { postAuthRefresh6Aadba1Bf11B4320428155Ff0462660D as postAuthRefreshApi } from './api/sdk.gen'
import { reloginNeeded } from './auth/actions'
import type {ClientOptions, RequestOptions} from './api/client/index';
import { env } from '@/lib/env'

const HTTP_METHODS = [
  'GET',
  'POST',
  'PUT',
  'PATCH',
  'DELETE',
  'OPTIONS',
  'HEAD',
  'CONNECT',
  'TRACE',
] as const
type HttpMethod = typeof HTTP_METHODS[number]

function isHttpMethod(method: string): method is HttpMethod {
  return HTTP_METHODS.includes(method as HttpMethod)
}

const baseConfig: ClientOptions = {
  baseUrl: env.VITE_API_URL,
}

export const publicClient = createClient(baseConfig)

export const authClient = createClient(baseConfig)

let isRefreshing = false
let failedQueue: Array<{
  resolve: (value: Response) => void
  reject: (reason?: unknown) => void
  request: Request
}> = []

const processQueue = (newAccessToken: string | null = null, error: unknown | null = null) => {
  failedQueue.forEach(async (prom) => {
    if (error) {
      prom.reject(error)
    } else if (newAccessToken) {
      const clonedRequest = prom.request.clone()
      clonedRequest.headers.set('Authorization', `Bearer ${newAccessToken}`)

      const url = clonedRequest.url.replace(baseConfig.baseUrl || '', '');
      const method = clonedRequest.method;
      let httpMethod: HttpMethod = 'GET';
      if (isHttpMethod(method)) {
        httpMethod = method;
      } else {
        console.warn(`Unknown HTTP method in queued request: ${method}. Defaulting to GET.`);
      }

      const retryOptions: RequestOptions & { method: HttpMethod } = {
        method: httpMethod,
        url: url,
        headers: Object.fromEntries(clonedRequest.headers.entries()),
      };

      if (clonedRequest.body) {
        const contentType = clonedRequest.headers.get('Content-Type');
        if (contentType?.includes('application/json')) {
          retryOptions.body = await clonedRequest.json();
        } else if (contentType?.includes('application/x-www-form-urlencoded')) {
          retryOptions.body = await clonedRequest.text();
        } else {
          retryOptions.body = await clonedRequest.arrayBuffer();
        }
      }

      try {
        const result = await authClient.request(retryOptions);
        prom.resolve(result.response);
      } catch (retryError) {
        prom.reject(retryError);
      }
    } else {
      prom.reject(new Error('Failed to refresh token and no new access token provided.'));
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
  async (response: Response, request: Request) => {
    const originalRequest = request.clone()
    const originalResponse = response.clone()
    const session = await getActiveSessionServer()

    if (
      originalResponse?.status === 401 &&
      session?.tokens?.refresh_token &&
      originalRequest.url !== `${env.VITE_API_URL}/auth/refresh`
    ) {
      if (isRefreshing) {
        return new Promise<Response>((resolve, reject) => {
          failedQueue.push({ resolve, reject, request: originalRequest })
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

          const clonedRequest = originalRequest.clone()
          clonedRequest.headers.set(
            'Authorization',
            `Bearer ${newSession.tokens.token}`,
          )

          const method = clonedRequest.method
          let httpMethod: HttpMethod = 'GET' // Default method
          if (isHttpMethod(method)) {
            httpMethod = method
          } else {
            console.warn(`Unknown HTTP method: ${method}. Defaulting to GET.`)
            httpMethod = 'GET'
          }

          const retryOptions: RequestOptions & { method: HttpMethod } = {
            method: httpMethod,
            url: clonedRequest.url.replace(baseConfig.baseUrl || '', ''),
            headers: Object.fromEntries(clonedRequest.headers.entries()),
          }

          if (clonedRequest.body) {
            const contentType = clonedRequest.headers.get('Content-Type')
            if (contentType?.includes('application/json')) {
              retryOptions.body = await clonedRequest.json()
            } else if (
              contentType?.includes('application/x-www-form-urlencoded')
            ) {
              retryOptions.body = await clonedRequest.text()
            } else {
              retryOptions.body = await clonedRequest.arrayBuffer()
            }
          }

          processQueue(newSession.tokens.token)
          const result = await authClient.request(retryOptions)
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
