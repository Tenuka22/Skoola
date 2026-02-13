import type { UpdateUserValues } from '../schemas'
import type { User } from '../types'
import {
  Dialog,
  DialogContent,
  DialogDescription,
  DialogHeader,
  DialogTitle,
} from '@/components/ui/dialog'
import { UserEditForm } from './user-edit-form'

interface UserEditDialogProps {
  user: User | null
  open: boolean
  onOpenChange: (open: boolean) => void
  onConfirm: (data: UpdateUserValues) => void
  isSubmitting?: boolean
}

export function UserEditDialog({
  user,
  open,
  onOpenChange,
  onConfirm,
  isSubmitting,
}: UserEditDialogProps) {
  return (
    <Dialog open={open} onOpenChange={onOpenChange}>
      <DialogContent className="sm:max-w-[600px]">
        <DialogHeader>
          <DialogTitle>Edit User Account</DialogTitle>
          <DialogDescription>
            Modify identity parameters for {user?.email}. Changes are applied
            immediately.
          </DialogDescription>
        </DialogHeader>

        {user && (
          <UserEditForm
            user={user}
            onConfirm={onConfirm}
            onOpenChange={onOpenChange}
            isSubmitting={isSubmitting}
          />
        )}
      </DialogContent>
    </Dialog>
  )
}
