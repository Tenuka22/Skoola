import { keepPreviousData, useQuery } from '@tanstack/react-query'
import { useUsersStore } from '../store'
import { authClient } from '@/lib/clients'
import {
  getAllUsersOptions,
  getUserStatisticsOptions,
} from '@/lib/api/@tanstack/react-query.gen'

export function useUsers() {
  const {
    page,
    debouncedSearch,
    statusFilter,
    authFilter,
    createdAfter,
    createdBefore,
    sorting,
  } = useUsersStore()

  const limit = 10
  const sortBy = sorting[0]?.id
  type SortOrder = 'asc' | 'desc'
  const sortOrder: SortOrder = sorting[0]?.desc ? 'desc' : 'asc'

  return useQuery({
    ...getAllUsersOptions({
      client: authClient,
      query: {
        page,
        limit,
        search: debouncedSearch,
        is_verified:
          statusFilter === 'all' ? undefined : statusFilter === 'verified',
        auth_method: authFilter === 'all' ? undefined : authFilter,
        created_after: createdAfter ?? undefined,
        created_before: createdBefore ?? undefined,
        sort_by: sortBy,
        sort_order: sortOrder,
      },
    }),
    placeholderData: keepPreviousData,
  })
}

export function useUserStats() {
  return useQuery({
    ...getUserStatisticsOptions({
      client: authClient,
    }),
  })
}
