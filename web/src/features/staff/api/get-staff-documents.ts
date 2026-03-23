import { queryOptions } from '@tanstack/react-query'
import type { Options } from '@/lib/api/sdk.gen'
import type { StaffDocumentGetAllData } from '@/lib/api/types.gen'
import { staffDocumentGetAllOptions } from '@/lib/api/@tanstack/react-query.gen'
import { authClient } from '@/lib/clients'

export const getStaffDocumentsQueryOptions = (
  options?: Options<StaffDocumentGetAllData>,
) => {
  return queryOptions({
    ...staffDocumentGetAllOptions({
      client: authClient,
      ...options,
    }),
  })
}
