export interface User {
  id: string
  email: string
  name?: string
  role?: string
  [key: string]: any
}

export interface Session {
  token: string
  refreshToken?: string
  user: User
  expiresAt?: number
}

export interface AuthStorage {
  activeUserId: string | null
  sessions: Record<string, Session>
}
