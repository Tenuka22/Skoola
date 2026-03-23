import { queryOptions } from '@tanstack/react-query'
import type { Options } from '@/lib/api/sdk.gen'
import type { StaffMediaGetAllData } from '@/lib/api/types.gen'
import { staffMediaGetAllOptions } from '@/lib/api/@tanstack/react-query.gen'
import { authClient } from '@/lib/clients'

export const getStaffMediaQueryOptions = (
  options?: Options<StaffMediaGetAllData>,
) => {
  return queryOptions({
    ...staffMediaGetAllOptions({
      client: authClient,
      ...options,
    }),
  })
}
