import { queryOptions } from '@tanstack/react-query'
import type { GetStaffPermissionSetsData } from '@/lib/api/types.gen'
import type { Options } from '@/lib/api/sdk.gen'
import { authClient } from '@/lib/clients'
import { getStaffPermissionSetsOptions as getStaffPermissionSetsOptionsApi } from '@/lib/api/@tanstack/react-query.gen'

export const getStaffPermissionSetsQueryOptions = (
  options: Options<GetStaffPermissionSetsData>,
) => {
  return queryOptions({
    ...getStaffPermissionSetsOptionsApi({
      client: authClient,
      ...options,
    }),
  })
}
