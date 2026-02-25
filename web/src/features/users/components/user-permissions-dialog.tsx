'use client'

import * as React from 'react'
import { useMutation, useQuery } from '@tanstack/react-query'
import { toast } from 'sonner'
import type {
  PermissionEnum,
  UserResponse,
} from '@/lib/api/types.gen'
import {
  Dialog,
  DialogContent,
  DialogDescription,
  DialogFooter,
  DialogHeader,
  DialogTitle,
} from '@/components/ui/dialog'
import { Button } from '@/components/ui/button'
import { authClient } from '@/lib/clients'
import {
  assignPermissionToUserMutation,
  getUserPermissionsOptions,
  unassignPermissionFromUserMutation,
} from '@/lib/api/@tanstack/react-query.gen'
import { zPermissionEnum } from '@/lib/api/zod.gen'
import { Checkbox } from '@/components/ui/checkbox'
import { ScrollArea } from '@/components/ui/scroll-area'
import { Spinner } from '@/components/ui/spinner'
import { Badge } from '@/components/ui/badge'

interface UserPermissionsDialogProps {
  user: UserResponse | null
  onClose: () => void
}

export function UserPermissionsDialog({
  user,
  onClose,
}: UserPermissionsDialogProps) {
  const [selectedPermissions, setSelectedPermissions] = React.useState<
    Array<PermissionEnum>
  >([])
  const [initialPermissions, setInitialPermissions] = React.useState<
    Array<PermissionEnum>
  >([])

  const { data: userPermissions, isLoading } = useQuery({
    ...getUserPermissionsOptions({
      client: authClient,
      path: {
        user_id: user?.id ?? '',
      },
    }),
    enabled: !!user,
  })

  React.useEffect(() => {
    if (userPermissions) {
      const perms = userPermissions as unknown as Array<PermissionEnum>
      setSelectedPermissions(perms)
      setInitialPermissions(perms)
    }
  }, [userPermissions])

  const assignPermission = useMutation({
    ...assignPermissionToUserMutation({
      client: authClient,
    }),
  })

  const unassignPermission = useMutation({
    ...unassignPermissionFromUserMutation({
      client: authClient,
    }),
  })

  const handleSave = async () => {
    if (!user) return

    const permissionsToAssign = selectedPermissions.filter(
      (p) => !initialPermissions.includes(p),
    )
    const permissionsToUnassign = initialPermissions.filter(
      (p) => !selectedPermissions.includes(p),
    )

    try {
      await Promise.all([
        ...permissionsToAssign.map((permission) =>
          assignPermission.mutateAsync({
            path: { user_id: user.id, permission } as any,
            body: { permission },
          }),
        ),
        ...permissionsToUnassign.map((permission) =>
          unassignPermission.mutateAsync({
            path: { user_id: user.id, permission } as any,
            body: { permission },
          }),
        ),
      ])
      toast.success('Permissions updated successfully.')
      onClose()
    } catch (error) {
      toast.error('Failed to update permissions.')
    }
  }

  const allPermissions = zPermissionEnum.options

  return (
    <Dialog open={!!user} onOpenChange={(open) => !open && onClose()}>
      <DialogContent>
        <DialogHeader>
          <DialogTitle>Manage Permissions</DialogTitle>
          <DialogDescription>
            Assign or unassign direct permissions for {user?.email}.
          </DialogDescription>
        </DialogHeader>

        {isLoading ? (
          <div className="flex items-center justify-center p-8">
            <Spinner />
          </div>
        ) : (
          <>
            <div className="flex items-center gap-2">
              <p className="text-sm font-medium">
                Current Role:
              </p>
              <Badge variant="secondary">{user?.role}</Badge>
            </div>
            <ScrollArea className="h-72">
              <div className="grid grid-cols-2 gap-4 p-1">
                {allPermissions.map((permission) => (
                  <div key={permission} className="flex items-center gap-2">
                    <Checkbox
                      id={permission}
                      checked={selectedPermissions.includes(permission as PermissionEnum)}
                      onCheckedChange={(checked) => {
                        setSelectedPermissions((prev) =>
                          checked
                            ? [...prev, permission as PermissionEnum]
                            : prev.filter((p) => p !== permission),
                        )
                      }}
                    />
                    <label htmlFor={permission} className="text-sm">
                      {permission}
                    </label>
                  </div>
                ))}
              </div>
            </ScrollArea>
          </>
        )}

        <DialogFooter>
          <Button variant="ghost" onClick={onClose}>
            Cancel
          </Button>
          <Button
            onClick={handleSave}
            disabled={
              assignPermission.isPending || unassignPermission.isPending
            }
          >
            {assignPermission.isPending || unassignPermission.isPending ? (
              <Spinner />
            ) : (
              'Save'
            )}
          </Button>
        </DialogFooter>
      </DialogContent>
    </Dialog>
  )
}
