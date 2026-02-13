import { HugeiconsIcon } from '@hugeicons/react'
import { PencilEdit01Icon } from '@hugeicons/core-free-icons'
import type { BulkUpdateValues } from '../schemas'
import {
  Dialog,
  DialogContent,
  DialogDescription,
  DialogHeader,
  DialogTitle,
} from '@/components/ui/dialog'
import { UserBulkEditForm } from './user-bulk-edit-form'

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
      <DialogContent className="max-w-2xl rounded-[2.5rem] border-none p-10 shadow-2xl backdrop-blur-3xl ring-1 ring-white/20">
        <DialogHeader>
          <div className="mx-auto mb-4 flex size-20 items-center justify-center rounded-3xl bg-primary/10 text-primary">
            <HugeiconsIcon icon={PencilEdit01Icon} className="size-10" />
          </div>
          <DialogTitle className="text-center text-3xl font-black tracking-tight">
            Bulk Edit Users
          </DialogTitle>
          <DialogDescription className="text-center text-base font-medium leading-relaxed opacity-70">
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
