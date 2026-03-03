import type { GetAllGuardiansForStudentData } from '@/lib/api/types.gen'
import type { Options } from '@/lib/api/sdk.gen'
import { getAllGuardiansForStudentOptions } from '@/lib/api/@tanstack/react-query.gen'
import { authClient } from '@/lib/clients'

export const getGuardiansForStudentQueryOptions = (
  options: Options<GetAllGuardiansForStudentData>,
) => {
  return getAllGuardiansForStudentOptions({
    client: authClient,
    ...options,
  })
}
