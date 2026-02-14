import { usePermissionsStore } from '../store'
import { CreatePermissionDialog } from './create-permission-dialog'
import { EditPermissionDialog } from './edit-permission-dialog'
import { CreatePermissionSetDialog } from './create-permission-set-dialog'
import { EditPermissionSetDialog } from './edit-permission-set-dialog'
import { ManagePermissionSetPermissionsDialog } from './manage-permission-set-permissions-dialog'
import type { Permission } from '@/lib/api/types.gen'
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

interface PermissionModalsProps {
  onPermissionDeleteConfirm: (id: number) => void
  onPermissionSetDeleteConfirm: (id: string) => void
  allPermissions: Array<Permission>
}

export function PermissionModals({
  onPermissionDeleteConfirm,
  onPermissionSetDeleteConfirm,
  allPermissions,
}: PermissionModalsProps) {
  const {
    permissionToDelete,
    setPermissionToDelete,
    permissionToEdit,
    setPermissionToEdit,
    isCreatePermissionOpen,
    setIsCreatePermissionOpen,
    permissionSetToDelete,
    setPermissionSetToDelete,
    permissionSetToEdit,
    setPermissionSetToEdit,
    isCreatePermissionSetOpen,
    setIsCreatePermissionSetOpen,
    permissionSetToManage,
  } = usePermissionsStore()

  return (
    <>
      {/* Permission Delete */}
      <AlertDialog
        open={!!permissionToDelete}
        onOpenChange={(open) => !open && setPermissionToDelete(null)}
      >
        <AlertDialogContent>
          <AlertDialogHeader>
            <AlertDialogTitle>Delete Permission?</AlertDialogTitle>
            <AlertDialogDescription>
              This action cannot be undone. This will permanently delete the
              permission from the system.
            </AlertDialogDescription>
          </AlertDialogHeader>
          <AlertDialogFooter>
            <AlertDialogCancel>Cancel</AlertDialogCancel>
            <AlertDialogAction
              onClick={() =>
                permissionToDelete &&
                onPermissionDeleteConfirm(permissionToDelete)
              }
            >
              Delete
            </AlertDialogAction>
          </AlertDialogFooter>
        </AlertDialogContent>
      </AlertDialog>

      {/* Permission Set Delete */}
      <AlertDialog
        open={!!permissionSetToDelete}
        onOpenChange={(open) => !open && setPermissionSetToDelete(null)}
      >
        <AlertDialogContent>
          <AlertDialogHeader>
            <AlertDialogTitle>Delete Permission Set?</AlertDialogTitle>
            <AlertDialogDescription>
              This action cannot be undone. This will permanently delete the
              permission set from the system.
            </AlertDialogDescription>
          </AlertDialogHeader>
          <AlertDialogFooter>
            <AlertDialogCancel>Cancel</AlertDialogCancel>
            <AlertDialogAction
              onClick={() =>
                permissionSetToDelete &&
                onPermissionSetDeleteConfirm(permissionSetToDelete)
              }
            >
              Delete
            </AlertDialogAction>
          </AlertDialogFooter>
        </AlertDialogContent>
      </AlertDialog>

      <CreatePermissionDialog
        open={isCreatePermissionOpen}
        onOpenChange={setIsCreatePermissionOpen}
      />

      {permissionToEdit && (
        <EditPermissionDialog
          open={!!permissionToEdit}
          onOpenChange={() => setPermissionToEdit(null)}
          permission={permissionToEdit}
        />
      )}

      <CreatePermissionSetDialog
        open={isCreatePermissionSetOpen}
        onOpenChange={setIsCreatePermissionSetOpen}
      />

      {permissionSetToEdit && (
        <EditPermissionSetDialog
          open={!!permissionSetToEdit}
          onOpenChange={() => setPermissionSetToEdit(null)}
          permissionSet={permissionSetToEdit}
        />
      )}

      {permissionSetToManage && (
        <ManagePermissionSetPermissionsDialog
          open={!!permissionSetToManage}
          onOpenChange={() => setManagePermissionsForSet(null)}
          permissionSet={permissionSetToManage}
          allPermissions={allPermissions}
        />
      )}
    </>
  )
}

// Helper to bridge naming difference
function setManagePermissionsForSet(val: any) {
  const store = usePermissionsStore.getState()
  store.setPermissionSetToManage(val)
}
