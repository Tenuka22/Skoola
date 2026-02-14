'use client'

import {
  AlertDialog,
  AlertDialogAction,
  AlertDialogCancel,
  AlertDialogContent,
  AlertDialogDescription,
  AlertDialogFooter,
  AlertDialogHeader,
  AlertDialogTitle,
} from '../../../components/ui/alert-dialog'

interface StudentBulkDeleteDialogProps {
  isBulkDeleteOpen: boolean
  setIsBulkDeleteOpen: (open: boolean) => void
  onBulkDeleteConfirm: () => void
  selectedCount: number
}

export function StudentBulkDeleteDialog({
  isBulkDeleteOpen,
  setIsBulkDeleteOpen,
  onBulkDeleteConfirm,
  selectedCount,
}: StudentBulkDeleteDialogProps) {
  return (
    <AlertDialog
      open={isBulkDeleteOpen}
      onOpenChange={(open) => !open && setIsBulkDeleteOpen(false)}
    >
      <AlertDialogContent>
        <AlertDialogHeader>
          <AlertDialogTitle>
            Remove {selectedCount} Student{selectedCount > 1 ? 's' : ''}?
          </AlertDialogTitle>
          <AlertDialogDescription>
            This will permanently remove the selected students from the
            institution records. This action cannot be undone.
          </AlertDialogDescription>
        </AlertDialogHeader>
        <AlertDialogFooter>
          <AlertDialogCancel>Abort</AlertDialogCancel>
          <AlertDialogAction onClick={onBulkDeleteConfirm}>
            Confirm Removal
          </AlertDialogAction>
        </AlertDialogFooter>
      </AlertDialogContent>
    </AlertDialog>
  )
}
