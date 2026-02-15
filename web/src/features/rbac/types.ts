import type {
  PermissionEnum,
  RoleEnum,
  UserResponse,
  UserSet,
} from '@/lib/api/types.gen'

export type { UserResponse, UserSet, RoleEnum, PermissionEnum }

export interface RBACStats {
  totalUsers: number
  totalRoles: number
  totalPermissionSets: number
}

export interface PermissionGroup {
  name: string
  permissions: Array<PermissionEnum>
}
