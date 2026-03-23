import { queryOptions } from '@tanstack/react-query'
import type { Options } from '@/lib/api/sdk.gen'
import type { StaffDepartmentGetAllData } from '@/lib/api/types.gen'
import { staffDepartmentGetAllOptions } from '@/lib/api/@tanstack/react-query.gen'
import { authClient } from '@/lib/clients'

export const getStaffDepartmentsQueryOptions = (
  options?: Options<StaffDepartmentGetAllData>,
) => {
  return queryOptions({
    ...staffDepartmentGetAllOptions({
      client: authClient,
      ...options,
    }),
  })
}
