import type { GetAllAuditLogsData } from '@/lib/api/types.gen'
import type { Options } from '@/lib/api/sdk.gen'
import { getAllAuditLogsOptions } from '@/lib/api/@tanstack/react-query.gen'
import { authClient } from '@/lib/clients'

export const getAllAuditLogsQueryOptions = (
  options?: Options<GetAllAuditLogsData>,
) => {
  return getAllAuditLogsOptions({
    client: authClient,
    ...options,
  })
}
