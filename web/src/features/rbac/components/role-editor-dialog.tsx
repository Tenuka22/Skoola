import * as React from 'react'
import { useQuery } from '@tanstack/react-query'
import { HugeiconsIcon } from '@hugeicons/react'
import { Cancel01Icon } from '@hugeicons/core-free-icons'
import { useRBACSearchParams } from '../search-params'
import { isPermissionEnum } from '../utils/permissions'
import {
  useAssignPermissionToRole,
  useUnassignPermissionFromRole,
} from '../api'
import { PermissionList } from './permission-list'
import type { PermissionEnum } from '@/lib/api/types.gen'
import { Button } from '@/components/ui/button'
import {
  Dialog,
  DialogContent,
  DialogDescription,
  DialogFooter,
  DialogHeader,
  DialogTitle,
} from '@/components/ui/dialog'
import { HStack } from '@/components/primitives'
import { getRolePermissionsOptions } from '@/lib/api/@tanstack/react-query.gen'
import { authClient } from '@/lib/clients'

export function RoleEditorDialog() {
  const { selectedRoleId, isRoleEditorOpen, setIsRoleEditorOpen } =
    useRBACSearchParams()

  const { data: rawPermissions } = useQuery({
    ...getRolePermissionsOptions({
      client: authClient,
      path: { role_id: selectedRoleId || '' },
    }),
    enabled: !!selectedRoleId,
  })

  const assignedPermissions = React.useMemo(() => {
    const perms = rawPermissions?.permissions || []
    return perms.filter(isPermissionEnum)
  }, [rawPermissions])

  const assignPerm = useAssignPermissionToRole()
  const unassignPerm = useUnassignPermissionFromRole()

  const handleTogglePermission = (
    permission: PermissionEnum,
    checked: boolean,
  ) => {
    if (!selectedRoleId) return

    if (checked) {
      assignPerm.mutate({
        path: { role_id: selectedRoleId },
        body: { permission },
      })
    } else {
      unassignPerm.mutate({
        path: { role_id: selectedRoleId },
        body: { permission },
      })
    }
  }

  return (
    <Dialog
      open={isRoleEditorOpen ?? false}
      onOpenChange={(open) => setIsRoleEditorOpen(open)}
    >
      <DialogContent className="max-w-2xl min-w-72 h-[85vh] overflow-y-auto">
        <DialogHeader>
          <DialogTitle>Configure Role : {selectedRoleId}</DialogTitle>
          <DialogDescription>
            Baseline permissions for all users assigned to the {selectedRoleId}{' '}
            role.
          </DialogDescription>
        </DialogHeader>

        <PermissionList
          assignedPermissions={assignedPermissions}
          onToggle={handleTogglePermission}
        />

        <DialogFooter className="p-4 border-t">
          <HStack justify="end" gap={2}>
            <Button
              variant="outline"
              onClick={() => setIsRoleEditorOpen(false)}
            >
              <HugeiconsIcon icon={Cancel01Icon} className="size-4 mr-2" />
              Close
            </Button>
          </HStack>
        </DialogFooter>
      </DialogContent>
    </Dialog>
  )
}
