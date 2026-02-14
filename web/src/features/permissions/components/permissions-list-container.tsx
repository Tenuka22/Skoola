import { usePermissionsStore } from '../store'
import { PermissionsTable } from './permissions-table'
import { PermissionSetsList } from './permission-sets-list'
import type { UseQueryResult } from '@tanstack/react-query'
import type { PaginatedPermissionResponse } from '@/lib/api/types.gen'
import type { PermissionSet } from '../types'

interface PermissionsListContainerProps {
  permissionsQuery: UseQueryResult<PaginatedPermissionResponse>
  permissionSetsQuery: UseQueryResult<Array<PermissionSet>>
  limit: number
}

export function PermissionsListContainer({
  permissionsQuery,
  permissionSetsQuery,
  limit,
}: PermissionsListContainerProps) {
  const { view, permissionsPage, setPermissionsPage, setPermissionsSorting } =
    usePermissionsStore()

  if (view === 'permissions') {
    return (
      <div className="px-8">
        <PermissionsTable
          permissions={permissionsQuery.data?.data || []}
          isLoading={permissionsQuery.isLoading}
          page={permissionsPage}
          limit={limit}
          totalPages={permissionsQuery.data?.total_pages || 0}
          setPage={setPermissionsPage}
          setSearch={() => {}} // Search handled by store and toolbar
          setSorting={setPermissionsSorting}
        />
      </div>
    )
  }

  return (
    <div className="px-8">
      <PermissionSetsList
        permissionSets={permissionSetsQuery.data || []}
        isLoading={permissionSetsQuery.isLoading}
        allPermissions={permissionsQuery.data?.data || []}
      />
    </div>
  )
}
