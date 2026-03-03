import { queryOptions } from '@tanstack/react-query'
import type { Options } from '@/lib/api/sdk.gen'

import type { GetSubjectsByClassData } from '@/lib/api/types.gen'
import { getSubjectsByClassOptions as getSubjectsByClassOptionsApi } from '@/lib/api/@tanstack/react-query.gen'
import { authClient } from '@/lib/clients'

export const getSubjectsByClassQueryOptions = (
  options: Options<GetSubjectsByClassData>,
) => {
  return queryOptions({
    ...getSubjectsByClassOptionsApi({
      client: authClient,
      ...options,
    }),
  })
}
