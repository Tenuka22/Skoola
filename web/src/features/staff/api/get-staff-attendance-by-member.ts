import type { GetStaffAttendanceByStaffMemberData } from '@/lib/api/types.gen'
import type { Options } from '@/lib/api/sdk.gen'
import { getStaffAttendanceByStaffMemberOptions } from '@/lib/api/@tanstack/react-query.gen'
import { authClient } from '@/lib/clients'

export const getStaffAttendanceByMemberQueryOptions = (
  options: Options<GetStaffAttendanceByStaffMemberData>,
) => {
  return getStaffAttendanceByStaffMemberOptions({
    client: authClient,
    ...options,
  })
}
