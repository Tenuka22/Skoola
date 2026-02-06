import type { AuthStorage, Session } from './types'

const AUTH_COOKIE_NAME = 'skoola_auth'

// Helper to parse cookies
export function parseCookies(cookieString: string): Record<string, string> {
  const cookies: Record<string, string> = {}
  if (!cookieString) return cookies

  cookieString.split(';').forEach((cookie) => {
    const parts = cookie.split('=')
    const name = parts[0]?.trim()
    const value = parts[1]?.trim()
    if (name && value) {
      cookies[name] = decodeURIComponent(value)
    }
  })
  return cookies
}

export function getAuthStorage(cookieString?: string): AuthStorage {
  let cookies: Record<string, string> = {}

  if (typeof document !== 'undefined') {
    cookies = parseCookies(document.cookie)
  } else if (cookieString) {
    cookies = parseCookies(cookieString)
  }

  const authData = cookies[AUTH_COOKIE_NAME]
  if (!authData) {
    return { activeUserId: null, sessions: {} }
  }

  try {
    return JSON.parse(authData)
  } catch {
    return { activeUserId: null, sessions: {} }
  }
}

export function setAuthStorage(storage: AuthStorage) {
  if (typeof document === 'undefined') return

  const value = encodeURIComponent(JSON.stringify(storage))
  const days = 30 // Persist for 30 days
  const date = new Date()
  date.setTime(date.getTime() + days * 24 * 60 * 60 * 1000)
  const expires = `; expires=${date.toUTCString()}`

  document.cookie = `${AUTH_COOKIE_NAME}=${value}${expires}; path=/; SameSite=Lax`
}

export function addSession(session: Session) {
  const storage = getAuthStorage()
  storage.sessions[session.user.id] = session
  storage.activeUserId = session.user.id
  setAuthStorage(storage)
}

export function removeSession(userId?: string) {
  const storage = getAuthStorage()
  const idToRemove = userId || storage.activeUserId

  if (idToRemove && storage.sessions[idToRemove]) {
    delete storage.sessions[idToRemove]

    // If we removed the active user, switch to another if available
    if (storage.activeUserId === idToRemove) {
      const remainingIds = Object.keys(storage.sessions)
      storage.activeUserId = remainingIds.length > 0 ? remainingIds[0] : null
    }

    setAuthStorage(storage)
  }
}

export function getActiveSession(cookieString?: string): Session | null {
  const storage = getAuthStorage(cookieString)
  if (!storage.activeUserId) return null
  return storage.sessions[storage.activeUserId] || null
}

export function switchUser(userId: string) {
  const storage = getAuthStorage()
  if (storage.sessions[userId]) {
    storage.activeUserId = userId
    setAuthStorage(storage)
  }
}
