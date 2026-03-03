import type { GetUserPermissionsData } from '@/lib/api/types.gen'
import type { Options } from '@/lib/api/sdk.gen'
import { getUserPermissionsOptions } from '@/lib/api/@tanstack/react-query.gen'
import { authClient } from '@/lib/clients'

export const getUserPermissionsQueryOptions = (
  options: Options<GetUserPermissionsData>,
) => {
  return getUserPermissionsOptions({
    client: authClient,
    ...options,
  })
}
