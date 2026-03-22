import { queryOptions } from '@tanstack/react-query'
import type { Options } from '@/lib/api/sdk.gen'

import type { UserGetAllData } from '@/lib/api/types.gen'
import { userGetAllOptions } from '@/lib/api/@tanstack/react-query.gen'
import { authClient } from '@/lib/clients'

export const getUsersQueryOptions = (options?: Options<UserGetAllData>) => {
  return queryOptions({
    ...userGetAllOptions({
      client: authClient,
      ...options,
    }),
  })
}
