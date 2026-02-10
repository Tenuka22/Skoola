import type { Permission, RoleEnum, UserResponse } from '@/lib/api/types.gen'

export interface PermissionSet {
  id: string
  name: string
  description: string
  created_at: string
  updated_at: string
}

export interface PermissionSetWithPermissions extends PermissionSet {
  permissions: Permission[]
}

export interface UserWithRoles extends UserResponse {
  roles: RoleEnum[]
  permission_sets: PermissionSet[]
  direct_permissions: Permission[]
  all_permissions: Permission[]
}