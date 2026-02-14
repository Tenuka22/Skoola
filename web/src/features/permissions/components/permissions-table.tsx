'use client'

import * as React from 'react'
import { usePermissionsStore } from '../store'
import { getPermissionColumns } from './permission-table-columns'
import type { Permission } from '@/lib/api/types.gen'
import type { OnChangeFn, SortingState } from '@tanstack/react-table'
import { DataTable } from '@/components/ui/data-table'

interface PermissionsTableProps {
  permissions: Array<Permission>
  isLoading: boolean
  page: number
  limit: number
  totalPages: number
  setPage: (page: number) => void
  setSearch: (search: string) => void
  setSorting: OnChangeFn<SortingState>
}

export function PermissionsTable({
  permissions,
  isLoading,
  page,
  limit,
  totalPages,
  setPage,
  setSorting,
}: PermissionsTableProps) {
  const { setPermissionToEdit, setPermissionToDelete, permissionsSorting } =
    usePermissionsStore()

  const columns = React.useMemo(
    () =>
      getPermissionColumns({
        onEdit: (permission) => setPermissionToEdit(permission),
        onDelete: (permissionId) => setPermissionToDelete(permissionId),
      }),
    [setPermissionToEdit, setPermissionToDelete],
  )

  return (
    <div className="space-y-4">
      <div className={isLoading ? 'opacity-50 pointer-events-none' : ''}>
        <DataTable
          columns={columns}
          data={permissions}
          pageIndex={page - 1}
          pageSize={limit}
          pageCount={totalPages}
          canPreviousPage={page > 1}
          canNextPage={page < totalPages}
          fetchPreviousPage={() => setPage(page - 1)}
          fetchNextPage={() => setPage(page + 1)}
          sorting={permissionsSorting}
          onSortingChange={setSorting}
        />
      </div>
    </div>
  )
}
