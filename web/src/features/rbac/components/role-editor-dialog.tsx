import * as React from 'react'
import { useMutation, useQuery, useQueryClient } from '@tanstack/react-query'
import { toast } from 'sonner'
import { HugeiconsIcon } from '@hugeicons/react'
import {
  Cancel01Icon,
  Tick01Icon,
  Shield01Icon,
} from '@hugeicons/core-free-icons'
import { useRBACStore } from '../store'
import { rbacApi } from '../api'
import { isPermissionEnum } from '../utils/permissions'
import { PermissionList } from './permission-list'
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

export function RoleEditorDialog() {
  const { selectedRoleId, isRoleEditorOpen, setIsRoleEditorOpen } =
    useRBACStore()
  const queryClient = useQueryClient()

  const { data: rawPermissions = '' } = useQuery({
    ...rbacApi.getRolePermissionsOptions(selectedRoleId || ''),
    enabled: !!selectedRoleId,
  })

  const assignedPermissions = React.useMemo(
    () =>
      typeof rawPermissions === 'string' && rawPermissions
        ? rawPermissions.split(',').filter(isPermissionEnum)
        : [],
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
    onError: (err) => {
      toast.error(err instanceof Error ? err.message : 'Failed to assign permission')
    },
  })

  const unassignPerm = useMutation({
    ...rbacApi.unassignPermissionFromRoleMutation(),
    onSuccess: () => {
      queryClient.invalidateQueries({
        queryKey: ['getRolePermissions', { role_id: selectedRoleId }],
      })
      toast.success('Permission removed from role')
    },
    onError: (err) => {
      toast.error(err instanceof Error ? err.message : 'Failed to remove permission')
    },
  })

  const handleTogglePermission = (permission: PermissionEnum, checked: boolean) => {
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
    <Dialog open={isRoleEditorOpen} onOpenChange={setIsRoleEditorOpen}>
      <DialogContent className="max-w-2xl h-[85vh] flex flex-col p-0 gap-0 overflow-hidden border-none">
        <DialogHeader className="p-4 pb-3">
          <div className="flex items-center gap-4">
            <div className="size-10 flex items-center justify-center">
              <HugeiconsIcon icon={Shield01Icon} className="size-6 text-primary" />
            </div>
            <div className="flex flex-col gap-0.5">
              <DialogTitle className="text-2xl font-bold tracking-tight flex items-center gap-2">
                Configure Role: <span className="text-primary">{selectedRoleId}</span>
              </DialogTitle>
              <p className="text-sm text-muted-foreground">
                Baseline permissions for all users assigned to the {selectedRoleId} role.
              </p>
            </div>
          </div>
        </DialogHeader>

        <div className="flex-1 min-h-0 overflow-hidden p-4">
          <div className="flex items-center justify-between mb-4">
            <h3 className="font-bold text-[13px] uppercase tracking-wider text-foreground/60 flex items-center gap-2">
              Permissions Management
            </h3>
            <Badge variant="secondary" className="font-mono">
              {assignedPermissions.length} ASSIGNED
            </Badge>
          </div>
          
          <div className="h-full pb-8">
            <PermissionList 
              assignedPermissions={assignedPermissions}
              onToggle={handleTogglePermission}
            />
          </div>
        </div>

        <DialogFooter className="p-4">
          <Button 
            variant="outline" 
            className="px-6 h-11"
            onClick={() => setIsRoleEditorOpen(false)}
          >
            <HugeiconsIcon icon={Cancel01Icon} className="size-4 mr-2" />
            Cancel
          </Button>
          <Button 
            className="px-8 h-11 font-semibold"
            onClick={() => setIsRoleEditorOpen(false)}
          >
            <HugeiconsIcon icon={Tick01Icon} className="size-4 mr-2" />
            Save Configuration
          </Button>
        </DialogFooter>
      </DialogContent>
    </Dialog>
  )
}
