import * as React from 'react'
import { useMutation, useQuery, useQueryClient } from '@tanstack/react-query'
import { toast } from 'sonner'
import { HugeiconsIcon } from '@hugeicons/react'
import {
  Alert01Icon,
  Cancel01Icon,
  Delete02Icon,
  Tick01Icon,
} from '@hugeicons/core-free-icons'
import { useRBACStore } from '../store'
import { rbacApi } from '../api'
import { PermissionPalette } from './permission-palette'
import type { PermissionEnum } from '@/lib/api/types.gen'
import { Badge } from '@/components/ui/badge'
import { Button } from '@/components/ui/button'
import {
  Dialog,
  DialogContent,
  DialogFooter,
  DialogHeader,
  DialogTitle,
} from '@/components/ui/dialog'
import { ScrollArea } from '@/components/ui/scroll-area'

export function RoleEditorDialog() {
  const { selectedRoleId, isRoleEditorOpen, setIsRoleEditorOpen } =
    useRBACStore()
  const queryClient = useQueryClient()

  const { data: rawPermissions = [] } = useQuery({
    ...rbacApi.getRolePermissionsOptions(selectedRoleId as string),
    enabled: !!selectedRoleId,
  })

  const assignedPermissions = React.useMemo(
    () =>
      Array.isArray(rawPermissions) ? (rawPermissions as Array<PermissionEnum>) : [],
    [rawPermissions],
  )

  const assignPerm = useMutation({
    ...rbacApi.assignPermissionToRoleMutation(),
    onSuccess: () => {
      queryClient.invalidateQueries({
        queryKey: ['getRolePermissions', { role_id: selectedRoleId }],
      })
      toast.success('Permission assigned to role')
    },
    onError: (err) => toast.error(err.message),
  })

  const unassignPerm = useMutation({
    ...rbacApi.unassignPermissionFromRoleMutation(),
    onSuccess: () => {
      queryClient.invalidateQueries({
        queryKey: ['getRolePermissions', { role_id: selectedRoleId }],
      })
      toast.success('Permission removed from role')
    },
    onError: (err) => toast.error(err.message),
  })

  const handleDrop = (e: React.DragEvent) => {
    e.preventDefault()
    const permission = e.dataTransfer.getData('permission') as PermissionEnum
    if (
      selectedRoleId &&
      permission &&
      !assignedPermissions.includes(permission)
    ) {
      assignPerm.mutate({
        path: { role_id: selectedRoleId as any },
        body: { permission },
      })
    }
  }

  const handleDragOver = (e: React.DragEvent) => {
    e.preventDefault()
  }

  return (
    <Dialog open={isRoleEditorOpen} onOpenChange={setIsRoleEditorOpen}>
      <DialogContent className="max-w-4xl h-[80vh] flex flex-col p-0 gap-0 overflow-hidden">
        <DialogHeader className="p-6 pb-4 border-b">
          <DialogTitle className="text-2xl flex items-center gap-2">
            Editing Role: <span className="text-primary">{selectedRoleId}</span>
          </DialogTitle>
          <p className="text-sm text-muted-foreground mt-1">
            Permissions assigned to this role apply to all users with this role.
          </p>
        </DialogHeader>

        <div className="flex-1 flex min-h-0 overflow-hidden">
          {/* Assigned Permissions */}
          <div
            className="w-1/2 flex flex-col p-6 gap-4 border-r overflow-hidden"
            onDrop={handleDrop}
            onDragOver={handleDragOver}
          >
            <div className="flex items-center justify-between">
              <h3 className="font-semibold text-sm flex items-center gap-2">
                Assigned Permissions
                <Badge variant="secondary">{assignedPermissions.length}</Badge>
              </h3>
            </div>

            <ScrollArea className="flex-1 border rounded-lg bg-muted/5 p-4">
              {assignedPermissions.length === 0 ? (
                <div className="flex flex-col items-center justify-center h-40 text-muted-foreground">
                  <HugeiconsIcon
                    icon={Alert01Icon}
                    className="size-8 mb-2 opacity-20"
                  />
                  <p className="text-sm">No permissions assigned</p>
                  <p className="text-xs">Drag from the right to assign</p>
                </div>
              ) : (
                <div className="flex flex-wrap gap-2">
                  {assignedPermissions.map((perm) => (
                    <Badge
                      key={perm}
                      className="flex items-center gap-1 pl-2 pr-1 py-1"
                    >
                      {perm}
                      <Button
                        variant="ghost"
                        size="icon"
                        className="size-4 p-0 h-4 w-4 hover:bg-destructive/20 hover:text-destructive"
                        onClick={() =>
                          unassignPerm.mutate({
                            path: { role_id: selectedRoleId as any },
                            body: { permission: perm },
                          })
                        }
                      >
                        <HugeiconsIcon icon={Delete02Icon} className="size-3" />
                      </Button>
                    </Badge>
                  ))}
                </div>
              )}
            </ScrollArea>
          </div>

          {/* Palette */}
          <div className="w-1/2 flex flex-col p-6 gap-4 bg-muted/5 overflow-hidden">
            <h3 className="font-semibold text-sm">Available Permissions</h3>
            <PermissionPalette />
          </div>
        </div>

        <DialogFooter className="p-4 border-t bg-muted/5">
          <Button variant="outline" onClick={() => setIsRoleEditorOpen(false)}>
            <HugeiconsIcon icon={Cancel01Icon} className="size-4 mr-2" />
            Close
          </Button>
          <Button onClick={() => setIsRoleEditorOpen(false)}>
            <HugeiconsIcon icon={Tick01Icon} className="size-4 mr-2" />
            Done
          </Button>
        </DialogFooter>
      </DialogContent>
    </Dialog>
  )
}
