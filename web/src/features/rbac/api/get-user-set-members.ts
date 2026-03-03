import { queryOptions } from '@tanstack/react-query'
import type { Options } from '@/lib/api/sdk.gen'
import type { GetUserSetMembersData } from '@/lib/api/types.gen'
import { getUserSetMembersOptions } from '@/lib/api/@tanstack/react-query.gen'
import { authClient } from '@/lib/clients'

export const getUserSetMembersQueryOptions = (
  options: Options<GetUserSetMembersData>,
) => {
  return queryOptions({
    ...getUserSetMembersOptions({
      client: authClient,
      ...options,
    }),
  })
}
