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
import { HugeiconsIcon } from '@hugeicons/react'
import { Delete02Icon, UserGroupIcon } from '@hugeicons/core-free-icons'
import { UserBulkEditDialog } from './user-bulk-edit-dialog'
import { UserEditDialog } from './user-edit-dialog'
import { BulkUpdateValues, UpdateUserValues } from '../schemas'
import type { User } from '../types'

interface UserModalsProps {
  userToDelete: string | null
  setUserToDelete: (id: string | null) => void
  onDeleteConfirm: (id: string) => void
  isBulkDeleteOpen: boolean
  setIsBulkDeleteOpen: (open: boolean) => void
  onBulkDeleteConfirm: () => void
  isBulkEditOpen: boolean
  setIsBulkEditOpen: (open: boolean) => void
  onBulkEditConfirm: (data: BulkUpdateValues) => void
  selectedCount: number
  isBulkUpdating?: boolean
  userToEdit: User | null
  setUserToEdit: (user: User | null) => void
  onEditConfirm: (data: UpdateUserValues) => void
  isUpdating?: boolean
}

export function UserModals({
  userToDelete,
  setUserToDelete,
  onDeleteConfirm,
  isBulkDeleteOpen,
  setIsBulkDeleteOpen,
  onBulkDeleteConfirm,
  isBulkEditOpen,
  setIsBulkEditOpen,
  onBulkEditConfirm,
  selectedCount,
  isBulkUpdating,
  userToEdit,
  setUserToEdit,
  onEditConfirm,
  isUpdating,
}: UserModalsProps) {
  return (
    <>
      <AlertDialog
        open={!!userToDelete}
        onOpenChange={(open) => !open && setUserToDelete(null)}
      >
        <AlertDialogContent className="rounded-[2.5rem] border-none p-10 shadow-2xl backdrop-blur-3xl ring-1 ring-white/20">
          <AlertDialogHeader>
            <div className="mx-auto mb-4 flex size-20 items-center justify-center rounded-3xl bg-destructive/10 text-destructive">
              <HugeiconsIcon icon={Delete02Icon} className="size-10" />
            </div>
            <AlertDialogTitle className="text-center text-3xl font-black tracking-tight">
              Confirm Purge?
            </AlertDialogTitle>
            <AlertDialogDescription className="text-center text-base font-medium leading-relaxed opacity-70">
              This will permanently erase this user identity from the Skoola
              infrastructure. All session data, preferences, and associations
              will be lost.
            </AlertDialogDescription>
          </AlertDialogHeader>
          <AlertDialogFooter className="mt-10 sm:justify-center gap-3">
            <AlertDialogCancel className="h-14 min-w-[120px] rounded-2xl border-none bg-muted/50 font-black uppercase tracking-widest transition-colors hover:bg-muted">
              Abort
            </AlertDialogCancel>
            <AlertDialogAction
              onClick={() => userToDelete && onDeleteConfirm(userToDelete)}
              className="h-14 min-w-[160px] rounded-2xl bg-destructive font-black uppercase tracking-widest text-destructive-foreground shadow-2xl shadow-destructive/20 transition-all hover:bg-destructive/90 active:scale-95"
            >
              Execute Purge
            </AlertDialogAction>
          </AlertDialogFooter>
        </AlertDialogContent>
      </AlertDialog>

      <AlertDialog open={isBulkDeleteOpen} onOpenChange={setIsBulkDeleteOpen}>
        <AlertDialogContent className="max-w-md rounded-[2.5rem] border-none p-10 shadow-2xl backdrop-blur-3xl ring-1 ring-white/20">
          <AlertDialogHeader>
            <div className="mx-auto mb-4 flex size-20 items-center justify-center rounded-3xl bg-destructive/10 text-destructive">
              <HugeiconsIcon icon={UserGroupIcon} className="size-10" />
            </div>
            <AlertDialogTitle className="text-center text-3xl font-black tracking-tight">
              Mass Data Wipe
            </AlertDialogTitle>
            <AlertDialogDescription className="text-center text-base font-medium leading-relaxed text-destructive/80">
              FATAL ACTION: You are about to simultaneously destroy{' '}
              {selectedCount} user accounts. This procedure is irreversible and
              will remove all associated platform data.
            </AlertDialogDescription>
          </AlertDialogHeader>
          <AlertDialogFooter className="mt-10 sm:justify-center gap-3">
            <AlertDialogCancel className="h-14 min-w-[120px] rounded-2xl border-none bg-muted/50 font-black uppercase tracking-widest">
              Halt Action
            </AlertDialogCancel>
            <AlertDialogAction
              onClick={onBulkDeleteConfirm}
              className="h-14 min-w-[200px] rounded-2xl bg-destructive font-black uppercase tracking-widest text-destructive-foreground shadow-2xl shadow-destructive/30"
            >
              Confirm Mass Wipe
            </AlertDialogAction>
          </AlertDialogFooter>
        </AlertDialogContent>
      </AlertDialog>

      <UserBulkEditDialog
        open={isBulkEditOpen}
        onOpenChange={setIsBulkEditOpen}
        onConfirm={onBulkEditConfirm}
        selectedCount={selectedCount}
        isSubmitting={isBulkUpdating}
      />

      <UserEditDialog
        user={userToEdit}
        open={!!userToEdit}
        onOpenChange={(open) => !open && setUserToEdit(null)}
        onConfirm={onEditConfirm}
        isSubmitting={isUpdating}
      />
    </>
  )
}
