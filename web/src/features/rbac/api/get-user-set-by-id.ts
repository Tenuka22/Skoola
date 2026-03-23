import { queryOptions } from '@tanstack/react-query'
import type { Options } from '@/lib/api/sdk.gen'
import type { UserSetGetByIdData } from '@/lib/api/types.gen'
import { userSetGetByIdOptions } from '@/lib/api/@tanstack/react-query.gen'
import { authClient } from '@/lib/clients'

export const getUserSetByIdQueryOptions = (
  options: Options<UserSetGetByIdData>,
) => {
  return queryOptions({
    ...userSetGetByIdOptions({
      client: authClient,
      ...options,
    }),
  })
}

export const getUserSetByIdQueryKey = (options: Options<UserSetGetByIdData>) =>
  userSetGetByIdOptions({ client: authClient, ...options }).queryKey
