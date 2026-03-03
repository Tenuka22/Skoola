import { useQuery } from '@tanstack/react-query'
import { Delete02Icon, PencilEdit01Icon } from '@hugeicons/core-free-icons'
import { HugeiconsIcon } from '@hugeicons/react'
import * as React from 'react'
import {
  getAllBehaviorIncidentTypesQueryOptions,
  useDeleteBehaviorIncidentType,
} from '../api'
import type { BehaviorIncidentTypeResponse } from '@/lib/api/types.gen'
import {
  Table,
  TableBody,
  TableCell,
  TableHead,
  TableHeader,
  TableRow,
} from '@/components/ui/table'
import { Button } from '@/components/ui/button'
import { Spinner } from '@/components/ui/spinner'
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

interface BehaviorTypesTableProps {
  setTypeToEdit: (type: BehaviorIncidentTypeResponse | null) => void
}

export function BehaviorTypesTable({ setTypeToEdit }: BehaviorTypesTableProps) {
  const [typeToDelete, setTypeToDelete] = React.useState<string | null>(null)

  const { data: types, isLoading } = useQuery(
    getAllBehaviorIncidentTypesQueryOptions(),
  )

  const deleteMutation = useDeleteBehaviorIncidentType()

  if (isLoading) {
    return (
      <div className="flex h-64 items-center justify-center">
        <Spinner />
      </div>
    )
  }

  return (
    <div className="rounded-md border">
      <Table>
        <TableHeader>
          <TableRow>
            <TableHead>Type Name</TableHead>
            <TableHead>Description</TableHead>
            <TableHead>Default Points</TableHead>
            <TableHead className="w-[100px] text-right">Actions</TableHead>
          </TableRow>
        </TableHeader>
        <TableBody>
          {!types || types.length === 0 ? (
            <TableRow>
              <TableCell colSpan={4} className="h-24 text-center">
                No behavior types found.
              </TableCell>
            </TableRow>
          ) : (
            types.map((type) => (
              <TableRow key={type.id}>
                <TableCell className="font-medium">{type.type_name}</TableCell>
                <TableCell className="max-w-[300px] truncate">
                  {type.description || '-'}
                </TableCell>
                <TableCell>{type.default_points}</TableCell>
                <TableCell className="text-right">
                  <div className="flex justify-end gap-2">
                    <Button
                      variant="ghost"
                      size="icon"
                      className="size-8"
                      onClick={() => setTypeToEdit(type)}
                    >
                      <HugeiconsIcon
                        icon={PencilEdit01Icon}
                        className="size-4"
                      />
                    </Button>
                    <Button
                      variant="ghost"
                      size="icon"
                      className="size-8 text-destructive hover:text-destructive"
                      onClick={() => setTypeToDelete(type.id)}
                    >
                      <HugeiconsIcon icon={Delete02Icon} className="size-4" />
                    </Button>
                  </div>
                </TableCell>
              </TableRow>
            ))
          )}
        </TableBody>
      </Table>

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
    </div>
  )
}
