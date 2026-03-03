import { queryOptions } from '@tanstack/react-query'
import type { Options } from '@/lib/api/sdk.gen'

import type { GetAllStudentsData } from '@/lib/api/types.gen'
import { getAllStudentsOptions } from '@/lib/api/@tanstack/react-query.gen'
import { authClient } from '@/lib/clients'

export const getAllStudentsQueryOptions = (
  options?: Options<GetAllStudentsData>,
) => {
  return queryOptions({
    ...getAllStudentsOptions({
      client: authClient,
      ...options,
    }),
  })
}
