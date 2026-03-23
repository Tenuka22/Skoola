import { queryOptions } from '@tanstack/react-query'
import type { Options } from '@/lib/api/sdk.gen'
import type { StaffGetByIdData } from '@/lib/api/types.gen'
import { staffGetByIdOptions } from '@/lib/api/@tanstack/react-query.gen'
import { authClient } from '@/lib/clients'

export const getStaffByIdQueryOptions = (options: Options<StaffGetByIdData>) => {
  return queryOptions({
    ...staffGetByIdOptions({
      client: authClient,
      ...options,
    }),
  })
}
