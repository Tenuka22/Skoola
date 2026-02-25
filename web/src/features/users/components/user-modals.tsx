import { UserBulkEditDialog } from './user-bulk-edit-dialog'
import { UserEditDialog } from './user-edit-dialog'
import { UserLockDialog } from './user-lock-dialog'
import { UserPermissionsDialog } from './user-permissions-dialog'
import type { BulkUpdateValues, UpdateUserValues } from '../schemas'
import type { User } from '../types'
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
  userToLock: User | null
  setUserToLock: (user: User | null) => void
  onLockConfirm: (date: Date) => void
  isLocking?: boolean
  userToManagePermissions: User | null
  setUserToManagePermissions: (user: User | null) => void
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
  userToLock,
  setUserToLock,
  onLockConfirm,
  isLocking,
  userToManagePermissions,
  setUserToManagePermissions,
}: UserModalsProps) {
  return (
    <>
      <AlertDialog
        open={!!userToDelete}
        onOpenChange={(open) => !open && setUserToDelete(null)}
      >
        <AlertDialogContent>
          <AlertDialogHeader>
            <AlertDialogTitle>Are you absolutely sure?</AlertDialogTitle>
            <AlertDialogDescription>
              This action cannot be undone. This will permanently delete the
              user account and remove their data from our servers.
            </AlertDialogDescription>
          </AlertDialogHeader>
          <AlertDialogFooter>
            <AlertDialogCancel>Cancel</AlertDialogCancel>
            <AlertDialogAction
              onClick={() => userToDelete && onDeleteConfirm(userToDelete)}
            >
              Delete
            </AlertDialogAction>
          </AlertDialogFooter>
        </AlertDialogContent>
      </AlertDialog>

      <AlertDialog open={isBulkDeleteOpen} onOpenChange={setIsBulkDeleteOpen}>
        <AlertDialogContent>
          <AlertDialogHeader>
            <AlertDialogTitle>Are you absolutely sure?</AlertDialogTitle>
            <AlertDialogDescription>
              This action cannot be undone. This will permanently delete{' '}
              {selectedCount} user accounts and remove their data from our
              servers.
            </AlertDialogDescription>
          </AlertDialogHeader>
          <AlertDialogFooter>
            <AlertDialogCancel>Cancel</AlertDialogCancel>
            <AlertDialogAction onClick={onBulkDeleteConfirm}>
              Delete All
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

      <UserLockDialog
        user={userToLock}
        open={!!userToLock}
        onOpenChange={(open) => !open && setUserToLock(null)}
        onConfirm={onLockConfirm}
        isSubmitting={isLocking}
      />

      <UserPermissionsDialog
        user={userToManagePermissions}
        onClose={() => setUserToManagePermissions(null)}
      />
    </>
  )
}
