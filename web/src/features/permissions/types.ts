import type { Permission, RoleEnum, UserResponse } from '@/lib/api/types.gen'

export interface PermissionSet {
  id: string
  name: string
  description: string
  created_at: string
  updated_at: string
}

export interface PermissionSetWithPermissions extends PermissionSet {
  permissions: Array<Permission>
}

export interface UserWithRoles extends UserResponse {
  roles: Array<RoleEnum>
  permission_sets: Array<PermissionSet>
  direct_permissions: Array<Permission>
  all_permissions: Array<Permission>
}
