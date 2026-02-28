'use client'

import * as React from 'react'
import { useMutation, useQuery } from '@tanstack/react-query'
import { toast } from 'sonner'
import type { PermissionEnum, UserResponse } from '@/lib/api/types.gen'
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
import { Spinner } from '@/components/ui/spinner'
import { Badge } from '@/components/ui/badge'
import { HStack } from '@/components/primitives'

interface UserPermissionsDialogProps {
  user: UserResponse | null
  onClose: () => void
}

/** Humanize a permission enum like "UserCreate" -> "Create" */
function humanizeAction(permission: string, prefix: string): string {
  const action = permission.replace(prefix, '')
  // Split camelCase into words
  return action.replace(/([A-Z])/g, ' $1').trim()
}

/** Group permissions by their resource prefix */
function groupPermissions(
  permissions: ReadonlyArray<PermissionEnum>,
): Map<string, Array<{ key: PermissionEnum; action: string }>> {
  const groups = new Map<string, Array<{ key: PermissionEnum; action: string }>>()

  for (const perm of permissions) {
    // Find the prefix (e.g., "User", "Role", "Permission", "Staff", "PermissionSet")
    const match = perm.match(
      /^(PermissionSet|Permission|User|Role|Staff|Student|Class|Grade|Subject|Exam|Fee|Budget|Library|Attendance|Report|Setting|Notification|Transport|Hostel|Sport|Club|Cultural|Competition|Activity|Message|Announcement|Timetable|Resource|Payroll|Leave|Asset|Behavior|AuditLog|Academic|Dashboard|Conversation|Analytics|Scholarship|SystemConfig|SchoolInfo|Health|Maintenance|Visitor)/,
    )

    const prefix = match ? match[1] : 'Other'
    const action = match ? humanizeAction(perm, prefix) : perm

    if (!groups.has(prefix)) {
      groups.set(prefix, [])
    }
    groups.get(prefix)?.push({ key: perm, action })
  }

  return groups
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
    if (userPermissions && userPermissions.permissions) {
      const perms = userPermissions.permissions.flatMap((permission) => {
        const parsed = zPermissionEnum.safeParse(permission)
        return parsed.success ? [parsed.data] : []
      })
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
            path: { user_id: user.id },
            body: { permission },
          }),
        ),
        ...permissionsToUnassign.map((permission) =>
          unassignPermission.mutateAsync({
            path: { user_id: user.id },
            body: { permission },
          }),
        ),
      ])
      toast.success('Permissions updated successfully.')
      onClose()
    } catch (_error) {
      toast.error('Failed to update permissions.')
    }
  }

  const allPermissions = zPermissionEnum.options
  const grouped = React.useMemo(() => groupPermissions(allPermissions), [allPermissions])

  const changedCount =
    selectedPermissions.filter((p) => !initialPermissions.includes(p)).length +
    initialPermissions.filter((p) => !selectedPermissions.includes(p)).length

  const handleToggleGroup = (groupPerms: Array<PermissionEnum>) => {
    const allSelected = groupPerms.every((p) => selectedPermissions.includes(p))
    if (allSelected) {
      setSelectedPermissions((prev) => prev.filter((p) => !groupPerms.includes(p)))
    } else {
      setSelectedPermissions((prev) => [
        ...prev,
        ...groupPerms.filter((p) => !prev.includes(p)),
      ])
    }
  }

  return (
    <Dialog open={!!user} onOpenChange={(open) => !open && onClose()}>
<<<<<<< HEAD
      <DialogContent className="max-h-[85vh] w-full h-full overflow-y-auto">
=======
      <DialogContent className="sm:max-w-[640px]">
>>>>>>> 2780fcf (feat: Refactor theme management to use `next-themes` and update user views from board to grid, alongside various UI and API enhancements.)
        <DialogHeader>
          <DialogTitle>Manage Permissions</DialogTitle>
          <DialogDescription>
            Configure direct permissions for{' '}
            <span className="font-medium text-foreground">{user?.email}</span>
          </DialogDescription>
        </DialogHeader>

        {isLoading ? (
          <div className="flex items-center justify-center p-12">
            <Spinner />
          </div>
        ) : (
          <>
            {/* User meta bar */}
            <HStack gap={3} align="center" className="bg-muted/30 border border-border/50 rounded-lg px-4 py-3">
              <div className="flex items-center gap-2 flex-1">
                <span className="text-xs font-medium text-muted-foreground uppercase tracking-wider">Role</span>
                <Badge variant="secondary" className="font-medium">
                  {user?.role}
                </Badge>
              </div>
              <div className="flex items-center gap-2">
                <span className="text-xs font-medium text-muted-foreground uppercase tracking-wider">Selected</span>
                <Badge variant="outline" className="font-mono tabular-nums">
                  {selectedPermissions.length}/{allPermissions.length}
                </Badge>
              </div>
            </HStack>

            {/* Permissions grid grouped by category */}
            <ScrollArea className="h-[420px] w-full rounded-lg border">
              <div className="p-3 space-y-1">
                {Array.from(grouped.entries()).map(([group, permissions]) => {
                  const groupKeys = permissions.map((p) => p.key)
                  const selectedInGroup = groupKeys.filter((k) =>
                    selectedPermissions.includes(k),
                  ).length
                  const allInGroupSelected = selectedInGroup === groupKeys.length
                  const someInGroupSelected = selectedInGroup > 0 && !allInGroupSelected

                  return (
                    <div
                      key={group}
                      className="rounded-lg border border-border/40 overflow-hidden"
                    >
                      {/* Group header */}
                      <label
                        className="flex items-center gap-3 px-4 py-2.5 bg-muted/30 cursor-pointer hover:bg-muted/50 transition-colors select-none"
                      >
                        <Checkbox
                          checked={allInGroupSelected}
                          indeterminate={someInGroupSelected}
                          onCheckedChange={() => handleToggleGroup(groupKeys)}
                        />
                        <span className="text-sm font-semibold text-foreground flex-1">
                          {group}
                        </span>
                        <span className="text-[11px] font-medium text-muted-foreground tabular-nums">
                          {selectedInGroup}/{groupKeys.length}
                        </span>
                      </label>

                      {/* Permission items */}
                      <div className="grid grid-cols-2 sm:grid-cols-3 gap-0">
                        {permissions.map(({ key, action }) => {
                          const isChecked = selectedPermissions.includes(key)
                          return (
                            <label
                              key={key}
                              htmlFor={key}
                              className={`flex items-center gap-2.5 px-4 py-2 cursor-pointer select-none transition-colors text-sm hover:bg-muted/30 border-t border-border/20 ${
                                isChecked
                                  ? 'text-foreground'
                                  : 'text-muted-foreground'
                              }`}
                            >
                              <Checkbox
                                id={key}
                                checked={isChecked}
                                onCheckedChange={(checked) => {
                                  setSelectedPermissions((prev) =>
                                    checked
                                      ? [...prev, key]
                                      : prev.filter((p) => p !== key),
                                  )
                                }}
                              />
                              <span className="truncate">{action}</span>
                            </label>
                          )
                        })}
                      </div>
                    </div>
                  )
                })}
              </div>
            </ScrollArea>
          </>
        )}

        <DialogFooter>
          <Button variant="outline" onClick={onClose}>
            Cancel
          </Button>
          <Button
            onClick={handleSave}
            disabled={
              assignPermission.isPending ||
              unassignPermission.isPending ||
              changedCount === 0
            }
          >
            {assignPermission.isPending || unassignPermission.isPending ? (
              <Spinner className="mr-2" />
            ) : null}
            {changedCount > 0 ? `Save ${changedCount} change${changedCount > 1 ? 's' : ''}` : 'Save'}
          </Button>
        </DialogFooter>
      </DialogContent>
    </Dialog>
  )
}
