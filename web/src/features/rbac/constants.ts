export const RBAC_ACTIVE_TABS = [
  'users',
  'roles',
  'permission-sets',
  'role-sets',
] as const

export type RBACActiveTab = (typeof RBAC_ACTIVE_TABS)[number]
