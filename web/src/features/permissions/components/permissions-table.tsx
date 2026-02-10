'use client'

import * as React from 'react'
import { type SortingState, type OnChangeFn } from '@tanstack/react-table'
import { DataTable } from '@/components/ui/data-table'
import { Button } from '@/components/ui/button'
import { HugeiconsIcon } from '@hugeicons/react'
import { Search01Icon, PlusSignIcon } from '@hugeicons/core-free-icons'
import { Input } from '@/components/ui/input'
import type { Permission } from '@/lib/api/types.gen'
import { getPermissionColumns } from './permission-table-columns'
import { CreatePermissionDialog } from './create-permission-dialog'
import { EditPermissionDialog } from './edit-permission-dialog'
import { useMutation, useQueryClient } from '@tanstack/react-query'
import { deletePermission } from '../../permissions/api'
import { toast } from 'sonner'

interface PermissionsTableProps {
  permissions: Permission[]
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
  setSearch,
  setSorting,
}: PermissionsTableProps) {
  const queryClient = useQueryClient()
  const [isCreateDialogOpen, setIsCreateDialogOpen] = React.useState(false)
  const [editPermission, setEditPermission] = React.useState<Permission | null>(
    null,
  )

  const deleteMutation = useMutation({
    mutationFn: deletePermission,
    onSuccess: () => {
      toast.success('Permission deleted successfully.')
      queryClient.invalidateQueries({ queryKey: ['permissions'] })
    },
    onError: (error) => {
      toast.error(`Failed to delete permission: ${(error as any).message}`)
    },
  })

  const columns = React.useMemo(
    () =>
      getPermissionColumns({
        onEdit: (permission) => setEditPermission(permission),
        onDelete: (permissionId) => deleteMutation.mutate(permissionId),
      }),
    [deleteMutation],
  )

  return (
    <div className="space-y-4">
      <div className="flex items-center gap-3 p-4">
        <div className="relative group">
          <HugeiconsIcon
            icon={Search01Icon}
            className="absolute left-3 top-1/2 size-4 -translate-y-1/2 text-muted-foreground transition-colors group-focus-within:text-primary"
          />
          <Input
            placeholder="Search permissions..."
            className="w-72 border-none bg-background/50 pl-10 ring-1 ring-border focus-visible:ring-2 focus-visible:ring-primary shadow-sm"
            onChange={(e) => setSearch(e.target.value)}
          />
        </div>

        {/* Filter for is_admin_only will go here */}
        <Button
          onClick={() => setIsCreateDialogOpen(true)}
          size="sm"
          className="rounded-xl"
        >
          <HugeiconsIcon icon={PlusSignIcon} className="mr-2 size-4" />
          New Permission
        </Button>
      </div>

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
          sorting={[]}
          onSortingChange={setSorting}
        />
      </div>

      <CreatePermissionDialog
        open={isCreateDialogOpen}
        onOpenChange={setIsCreateDialogOpen}
      />

      {editPermission && (
        <EditPermissionDialog
          open={!!editPermission}
          onOpenChange={() => setEditPermission(null)}
          permission={editPermission}
        />
      )}
    </div>
  )
}
