import { queryOptions } from '@tanstack/react-query'
import type { Options } from '@/lib/api/sdk.gen'
import type { StaffQualificationGetAllData } from '@/lib/api/types.gen'
import { staffQualificationGetAllOptions } from '@/lib/api/@tanstack/react-query.gen'
import { authClient } from '@/lib/clients'

export const getStaffQualificationsQueryOptions = (
  options?: Options<StaffQualificationGetAllData>,
) => {
  return queryOptions({
    ...staffQualificationGetAllOptions({
      client: authClient,
      ...options,
    }),
  })
}
