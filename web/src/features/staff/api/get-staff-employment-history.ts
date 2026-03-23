import { queryOptions } from '@tanstack/react-query'
import type { Options } from '@/lib/api/sdk.gen'
import type { StaffEmploymentHistoryGetAllData } from '@/lib/api/types.gen'
import { staffEmploymentHistoryGetAllOptions } from '@/lib/api/@tanstack/react-query.gen'
import { authClient } from '@/lib/clients'

export const getStaffEmploymentHistoryQueryOptions = (
  options?: Options<StaffEmploymentHistoryGetAllData>,
) => {
  return queryOptions({
    ...staffEmploymentHistoryGetAllOptions({
      client: authClient,
      ...options,
    }),
  })
}
