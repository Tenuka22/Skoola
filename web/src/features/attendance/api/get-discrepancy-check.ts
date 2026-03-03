import { queryOptions } from '@tanstack/react-query'
import type { Options } from '@/lib/api/sdk.gen'
import type { RunDiscrepancyCheckData } from '@/lib/api/types.gen'
import { runDiscrepancyCheckOptions } from '@/lib/api/@tanstack/react-query.gen'
import { authClient } from '@/lib/clients'

export const getDiscrepancyCheckQueryOptions = (
  options: Options<RunDiscrepancyCheckData>,
) => {
  return queryOptions({
    ...runDiscrepancyCheckOptions({
      client: authClient,
      ...options,
    }),
  })
}
