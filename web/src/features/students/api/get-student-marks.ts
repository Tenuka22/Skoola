import type { GetStudentMarksByStudentIdData } from '@/lib/api/types.gen'
import type { Options } from '@/lib/api/sdk.gen'
import { getStudentMarksByStudentIdOptions } from '@/lib/api/@tanstack/react-query.gen'
import { authClient } from '@/lib/clients'

export const getStudentMarksQueryOptions = (
  options: Options<GetStudentMarksByStudentIdData>,
) => {
  return getStudentMarksByStudentIdOptions({
    client: authClient,
    ...options,
  })
}
