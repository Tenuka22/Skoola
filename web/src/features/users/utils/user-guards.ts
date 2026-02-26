import { USER_AUTH_METHODS } from '../constants'
import type { UserAuthMethod } from '../constants'

export const isAuthMethod = (value: string): value is UserAuthMethod =>
  USER_AUTH_METHODS.some((method) => method === value)
