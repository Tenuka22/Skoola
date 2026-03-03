import { queryOptions } from '@tanstack/react-query'
import type { Options } from '@/lib/api/sdk.gen'

import type { GetAllUsersData } from '@/lib/api/types.gen'
import { getAllUsersOptions } from '@/lib/api/@tanstack/react-query.gen'
import { authClient } from '@/lib/clients'

export const getUsersQueryOptions = (options?: Options<GetAllUsersData>) => {
  return queryOptions({
    ...getAllUsersOptions({
      client: authClient,
      ...options,
    }),
  })
}
