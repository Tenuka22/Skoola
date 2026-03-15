import { queryOptions } from '@tanstack/react-query'
import type { Options } from '@/lib/api/sdk.gen'
import type { GetAllUserSetUserData } from '@/lib/api/types.gen'
import { getAllUserSetUserOptions } from '@/lib/api/@tanstack/react-query.gen'
import { authClient } from '@/lib/clients'

export const getAllUserSetUserQueryOptions = (
  options?: Options<GetAllUserSetUserData>,
) => {
  return queryOptions({
    ...getAllUserSetUserOptions({
      client: authClient,
      ...options,
    }),
  })
}
