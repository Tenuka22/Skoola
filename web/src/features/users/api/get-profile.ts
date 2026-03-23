import { queryOptions } from '@tanstack/react-query'
import type { Options } from '@/lib/api/sdk.gen'
import type { GetProfileData } from '@/lib/api/types.gen'
import { getProfileOptions } from '@/lib/api/@tanstack/react-query.gen'
import { authClient } from '@/lib/clients'

export const getProfileQueryOptions = (options?: Options<GetProfileData>) => {
  return queryOptions({
    ...getProfileOptions({
      client: authClient,
      ...options,
    }),
  })
}
