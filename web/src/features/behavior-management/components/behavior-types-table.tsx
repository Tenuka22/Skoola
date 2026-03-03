import { useQuery } from '@tanstack/react-query'
import { Delete02Icon, PencilEdit01Icon } from '@hugeicons/core-free-icons'
import { HugeiconsIcon } from '@hugeicons/react'
import * as React from 'react'
import {
  getAllBehaviorIncidentTypesQueryOptions,
  useDeleteBehaviorIncidentType,
} from '../api'
import type { BehaviorIncidentTypeResponse } from '@/lib/api/types.gen'
import type { DataTableColumnDef } from '@/components/data-table'
import { Button } from '@/components/ui/button'
import {
  AlertDialog,
  AlertDialogAction,
  AlertDialogCancel,
  AlertDialogContent,
  AlertDialogDescription,
  AlertDialogFooter,
  AlertDialogHeader,
  AlertDialogTitle,
} from '@/components/ui/alert-dialog'
import { DataTable, DataTableColumnHeader } from '@/components/data-table'

interface BehaviorTypesTableProps {
  setTypeToEdit: (type: BehaviorIncidentTypeResponse | null) => void
  onAdd?: () => void
  onAddLabel?: string
}

export function BehaviorTypesTable({
  setTypeToEdit,
  onAdd,
  onAddLabel,
}: BehaviorTypesTableProps) {
  const [typeToDelete, setTypeToDelete] = React.useState<string | null>(null)
  const [search, setSearch] = React.useState('')

  const { data: types, isLoading } = useQuery(
    getAllBehaviorIncidentTypesQueryOptions(),
  )

  const deleteMutation = useDeleteBehaviorIncidentType()

  const filteredData = React.useMemo(() => {
    if (!search) return types || []
    const s = search.toLowerCase()
    return (types || []).filter(
      (type) =>
        type.type_name.toLowerCase().includes(s) ||
        type.description?.toLowerCase().includes(s),
    )
  }, [types, search])

  const columns = React.useMemo<
    Array<DataTableColumnDef<BehaviorIncidentTypeResponse>>
  >(
    () => [
      {
        accessorKey: 'type_name',
        header: ({ column }) => (
          <DataTableColumnHeader column={column} title="Type Name" />
        ),
        cell: ({ row }) => (
          <span className="font-medium">{row.original.type_name}</span>
        ),
        meta: { isPinned: 'left' },
      },
      {
        accessorKey: 'description',
        header: ({ column }) => (
          <DataTableColumnHeader column={column} title="Description" />
        ),
        cell: ({ row }) => (
          <div
            className="max-w-[300px] truncate"
            title={row.original.description || ''}
          >
            {row.original.description || '-'}
          </div>
        ),
      },
      {
        accessorKey: 'default_points',
        header: ({ column }) => (
          <DataTableColumnHeader column={column} title="Default Points" />
        ),
      },
      {
        id: 'row-actions',
        header: 'Actions',
        cell: ({ row }) => (
          <div className="flex justify-end gap-2">
            <Button
              variant="ghost"
              size="icon"
              className="size-8"
              onClick={() => setTypeToEdit(row.original)}
            >
              <HugeiconsIcon icon={PencilEdit01Icon} className="size-4" />
            </Button>
            <Button
              variant="ghost"
              size="icon"
              className="size-8 text-destructive hover:text-destructive"
              onClick={() => setTypeToDelete(row.original.id)}
            >
              <HugeiconsIcon icon={Delete02Icon} className="size-4" />
            </Button>
          </div>
        ),
        meta: { align: 'end', isPinned: 'right' },
      },
    ],
    [setTypeToEdit],
  )

  return (
    <>
      <DataTable
        columns={columns}
        data={filteredData}
        isLoading={isLoading}
        pageIndex={0}
        pageSize={filteredData.length || 10}
        pageCount={1}
        canNextPage={false}
        canPreviousPage={false}
        fetchNextPage={() => {}}
        fetchPreviousPage={() => {}}
        enablePinning
        search={search}
        onSearchChange={setSearch}
        searchPlaceholder="Search incident types..."
        onAdd={onAdd}
        onAddLabel={onAddLabel}
      />

      <AlertDialog
        open={!!typeToDelete}
        onOpenChange={() => setTypeToDelete(null)}
      >
        <AlertDialogContent>
          <AlertDialogHeader>
            <AlertDialogTitle>Are you absolutely sure?</AlertDialogTitle>
            <AlertDialogDescription>
              This action cannot be undone. This will permanently delete the
              behavior incident type.
            </AlertDialogDescription>
          </AlertDialogHeader>
          <AlertDialogFooter>
            <AlertDialogCancel>Cancel</AlertDialogCancel>
            <AlertDialogAction
              onClick={() =>
                typeToDelete &&
                deleteMutation.mutate(
                  { path: { type_id: typeToDelete } },
                  {
                    onSuccess: () => {
                      setTypeToDelete(null)
                    },
                  },
                )
              }
            >
              Delete
            </AlertDialogAction>
          </AlertDialogFooter>
        </AlertDialogContent>
      </AlertDialog>
    </>
  )
}
