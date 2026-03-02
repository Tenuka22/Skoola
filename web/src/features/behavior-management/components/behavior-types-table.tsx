import { useMutation, useQuery, useQueryClient } from '@tanstack/react-query'
import { Delete02Icon, PencilEdit01Icon } from '@hugeicons/core-free-icons'
import { HugeiconsIcon } from '@hugeicons/react'
import { toast } from 'sonner'
import { useBehaviorStore } from '../store'
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
  deleteBehaviorIncidentTypeMutation,
  getAllBehaviorIncidentTypesOptions,
} from '@/lib/api/@tanstack/react-query.gen'
import { authClient } from '@/lib/clients'
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

export function BehaviorTypesTable() {
  const store = useBehaviorStore()
  const queryClient = useQueryClient()

  const { data: types, isLoading } = useQuery({
    ...getAllBehaviorIncidentTypesOptions({ client: authClient }),
  })

  const deleteMutation = useMutation({
    ...deleteBehaviorIncidentTypeMutation({ client: authClient }),
    onSuccess: () => {
      toast.success('Behavior type deleted successfully.')
      queryClient.invalidateQueries({
        queryKey: ['getAllBehaviorIncidentTypes'],
      })
      store.setTypeToDelete(null)
    },
    onError: (error) => {
      toast.error(
        `Failed to delete behavior type: ${error.message || 'Unknown error'}`,
      )
    },
  })

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
                      onClick={() => store.setTypeToEdit(type)}
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
                      onClick={() => store.setTypeToDelete(type.id)}
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
        open={!!store.typeToDelete}
        onOpenChange={() => store.setTypeToDelete(null)}
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
                store.typeToDelete &&
                deleteMutation.mutate({ path: { type_id: store.typeToDelete } })
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
