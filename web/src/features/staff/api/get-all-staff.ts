import { queryOptions } from '@tanstack/react-query'
import type { Options } from '@/lib/api/sdk.gen'

import type { GetAllStaffData } from '@/lib/api/types.gen'
import { getAllStaffOptions as getAllStaffOptionsApi } from '@/lib/api/@tanstack/react-query.gen'
import { authClient } from '@/lib/clients'

export const getAllStaffQueryOptions = (options?: Options<GetAllStaffData>) => {
  return queryOptions({
    ...getAllStaffOptionsApi({
      client: authClient,
      ...options,
    }),
  })
}
