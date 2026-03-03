import { queryOptions } from '@tanstack/react-query'
import type { Options } from '@/lib/api/sdk.gen'

import type { GetAllClassesData } from '@/lib/api/types.gen'
import { getAllClassesOptions } from '@/lib/api/@tanstack/react-query.gen'
import { authClient } from '@/lib/clients'

export const getAllClassesQueryOptions = (
  options?: Options<GetAllClassesData>,
) => {
  return queryOptions({
    ...getAllClassesOptions({
      client: authClient,
      ...options,
    }),
  })
}
