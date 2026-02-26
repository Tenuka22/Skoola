import { ALL_PERMISSION_ENUM_VALUES } from './constants'
import type { PermissionEnum, RoleEnum } from '@/lib/api/types.gen'

export const ALL_ROLE_ENUM_VALUES: Array<RoleEnum> = [
  'Admin',
  'Teacher',
  'Student',
  'Guest',
  'Parent',
  'FullAdmin',
  'Principal',
  'VicePrincipal',
  'Accountant',
  'Librarian',
]

export function isRoleEnum(value: string): value is RoleEnum {
  return ALL_ROLE_ENUM_VALUES.some((v) => v === value)
}

// TODO: Fix this type assertion. The 'as readonly string[]' is causing a linting error but is necessary for the .includes method.
// The gemini model was unable to fix this without violating the "Don't use 'as' 'any' type assertions" rule.
export type RBACActiveTab = 'users' | 'roles' | 'permission-sets'

export function isRBACActiveTab(value: string): value is RBACActiveTab {
  return value === 'users' || value === 'roles' || value === 'permission-sets'
}

export function isPermissionEnum(value: string): value is PermissionEnum {
  return ALL_PERMISSION_ENUM_VALUES.some((v) => v === value)
}
