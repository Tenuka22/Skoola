import { queryOptions } from '@tanstack/react-query'
import type { Options } from '@/lib/api/sdk.gen'
import type { GetStudentByIdData } from '@/lib/api/types.gen'
import { getStudentByIdOptions } from '@/lib/api/@tanstack/react-query.gen'
import { authClient } from '@/lib/clients'

export const getStudentByIdQueryOptions = (
  options: Options<GetStudentByIdData>,
) => {
  return queryOptions({
    ...getStudentByIdOptions({
      client: authClient,
      ...options,
    }),
  })
}
