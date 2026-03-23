import { queryOptions } from '@tanstack/react-query'
import type { Options } from '@/lib/api/sdk.gen'
import type { StaffContactGetAllData } from '@/lib/api/types.gen'
import { staffContactGetAllOptions } from '@/lib/api/@tanstack/react-query.gen'
import { authClient } from '@/lib/clients'

export const getStaffContactsQueryOptions = (
  options?: Options<StaffContactGetAllData>,
) => {
  return queryOptions({
    ...staffContactGetAllOptions({
      client: authClient,
      ...options,
    }),
  })
}
