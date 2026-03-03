import { queryOptions } from '@tanstack/react-query'
import type { Options } from '@/lib/api/sdk.gen'

import type { GetAllGradeLevelsData } from '@/lib/api/types.gen'
import { getAllGradeLevelsOptions } from '@/lib/api/@tanstack/react-query.gen'
import { authClient } from '@/lib/clients'

export const getAllGradeLevelsQueryOptions = (
  options?: Options<GetAllGradeLevelsData>,
) => {
  return queryOptions({
    ...getAllGradeLevelsOptions({
      client: authClient,
      ...options,
    }),
  })
}
