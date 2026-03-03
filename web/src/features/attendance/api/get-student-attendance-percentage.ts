import { queryOptions } from '@tanstack/react-query'
import type { Options } from '@/lib/api/sdk.gen'
import type { CalculateStudentAttendancePercentageData } from '@/lib/api/types.gen'
import { calculateStudentAttendancePercentageOptions } from '@/lib/api/@tanstack/react-query.gen'
import { authClient } from '@/lib/clients'

export const getStudentAttendancePercentageQueryOptions = (
  options: Options<CalculateStudentAttendancePercentageData>,
) => {
  return queryOptions({
    ...calculateStudentAttendancePercentageOptions({
      client: authClient,
      ...options,
    }),
  })
}
