import { queryOptions } from '@tanstack/react-query'
import type { Options } from '@/lib/api/sdk.gen'
import type { GetAllBehaviorIncidentTypesData } from '@/lib/api/types.gen'
import { getAllBehaviorIncidentTypesOptions } from '@/lib/api/@tanstack/react-query.gen'
import { authClient } from '@/lib/clients'

export const getAllBehaviorIncidentTypesQueryOptions = (
  options?: Options<GetAllBehaviorIncidentTypesData>,
) => {
  return queryOptions({
    ...getAllBehaviorIncidentTypesOptions({
      client: authClient,
      ...options,
    }),
  })
}
