import { UserBulkEditForm } from './user-bulk-edit-form'
import type { BulkUpdateValues } from '../schemas'
import {
  Dialog,
  DialogContent,
  DialogDescription,
  DialogHeader,
  DialogTitle,
} from '@/components/ui/dialog'

interface UserBulkEditDialogProps {
  open: boolean
  onOpenChange: (open: boolean) => void
  onConfirm: (data: BulkUpdateValues) => void
  selectedCount: number
  isSubmitting?: boolean
}

export function UserBulkEditDialog({
  open,
  onOpenChange,
  onConfirm,
  selectedCount,
  isSubmitting,
}: UserBulkEditDialogProps) {
  return (
    <Dialog open={open} onOpenChange={onOpenChange}>
      <DialogContent className="sm:max-w-[600px]">
        <DialogHeader>
          <DialogTitle>Bulk Edit Users</DialogTitle>
          <DialogDescription>
            Updating {selectedCount} selected user identities. Fields left
            unchanged will remain as they are.
          </DialogDescription>
        </DialogHeader>

        <UserBulkEditForm
          onConfirm={onConfirm}
          onOpenChange={onOpenChange}
          isSubmitting={isSubmitting}
        />
      </DialogContent>
    </Dialog>
  )
}
