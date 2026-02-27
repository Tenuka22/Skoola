import { RBAC_ACTIVE_TABS } from '../constants'
import { ALL_PERMISSION_ENUM_VALUES } from './constants'
import type { RBACActiveTab } from '../constants'
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

export function isRBACActiveTab(value: string): value is RBACActiveTab {
  return RBAC_ACTIVE_TABS.some((tab) => tab === value)
}

export function isPermissionEnum(value: string): value is PermissionEnum {
  return ALL_PERMISSION_ENUM_VALUES.some((v) => v === value)
}
