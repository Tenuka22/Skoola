import type { RoleEnum } from '@/lib/api/types.gen'
import {
  PermissionEnumSchema,
  PermissionSeveritySchema,
} from '@/lib/api/schemas.gen'

export const AdminRoutesAllowedRoles = [
  'Admin',
  'FullAdmin',
  'Principal',
  'VicePrincipal',
  'Accountant',
] satisfies Array<RoleEnum> as Array<RoleEnum>

export const PERMISSION_NAMES = PermissionEnumSchema.enum
export const PERMISSION_SEVERITIES = PermissionSeveritySchema.enum

export const AUTH_COOKIE_NAME = 'skoola_auth'
export const AUTH_COOKIE_TTL = 30 * 24 * 60 * 60
