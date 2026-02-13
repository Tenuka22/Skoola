import { HugeiconsIcon } from '@hugeicons/react'
import { PencilEdit01Icon } from '@hugeicons/core-free-icons'
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
      <DialogContent className="max-w-2xl rounded-[2.5rem] border-none p-10 shadow-2xl backdrop-blur-3xl ring-1 ring-white/20">
        <DialogHeader>
          <div className="mx-auto mb-4 flex size-20 items-center justify-center rounded-3xl bg-primary/10 text-primary">
            <HugeiconsIcon icon={PencilEdit01Icon} className="size-10" />
          </div>
          <DialogTitle className="text-center text-3xl font-black tracking-tight">
            Edit User Account
          </DialogTitle>
          <DialogDescription className="text-center text-base font-medium leading-relaxed opacity-70">
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
