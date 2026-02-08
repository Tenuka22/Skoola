import { z } from 'zod'
import { createServerFn } from '@tanstack/react-start'
import { zTokenResponse, zUserProfileResponse } from '../api/zod.gen'

const AUTH_COOKIE_NAME = 'skoola_auth'
const THIRTY_DAYS_IN_SECONDS = 30 * 24 * 60 * 60

interface CookieOptions {
  maxAge?: number
  path?: string
  expires?: Date
  secure?: boolean
  httpOnly?: boolean
  sameSite?: 'strict' | 'lax' | 'none'
}

function setCookie(
  name: string,
  value: string,
  options: CookieOptions = {},
): void {
  let cookieString = `${name}=${value}`

  if (options.maxAge) {
    cookieString += `; Max-Age=${options.maxAge}`
  }
  if (options.path) {
    cookieString += `; Path=${options.path}`
  }
  if (options.expires) {
    cookieString += `; Expires=${options.expires.toUTCString()}`
  }
  if (options.secure) {
    cookieString += `; Secure`
  }
  if (options.httpOnly) {
    cookieString += `; HttpOnly`
  }
  if (options.sameSite) {
    cookieString += `; SameSite=${options.sameSite}`
  }

  // Check if we are in a browser environment before trying to set the cookie
  if (typeof document !== 'undefined') {
    document.cookie = cookieString
  }
}

function getCookie(name: string): string | undefined {
  if (typeof document === 'undefined') {
    return undefined
  }
  const nameEQ = name + '='
  const ca = document.cookie.split(';')
  for (let i = 0; i < ca.length; i++) {
    let c = ca[i]
    while (c.charAt(0) === ' ') c = c.substring(1, c.length)
    if (c.indexOf(nameEQ) === 0) return c.substring(nameEQ.length, c.length)
  }
  return undefined
}

function deleteCookie(name: string, options: CookieOptions = {}): void {
  setCookie(name, '', { ...options, maxAge: -1 })
}

export const SessionSchema = z.object({
  token: zTokenResponse,
  user: zUserProfileResponse,
  expiresAt: z.number().optional(),
})

export const AuthStorageSchema = z.object({
  activeUserId: z.string().nullable(),
  sessions: z.record(z.string(), SessionSchema),
})

export type Session = z.infer<typeof SessionSchema>
export type AuthStorage = z.infer<typeof AuthStorageSchema>

function getAuthStorageFromCookie(): AuthStorage | null {
  try {
    const cookieValue = getCookie(AUTH_COOKIE_NAME)
    if (!cookieValue) {
      return null
    }
    const decoded = decodeURIComponent(cookieValue)
    const parsed = JSON.parse(decoded)
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
    return getAuthStorageFromCookie()
  },
)

export const getActiveSessionServer = createServerFn({ method: 'GET' }).handler(
  async () => {
    const storage = getAuthStorageFromCookie()
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
        maxAge: THIRTY_DAYS_IN_SECONDS,
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
    const storage = getAuthStorageFromCookie() || {
      activeUserId: null,
      sessions: {},
    }
    session_data.expiresAt = Date.now() + THIRTY_DAYS_IN_SECONDS * 1000

    storage.sessions[session_data.user.id] = session_data
    storage.activeUserId = session_data.user.id

    const value = encodeURIComponent(JSON.stringify(storage))
    setCookie(AUTH_COOKIE_NAME, value, {
      maxAge: THIRTY_DAYS_IN_SECONDS,
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
    const storage = getAuthStorageFromCookie()
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
        maxAge: THIRTY_DAYS_IN_SECONDS,
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
    const storage = getAuthStorageFromCookie()
    if (!storage) return { success: false }

    if (storage.sessions[userId_data]) {
      storage.activeUserId = userId_data

      const value = encodeURIComponent(JSON.stringify(storage))
      setCookie(AUTH_COOKIE_NAME, value, {
        maxAge: THIRTY_DAYS_IN_SECONDS,
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
    deleteCookie(AUTH_COOKIE_NAME, {
      path: '/',
    })
    return { success: true }
  },
)

export const requireAuth = createServerFn({ method: 'GET' }).handler(
  async () => {
    const storage = getAuthStorageFromCookie()
    if (!storage || !storage.activeUserId) {
      throw new Error('Unauthorized')
    }
    const session = storage.sessions[storage.activeUserId]
    if (!session) {
      throw new Error('Unauthorized')
    }
    if (session.expiresAt && session.expiresAt < Date.now()) {
      throw new Error('Unauthorized')
    }
    return session
  },
)

export const getAllSessionsServer = createServerFn({ method: 'GET' }).handler(
  async () => {
    const storage = getAuthStorageFromCookie()
    if (!storage) return []
    return Object.values(storage.sessions)
  },
)
