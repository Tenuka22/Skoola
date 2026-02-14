import type { RoleEnum } from '@/lib/api/types.gen'
import { AdminRoutesAllowedRoles } from '@/features/permissions/constants'

export const isAdminRouteRole = (
  role: RoleEnum,
): role is (typeof AdminRoutesAllowedRoles)[number] => {
  return AdminRoutesAllowedRoles.includes(
    role as (typeof AdminRoutesAllowedRoles)[number],
  )
}
