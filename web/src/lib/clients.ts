import { createClient } from './api/client/index'
import { getActiveSession } from '@/lib/auth/session'
import { env } from '@/lib/env'

// Base configuration
const baseConfig = {
  baseUrl: env.VITE_API_URL,
}

// 1. Public Client (No Auth)
export const publicClient = createClient(baseConfig)

// 2. Auth Client (Client-Side Interceptor)
export const authClient = createClient(baseConfig)

authClient.interceptors.request.use((request) => {
  // Only runs on client-side mostly, or if document is defined
  const session = getActiveSession()
  if (session?.token) {
    request.headers.set('Authorization', `Bearer ${session.token}`)
  }
  return request
})

// 3. Server-Side Client Creator
// Pass the cookie string from the request headers
export const createServerClient = (cookieString: string) => {
  const client = createClient(baseConfig)

  const session = getActiveSession(cookieString)
  if (session?.token) {
    client.interceptors.request.use((request) => {
      request.headers.set('Authorization', `Bearer ${session.token}`)
      return request
    })
  }

  return client
}
