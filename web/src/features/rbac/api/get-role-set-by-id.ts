import { queryOptions } from '@tanstack/react-query'
import type { Options } from '@/lib/api/sdk.gen'
import type { RoleSetGetByIdData } from '@/lib/api/types.gen'
import { roleSetGetByIdOptions } from '@/lib/api/@tanstack/react-query.gen'
import { authClient } from '@/lib/clients'

export const getRoleSetByIdQueryOptions = (
  options: Options<RoleSetGetByIdData>,
) => {
  return queryOptions({
    ...roleSetGetByIdOptions({
      client: authClient,
      ...options,
    }),
  })
}

export const getRoleSetByIdQueryKey = (options: Options<RoleSetGetByIdData>) =>
  roleSetGetByIdOptions({ client: authClient, ...options }).queryKey
