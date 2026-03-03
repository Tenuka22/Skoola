import { queryOptions } from '@tanstack/react-query'
import type { Options } from '@/lib/api/sdk.gen'

import type { GetTimetableByTeacherData } from '@/lib/api/types.gen'
import { getTimetableByTeacherOptions as getTimetableByTeacherOptionsApi } from '@/lib/api/@tanstack/react-query.gen'
import { authClient } from '@/lib/clients'

export const getTimetableByTeacherQueryOptions = (
  options: Options<GetTimetableByTeacherData>,
) => {
  return queryOptions({
    ...getTimetableByTeacherOptionsApi({
      client: authClient,
      ...options,
    }),
  })
}
