import { queryOptions } from '@tanstack/react-query'
import type { Options } from '@/lib/api/sdk.gen'

import type { GetAllSubjectsData } from '@/lib/api/types.gen'
import { getAllSubjectsOptions } from '@/lib/api/@tanstack/react-query.gen'
import { authClient } from '@/lib/clients'

export const getAllSubjectsQueryOptions = (
  options?: Options<GetAllSubjectsData>,
) => {
  return queryOptions({
    ...getAllSubjectsOptions({
      client: authClient,
      ...options,
    }),
  })
}
