import { queryOptions } from '@tanstack/react-query'
import type { Options } from '@/lib/api/sdk.gen'
import type { UserProfileGetAllData } from '@/lib/api/types.gen'
import { userProfileGetAllOptions } from '@/lib/api/@tanstack/react-query.gen'
import { authClient } from '@/lib/clients'

export const getUserProfilesQueryOptions = (
  options?: Options<UserProfileGetAllData>,
) => {
  return queryOptions({
    ...userProfileGetAllOptions({
      client: authClient,
      ...options,
    }),
  })
}
