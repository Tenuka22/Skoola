import { AdminRoutesAllowedRoles } from '@/features/permissions/constants'
import type { RoleEnum } from '@/lib/api/types.gen'

export const isAdminRouteRole = (role: RoleEnum): role is (typeof AdminRoutesAllowedRoles)[number] => {
  return AdminRoutesAllowedRoles.includes(role as (typeof AdminRoutesAllowedRoles)[number])
}
