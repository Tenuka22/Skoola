import { queryOptions } from '@tanstack/react-query'
import type { Options } from '@/lib/api/sdk.gen'
import type { GenerateAttendanceReportData } from '@/lib/api/types.gen'
import { generateAttendanceReportOptions } from '@/lib/api/@tanstack/react-query.gen'
import { authClient } from '@/lib/clients'

export const getAttendanceReportQueryOptions = (
  options: Options<GenerateAttendanceReportData>,
) => {
  return queryOptions({
    ...generateAttendanceReportOptions({
      client: authClient,
      ...options,
    }),
  })
}
