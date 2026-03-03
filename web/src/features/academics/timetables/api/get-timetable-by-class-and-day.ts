import { queryOptions } from '@tanstack/react-query'
import type { Options } from '@/lib/api/sdk.gen'

import type { GetTimetableByClassAndDayData } from '@/lib/api/types.gen'
import { getTimetableByClassAndDayOptions as getTimetableByClassAndDayOptionsApi } from '@/lib/api/@tanstack/react-query.gen'
import { authClient } from '@/lib/clients'

export const getTimetableByClassAndDayQueryOptions = (
  options: Options<GetTimetableByClassAndDayData>,
) => {
  return queryOptions({
    ...getTimetableByClassAndDayOptionsApi({
      client: authClient,
      ...options,
    }),
  })
}
