import { queryOptions } from '@tanstack/react-query'
import type { Options } from '@/lib/api/sdk.gen'
import type { UserSetUserGetAllData } from '@/lib/api/types.gen'
import { userSetUserGetAllOptions } from '@/lib/api/@tanstack/react-query.gen'
import { authClient } from '@/lib/clients'

export const getAllUserSetUserQueryOptions = (
  options?: Options<UserSetUserGetAllData>,
) => {
  return queryOptions({
    ...userSetUserGetAllOptions({
      client: authClient,
      ...options,
    }),
  })
}
