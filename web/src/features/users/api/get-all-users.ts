import { queryOptions } from '@tanstack/react-query'
import type { Options } from '@/lib/api/sdk.gen'

import type { GetAllUserData } from '@/lib/api'
import { getAllUserOptions } from '@/lib/api/@tanstack/react-query.gen'
import { authClient } from '@/lib/clients'

export const getUsersQueryOptions = (options?: Options<GetAllUserData>) => {
  return queryOptions({
    ...getAllUserOptions({
      client: authClient,
      ...options,
    }),
  })
}
