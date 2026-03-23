import { queryOptions } from '@tanstack/react-query'
import type { Options } from '@/lib/api/sdk.gen'
import type { StaffSkillGetAllData } from '@/lib/api/types.gen'
import { staffSkillGetAllOptions } from '@/lib/api/@tanstack/react-query.gen'
import { authClient } from '@/lib/clients'

export const getStaffSkillsQueryOptions = (
  options?: Options<StaffSkillGetAllData>,
) => {
  return queryOptions({
    ...staffSkillGetAllOptions({
      client: authClient,
      ...options,
    }),
  })
}
