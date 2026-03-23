import { queryOptions } from '@tanstack/react-query'
import type { Options } from '@/lib/api/sdk.gen'
import type { StaffEmploymentStatusGetAllData } from '@/lib/api/types.gen'
import { staffEmploymentStatusGetAllOptions } from '@/lib/api/@tanstack/react-query.gen'
import { authClient } from '@/lib/clients'

export const getStaffEmploymentStatusQueryOptions = (
  options?: Options<StaffEmploymentStatusGetAllData>,
) => {
  return queryOptions({
    ...staffEmploymentStatusGetAllOptions({
      client: authClient,
      ...options,
    }),
  })
}
