import { queryOptions } from '@tanstack/react-query'
import type { Options } from '@/lib/api/sdk.gen'

import type { GetUserStatisticsData } from '@/lib/api/types.gen'
import { getUserStatisticsOptions } from '@/lib/api/@tanstack/react-query.gen'
import { authClient } from '@/lib/clients'

export const getUserStatsQueryOptions = (
  options?: Options<GetUserStatisticsData>,
) => {
  return queryOptions({
    ...getUserStatisticsOptions({
      client: authClient,
      ...options,
    }),
  })
}
