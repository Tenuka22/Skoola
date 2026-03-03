import type { GetRolePermissionsData } from '@/lib/api/types.gen'
import type { Options } from '@/lib/api/sdk.gen'
import { getRolePermissionsOptions } from '@/lib/api/@tanstack/react-query.gen'
import { authClient } from '@/lib/clients'

export const getRolePermissionsQueryOptions = (
  options: Options<GetRolePermissionsData>,
) => {
  return getRolePermissionsOptions({
    client: authClient,
    ...options,
  })
}
