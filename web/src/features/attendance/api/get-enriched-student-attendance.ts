import { queryOptions } from '@tanstack/react-query'
import type { Options } from '@/lib/api/sdk.gen'
import type { GetEnrichedStudentListData } from '@/lib/api/types.gen'
import { getEnrichedStudentListOptions } from '@/lib/api/@tanstack/react-query.gen'
import { authClient } from '@/lib/clients'

export const getEnrichedStudentAttendanceQueryOptions = (
  options: Options<GetEnrichedStudentListData>,
) => {
  return queryOptions({
    ...getEnrichedStudentListOptions({
      client: authClient,
      ...options,
    }),
  })
}
