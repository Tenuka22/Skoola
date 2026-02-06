export type User = {
  id: string
  email: string
  is_verified: boolean
  created_at: string
  updated_at: string
}

export type PaginatedUserResponse = {
  data: User[]
  total: number
  page: number
  limit: number
  total_pages: number
}

export type UserStatsResponse = {
  total_users: number
  verified_users: number
  pending_users: number
  locked_users: number
  auth_methods: {
    google: number
    github: number
    password_only: number
  }
  registration_trend: Array<{ date: string; count: number }>
  top_domains: Array<{ domain: string; count: number }>
}

export type ClientResponseWrapper<T> = {
  data?: T
  error?: unknown
}
