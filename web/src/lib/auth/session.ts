import { z } from 'zod'
import { createServerFn } from '@tanstack/react-start'
import { getCookie, setCookie } from '@tanstack/react-start/server'
import { zTokenResponse, zUserProfileResponse } from '../api/zod.gen'

export const AUTH_COOKIE_NAME = 'skoola_auth'
export const AUTH_COOKIE_TTL = 30 * 24 * 60 * 60

export const SessionSchema = z.object({
  tokens: zTokenResponse,
  user: zUserProfileResponse,
  expiresAt: z.number().optional(),
})

export const AuthStorageSchema = z.object({
  activeUserId: z.string().nullable(),
  sessions: z.record(z.string(), SessionSchema),
})

export type Session = z.infer<typeof SessionSchema>
export type AuthStorage = z.infer<typeof AuthStorageSchema>

async function getAuthStorageFromCookie(): Promise<AuthStorage | null> {
  try {
    const cookieValue = getCookie(AUTH_COOKIE_NAME)
    if (!cookieValue) {
      return null
    }

    const decoded = decodeURIComponent(cookieValue)
    const parsed: unknown = JSON.parse(decoded)
    const result = AuthStorageSchema.safeParse(parsed)

    if (!result.success) {
      console.error('Auth storage validation failed:', result)
      return null
    }

    return result.data
  } catch (error) {
    console.error('Failed to parse auth storage:', error)
    return null
  }
}

export const getAuthStorageServer = createServerFn({ method: 'GET' }).handler(
  async () => {
    return await getAuthStorageFromCookie()
  },
)

export const getActiveSessionServer = createServerFn({ method: 'GET' }).handler(
  async () => {
    const storage = await getAuthStorageFromCookie()
    if (!storage || !storage.activeUserId) {
      return null
    }

    const session = storage.sessions[storage.activeUserId]
    if (!session) {
      return null
    }

    if (session.expiresAt && session.expiresAt < Date.now()) {
      return null
    }

    return session
  },
)

export const setAuthStorageServer = createServerFn({ method: 'POST' })
  .inputValidator((data: AuthStorage) => AuthStorageSchema.parse(data))
  .handler(async ({ data }: { data: AuthStorage }) => {
    try {
      const value = encodeURIComponent(JSON.stringify(data))
      setCookie(AUTH_COOKIE_NAME, value, {
        maxAge: AUTH_COOKIE_TTL,
        path: '/',
        sameSite: 'lax',
        httpOnly: false,
        secure: process.env.NODE_ENV === 'production',
      })
      return { success: true }
    } catch (error) {
      console.error('Failed to set auth storage:', error)
      throw error
    }
  })

export const addSessionServer = createServerFn({ method: 'POST' })
  .inputValidator((data: Session) => SessionSchema.parse(data))
  .handler(async ({ data: session_data }: { data: Session }) => {
    const storage = (await getAuthStorageFromCookie()) || {
      activeUserId: null,
      sessions: {},
    }

    session_data.expiresAt = Date.now() + AUTH_COOKIE_TTL * 1000
    storage.sessions[session_data.user.id] = session_data
    storage.activeUserId = session_data.user.id

    const value = encodeURIComponent(JSON.stringify(storage))
    setCookie(AUTH_COOKIE_NAME, value, {
      maxAge: AUTH_COOKIE_TTL,
      path: '/',
      sameSite: 'lax',
      httpOnly: false,
      secure: process.env.NODE_ENV === 'production',
    })

    return { success: true }
  })

export const removeSessionServer = createServerFn({ method: 'POST' })
  .inputValidator((userId?: string) => z.string().optional().parse(userId))
  .handler(async ({ data: userId_data }: { data: string | undefined }) => {
    const storage = await getAuthStorageFromCookie()
    if (!storage) return { success: false }

    const idToRemove = userId_data || storage.activeUserId
    if (idToRemove && storage.sessions[idToRemove]) {
      delete storage.sessions[idToRemove]

      if (storage.activeUserId === idToRemove) {
        const remainingIds = Object.keys(storage.sessions)
        storage.activeUserId = remainingIds.length > 0 ? remainingIds[0] : null
      }

      const value = encodeURIComponent(JSON.stringify(storage))
      setCookie(AUTH_COOKIE_NAME, value, {
        maxAge: AUTH_COOKIE_TTL,
        path: '/',
        sameSite: 'lax',
        httpOnly: false,
        secure: process.env.NODE_ENV === 'production',
      })
    }

    return { success: true }
  })

export const switchUserServer = createServerFn({ method: 'POST' })
  .inputValidator((userId: string) => z.string().parse(userId))
  .handler(async ({ data: userId_data }: { data: string }) => {
    const storage = await getAuthStorageFromCookie()
    if (!storage) return { success: false }

    if (storage.sessions[userId_data]) {
      storage.activeUserId = userId_data

      const value = encodeURIComponent(JSON.stringify(storage))
      setCookie(AUTH_COOKIE_NAME, value, {
        maxAge: AUTH_COOKIE_TTL,
        path: '/',
        sameSite: 'lax',
        httpOnly: false,
        secure: process.env.NODE_ENV === 'production',
      })
    }

    return { success: true }
  })

export const clearAuthServer = createServerFn({ method: 'POST' }).handler(
  async () => {
    setCookie(AUTH_COOKIE_NAME, '', {
      maxAge: -1,
      path: '/',
    })
    return { success: true }
  },
)
