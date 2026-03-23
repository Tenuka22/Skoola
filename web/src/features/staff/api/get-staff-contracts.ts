import { queryOptions } from '@tanstack/react-query'
import type { Options } from '@/lib/api/sdk.gen'
import type { StaffContractGetAllData } from '@/lib/api/types.gen'
import { staffContractGetAllOptions } from '@/lib/api/@tanstack/react-query.gen'
import { authClient } from '@/lib/clients'

export const getStaffContractsQueryOptions = (
  options?: Options<StaffContractGetAllData>,
) => {
  return queryOptions({
    ...staffContractGetAllOptions({
      client: authClient,
      ...options,
    }),
  })
}
