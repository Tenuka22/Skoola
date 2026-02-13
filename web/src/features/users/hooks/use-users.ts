import { keepPreviousData, useQuery } from '@tanstack/react-query'
import { useUsersStore } from '../store'
import { authClient } from '@/lib/clients'
import {
  getUsers06Bdcf95Aafda840B1D04322636De293Options,
  getUsersStatsBf304B57E4A0115F8280C4Bed2Fd9FbaOptions,
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
  const sortOrder = sorting[0]?.desc ? 'desc' : 'asc'

  return useQuery({
    ...getUsers06Bdcf95Aafda840B1D04322636De293Options({
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
        sort_order: sortOrder as any,
      },
    }),
    placeholderData: keepPreviousData,
  })
}

export function useUserStats() {
  return useQuery({
    ...getUsersStatsBf304B57E4A0115F8280C4Bed2Fd9FbaOptions({
      client: authClient,
    }),
  })
}
