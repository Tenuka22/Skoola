import { queryOptions } from '@tanstack/react-query'
import type { Options } from '@/lib/api/sdk.gen'
import type { UserSetGetAllData } from '@/lib/api/types.gen'
import { userSetGetAllOptions } from '@/lib/api/@tanstack/react-query.gen'
import { authClient } from '@/lib/clients'

export const getAllUserSetsQueryOptions = (
  options?: Options<UserSetGetAllData>,
) => {
  return queryOptions({
    ...userSetGetAllOptions({
      client: authClient,
      ...options,
    }),
  })
}

export const getUserSetsQueryKey = (options?: Options<UserSetGetAllData>) =>
  userSetGetAllOptions({ client: authClient, ...options }).queryKey
