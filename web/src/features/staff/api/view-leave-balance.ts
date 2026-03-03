import type { ViewLeaveBalanceData } from '@/lib/api/types.gen'
import type { Options } from '@/lib/api/sdk.gen'
import { viewLeaveBalanceOptions } from '@/lib/api/@tanstack/react-query.gen'
import { authClient } from '@/lib/clients'

export const getViewLeaveBalanceQueryOptions = (
  options: Options<ViewLeaveBalanceData>,
) => {
  return viewLeaveBalanceOptions({
    client: authClient,
    ...options,
  })
}
