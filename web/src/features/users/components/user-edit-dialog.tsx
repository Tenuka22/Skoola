import { UserEdit01Icon } from '@hugeicons/core-free-icons'
import { HugeiconsIcon } from '@hugeicons/react'

import { UserEditForm } from './user-edit-form'
import type { UpdateUserValues } from '../schemas'
import type { User } from '../types'
import {
  Dialog,
  DialogContent,
  DialogDescription,
  DialogTitle,
} from '@/components/ui/dialog'
import { Stack } from '@/components/primitives'

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
      <DialogContent className="p-0 overflow-hidden sm:max-w-[600px] border-border/60 shadow-xl">
        <div className="flex flex-col border-b border-border/40 bg-muted/20 p-6 pb-6">
          <div className="flex gap-4 items-start">
            <div className="flex h-10 w-10 shrink-0 items-center justify-center rounded-full bg-primary/10 ring-1 ring-primary/20">
              <HugeiconsIcon icon={UserEdit01Icon} className="size-5 text-primary" />
            </div>
            <Stack gap={1} className="pt-1">
              <DialogTitle className="text-xl">Edit User Account</DialogTitle>
              <DialogDescription className="text-sm">
                Modify identity parameters for {user?.email}. Changes are applied
                immediately.
              </DialogDescription>
            </Stack>
          </div>
        </div>

        <div className="px-6 pb-6 pt-4">
          {user && (
            <UserEditForm
              user={user}
              onConfirm={onConfirm}
              onOpenChange={onOpenChange}
              isSubmitting={isSubmitting}
            />
          )}
        </div>
      </DialogContent>
    </Dialog>
  )
}
