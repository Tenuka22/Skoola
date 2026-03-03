import { queryOptions } from '@tanstack/react-query'
import type { Options } from '@/lib/api/sdk.gen'
import type { GetStudentsWithLowAttendanceData } from '@/lib/api/types.gen'
import { getStudentsWithLowAttendanceOptions } from '@/lib/api/@tanstack/react-query.gen'
import { authClient } from '@/lib/clients'

export const getLowAttendanceQueryOptions = (
  options: Options<GetStudentsWithLowAttendanceData>,
) => {
  return queryOptions({
    ...getStudentsWithLowAttendanceOptions({
      client: authClient,
      ...options,
    }),
  })
}
