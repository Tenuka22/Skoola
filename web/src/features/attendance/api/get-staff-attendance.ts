import { queryOptions } from '@tanstack/react-query'
import type { Options } from '@/lib/api/sdk.gen'
import type { GetStaffAttendanceByDateData } from '@/lib/api/types.gen'
import { getStaffAttendanceByDateOptions } from '@/lib/api/@tanstack/react-query.gen'
import { authClient } from '@/lib/clients'

export const getStaffAttendanceQueryOptions = (
  options: Options<GetStaffAttendanceByDateData>,
) => {
  return queryOptions({
    ...getStaffAttendanceByDateOptions({
      client: authClient,
      ...options,
    }),
  })
}
