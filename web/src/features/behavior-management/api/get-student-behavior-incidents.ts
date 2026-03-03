import { queryOptions } from '@tanstack/react-query'
import type { Options } from '@/lib/api/sdk.gen'
import type { GetStudentBehaviorIncidentsData } from '@/lib/api/types.gen'
import { getStudentBehaviorIncidentsOptions } from '@/lib/api/@tanstack/react-query.gen'
import { authClient } from '@/lib/clients'

export const getStudentBehaviorIncidentsQueryOptions = (
  options: Options<GetStudentBehaviorIncidentsData>,
) => {
  return queryOptions({
    ...getStudentBehaviorIncidentsOptions({
      client: authClient,
      ...options,
    }),
  })
}
