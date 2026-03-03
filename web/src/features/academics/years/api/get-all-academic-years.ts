import { queryOptions } from '@tanstack/react-query'
import type { Options } from '@/lib/api/sdk.gen'

import type { GetAllAcademicYearsData } from '@/lib/api/types.gen'
import { getAllAcademicYearsOptions } from '@/lib/api/@tanstack/react-query.gen'
import { authClient } from '@/lib/clients'

export const getAllAcademicYearsQueryOptions = (
  options?: Options<GetAllAcademicYearsData>,
) => {
  return queryOptions({
    ...getAllAcademicYearsOptions({
      client: authClient,
      ...options,
    }),
  })
}
