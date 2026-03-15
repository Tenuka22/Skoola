import type { PaginatedResponseForUserResponse, UserResponse } from '@/lib/api/types.gen'
import type { UserAuthMethod } from './constants'

export type User = UserResponse & {
  auth_method?: UserAuthMethod | string | null
}
export type { PaginatedResponseForUserResponse }
