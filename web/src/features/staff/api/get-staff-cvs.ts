import { queryOptions } from '@tanstack/react-query'
import type { Options } from '@/lib/api/sdk.gen'
import type { StaffCvGetAllData } from '@/lib/api/types.gen'
import { staffCvGetAllOptions } from '@/lib/api/@tanstack/react-query.gen'
import { authClient } from '@/lib/clients'

export const getStaffCvsQueryOptions = (options?: Options<StaffCvGetAllData>) => {
  return queryOptions({
    ...staffCvGetAllOptions({
      client: authClient,
      ...options,
    }),
  })
}
