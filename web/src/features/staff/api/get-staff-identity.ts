import { queryOptions } from '@tanstack/react-query'
import type { Options } from '@/lib/api/sdk.gen'
import type { StaffIdentityGetAllData } from '@/lib/api/types.gen'
import { staffIdentityGetAllOptions } from '@/lib/api/@tanstack/react-query.gen'
import { authClient } from '@/lib/clients'

export const getStaffIdentityQueryOptions = (
  options?: Options<StaffIdentityGetAllData>,
) => {
  return queryOptions({
    ...staffIdentityGetAllOptions({
      client: authClient,
      ...options,
    }),
  })
}
