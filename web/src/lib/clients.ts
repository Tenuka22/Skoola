import { createClient } from './api/client/index'
import { env } from '@/lib/env'
import { getActiveSessionServer } from './auth/session'

const baseConfig = {
  baseUrl: env.VITE_API_URL,
}

export const publicClient = createClient(baseConfig)

export const authClient = createClient(baseConfig)

authClient.interceptors.request.use(async (request) => {
  const session = await getActiveSessionServer()
  if (session?.token) {
    request.headers.set('Authorization', `Bearer ${session.token}`)
  }
  return request
})

export const createServerClient = async () => {
  const client = createClient(baseConfig)

  // Await the session here, as the interceptor is set up synchronously
  const session = await getActiveSessionServer()

  if (session?.token) {
    client.interceptors.request.use((request) => {
      request.headers.set('Authorization', `Bearer ${session.token}`)
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
