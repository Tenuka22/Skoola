import { queryOptions } from '@tanstack/react-query'
import type { Options } from '@/lib/api/sdk.gen'
import type { GetAttendanceByStudentData } from '@/lib/api/types.gen'
import { getAttendanceByStudentOptions } from '@/lib/api/@tanstack/react-query.gen'
import { authClient } from '@/lib/clients'

export const getAttendanceByStudentQueryOptions = (
  options: Options<GetAttendanceByStudentData>,
) => {
  return queryOptions({
    ...getAttendanceByStudentOptions({
      client: authClient,
      ...options,
    }),
  })
}
