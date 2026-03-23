import { queryOptions } from '@tanstack/react-query'
import type { Options } from '@/lib/api/sdk.gen'
import type { RolePermissionGetAllData } from '@/lib/api/types.gen'
import { rolePermissionGetAllOptions } from '@/lib/api/@tanstack/react-query.gen'
import { authClient } from '@/lib/clients'

export const getRolePermissionsAllQueryOptions = (
  options?: Options<RolePermissionGetAllData>,
) => {
  return queryOptions({
    ...rolePermissionGetAllOptions({
      client: authClient,
      ...options,
    }),
  })
}

export const getRolePermissionsAllQueryKey = (
  options?: Options<RolePermissionGetAllData>,
) =>
  rolePermissionGetAllOptions({ client: authClient, ...options }).queryKey
