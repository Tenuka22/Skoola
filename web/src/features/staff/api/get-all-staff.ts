import { queryOptions } from '@tanstack/react-query'
import type { Options } from '@/lib/api/sdk.gen'
import type { StaffGetAllData } from '@/lib/api/types.gen'
import { staffGetAllOptions } from '@/lib/api/@tanstack/react-query.gen'
import { authClient } from '@/lib/clients'

export const getStaffQueryOptions = (options?: Options<StaffGetAllData>) => {
  return queryOptions({
    ...staffGetAllOptions({
      client: authClient,
      ...options,
    }),
  })
}
