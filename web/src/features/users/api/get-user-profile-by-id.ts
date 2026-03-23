import { queryOptions } from '@tanstack/react-query'
import type { Options } from '@/lib/api/sdk.gen'
import type { UserProfileGetByIdData } from '@/lib/api/types.gen'
import { userProfileGetByIdOptions } from '@/lib/api/@tanstack/react-query.gen'
import { authClient } from '@/lib/clients'

export const getUserProfileByIdQueryOptions = (
  options: Options<UserProfileGetByIdData>,
) => {
  return queryOptions({
    ...userProfileGetByIdOptions({
      client: authClient,
      ...options,
    }),
  })
}
