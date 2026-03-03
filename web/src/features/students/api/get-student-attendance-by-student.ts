import type { GetAttendanceByStudentData } from '@/lib/api/types.gen'
import type { Options } from '@/lib/api/sdk.gen'
import { getAttendanceByStudentOptions } from '@/lib/api/@tanstack/react-query.gen'
import { authClient } from '@/lib/clients'

export const getStudentAttendanceByStudentQueryOptions = (
  options: Options<GetAttendanceByStudentData>,
) => {
  return getAttendanceByStudentOptions({
    client: authClient,
    ...options,
  })
}
