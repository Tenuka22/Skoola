import * as React from 'react'
import { useMutation, useQuery, useQueryClient } from '@tanstack/react-query'
import { toast } from 'sonner'
import { HugeiconsIcon } from '@hugeicons/react'
import {
  Alert01Icon,
  Delete02Icon,
  Layers01Icon,
  UserIcon,
} from '@hugeicons/core-free-icons'
import { rbacApi } from '../api'
import { isPermissionEnum, isRoleEnum } from '../utils/permissions'
import { PermissionList } from './permission-list'
import type { RoleEnum, UserResponse } from '@/lib/api/types.gen'
import { z } from 'zod'
import { zPermissionEnum } from '@/lib/api/zod.gen'

type PermissionEnum = z.infer<typeof zPermissionEnum>
import { authClient } from '@/lib/clients'
import {
  getAllStaffOptions,
  updateUserMutation,
} from '@/lib/api/@tanstack/react-query.gen'
import { Badge } from '@/components/ui/badge'
import { Button } from '@/components/ui/button'
import { Card, CardContent, CardHeader, CardTitle } from '@/components/ui/card'
import {
  Select,
  SelectContent,
  SelectItem,
  SelectTrigger,
  SelectValue,
} from '@/components/ui/select'
import { ScrollArea } from '@/components/ui/scroll-area'

interface UserPermissionEditorProps {
  user: UserResponse
}

export function UserPermissionEditor({ user }: UserPermissionEditorProps) {
  const queryClient = useQueryClient()
  const isFullAdmin = user.role === 'FullAdmin'

  // Find staff member associated with this user
  const { data: staffList } = useQuery(
    getAllStaffOptions({ client: authClient }),
  )
  const staffMember = staffList?.data.find((s) => s.email === user.email)

  const { data: rawPermissions = '' } = useQuery({
    ...rbacApi.getUserPermissionsOptions(user.id),
    enabled: !!user.id,
  })

  const directPermissions = React.useMemo(
    () =>
      typeof rawPermissions === 'string' && rawPermissions
        ? rawPermissions.split(',').filter(isPermissionEnum)
        : [],
    [rawPermissions],
  )

  const { data: allPermissionSets = [] } = useQuery(rbacApi.getSetsOptions())

  const { data: userPermissionSets = [] } = useQuery({
    ...rbacApi.getStaffPermissionSetsOptions(staffMember?.id || ''),
    enabled: !!staffMember?.id,
  })

  const assignPerm = useMutation({
    ...rbacApi.assignPermissionToUserMutation(),
    onSuccess: () => {
      queryClient.invalidateQueries({
        queryKey: ['getUserPermissions', { user_id: user.id }],
      })
      toast.success('Permission assigned directly')
    },
    onError: (err) =>
      toast.error(
        err instanceof Error ? err.message : 'Failed to assign permission',
      ),
  })

  const unassignPerm = useMutation({
    ...rbacApi.unassignPermissionFromUserMutation(),
    onSuccess: () => {
      queryClient.invalidateQueries({
        queryKey: ['getUserPermissions', { user_id: user.id }],
      })
      toast.success('Direct permission removed')
    },
    onError: (err) =>
      toast.error(
        err instanceof Error ? err.message : 'Failed to remove permission',
      ),
  })

  const handleTogglePermission = (
    permission: PermissionEnum,
    checked: boolean,
  ) => {
    if (isFullAdmin) return

    if (checked) {
      assignPerm.mutate({
        path: { user_id: user.id },
        body: { permission },
      })
    } else {
      unassignPerm.mutate({
        path: { user_id: user.id },
        body: { permission },
      })
    }
  }

  const assignSet = useMutation({
    ...rbacApi.assignSetToStaffMutation(),
    onSuccess: () => {
      queryClient.invalidateQueries({
        queryKey: ['getStaffPermissionSets', { staff_id: staffMember?.id }],
      })
      toast.success('Permission set assigned')
    },
    onError: (err) =>
      toast.error(err instanceof Error ? err.message : 'Failed to assign set'),
  })

  const unassignSet = useMutation({
    ...rbacApi.unassignSetFromStaffMutation(),
    onSuccess: () => {
      queryClient.invalidateQueries({
        queryKey: ['getStaffPermissionSets', { staff_id: staffMember?.id }],
      })
      toast.success('Permission set removed')
    },
    onError: (err) =>
      toast.error(err instanceof Error ? err.message : 'Failed to remove set'),
  })

  const updateRole = useMutation({
    ...updateUserMutation({ client: authClient }),
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: ['getAllUsers'] })
      toast.success('User role updated')
    },
    onError: (err) =>
      toast.error(err instanceof Error ? err.message : 'Failed to update role'),
  })

  const handleAssignPermissionSet = (value: string | null) => {
    if (!value || !staffMember) return
    if (userPermissionSets.some((s) => s.id === value)) {
      toast.error('Set already assigned')
      return
    }
    assignSet.mutate({
      path: {
        staff_id: staffMember.id,
        set_id: value,
      },
    })
  }

  const roles: Array<RoleEnum> = [
    'Admin',
    'Teacher',
    'Student',
    'Guest',
    'Parent',
    'FullAdmin',
    'Principal',
    'VicePrincipal',
    'Accountant',
    'Librarian',
  ]

  return (
    <div className="flex flex-col h-full gap-6 animate-in fade-in duration-500">
      <Card className="border-none shadow-none bg-muted/30 rounded-2xl overflow-hidden">
        <CardHeader className="p-6">
          <div className="flex items-center justify-between">
            <div className="flex items-center gap-4">
              <div className="size-14 rounded-2xl bg-primary/10 flex items-center justify-center border border-primary/20 shadow-sm">
                <HugeiconsIcon
                  icon={UserIcon}
                  className="size-7 text-primary"
                />
              </div>
              <div className="flex flex-col gap-0.5">
                <CardTitle className="text-xl font-bold tracking-tight">
                  {user.email}
                </CardTitle>
                <div className="flex items-center gap-2">
                  <span className="text-xs font-mono text-muted-foreground opacity-70">
                    ID: {user.id}
                  </span>
                  {staffMember && (
                    <Badge
                      variant="outline"
                      className="h-4 text-[9px] px-1 bg-green-500/5 text-green-600 border-green-500/20 font-bold uppercase tracking-widest"
                    >
                      LINKED STAFF
                    </Badge>
                  )}
                </div>
              </div>
            </div>
            <div className="flex flex-col items-end gap-2">
              <span className="text-[10px] font-bold text-muted-foreground uppercase tracking-widest mr-1">
                Security Role
              </span>
              <Select
                value={user.role || 'Guest'}
                onValueChange={(val) => {
                  if (val && isRoleEnum(val)) {
                    updateRole.mutate({
                      path: { user_id: user.id },
                      body: { role: val },
                    })
                  }
                }}
              >
                <SelectTrigger className="w-[200px] h-11 rounded-xl bg-background border-muted-foreground/10 focus:ring-primary/20 font-medium">
                  <SelectValue />
                </SelectTrigger>
                <SelectContent className="rounded-xl border-muted-foreground/10 shadow-xl">
                  {roles.map((role) => (
                    <SelectItem
                      key={role}
                      value={role}
                      className="rounded-lg m-1"
                    >
                      {role}
                    </SelectItem>
                  ))}
                </SelectContent>
              </Select>
            </div>
          </div>
        </CardHeader>
      </Card>

      <div className="gap-6 flex-1 min-h-0">
        <div className=" flex flex-col gap-6 overflow-hidden">
          {isFullAdmin ? (
            <div className="flex-1 flex flex-col items-center justify-center p-8 border-2 border-dashed border-primary/30 rounded-3xl bg-primary/5 text-center gap-4">
              <div className="size-20 rounded-full bg-primary/10 flex items-center justify-center shadow-inner">
                <HugeiconsIcon
                  icon={Alert01Icon}
                  className="size-10 text-primary"
                />
              </div>
              <div className="max-w-md">
                <h3 className="text-xl font-bold text-primary tracking-tight">
                  Superuser Privilege Active
                </h3>
                <p className="text-sm text-muted-foreground mt-2 leading-relaxed">
                  This user is designated as a{' '}
                  <span className="font-bold text-foreground">FullAdmin</span>.
                  They possess absolute system authority, bypassing all granular
                  permission checks.
                </p>
              </div>
            </div>
          ) : (
            <div className="flex-1 flex flex-col gap-4 border border-muted-foreground/10 rounded-2xl bg-muted/5 p-6 overflow-hidden shadow-inner-sm">
              <div className="flex items-center justify-between mb-2">
                <h3 className="font-bold text-[13px] uppercase tracking-wider text-foreground/60 flex items-center gap-2">
                  Direct Permissions
                </h3>
                <Badge
                  variant="outline"
                  className="bg-primary/5 text-primary border-primary/20 font-mono"
                >
                  {directPermissions.length} ASSIGNED
                </Badge>
              </div>

              <div className="flex-1 min-h-0">
                <PermissionList
                  assignedPermissions={directPermissions}
                  onToggle={handleTogglePermission}
                />
              </div>
            </div>
          )}

          <div className=" flex flex-col gap-6 overflow-hidden">
            {/* Permission Sets Card */}
            <Card className="flex-1 border border-muted-foreground/10 rounded-2xl bg-card shadow-sm overflow-hidden flex flex-col">
              <CardHeader className="py-4 px-6 border-b bg-muted/30">
                <CardTitle className="text-[13px] font-bold uppercase tracking-wider text-foreground/60 flex items-center justify-between">
                  Linked Permission Sets
                  <Badge variant="secondary" className="font-mono h-5 px-1.5">
                    {userPermissionSets.length}
                  </Badge>
                </CardTitle>
              </CardHeader>
              <CardContent className="p-0 flex-1 overflow-hidden flex flex-col">
                <ScrollArea className="flex-1 p-6">
                  {userPermissionSets.length === 0 ? (
                    <div className="flex flex-col items-center justify-center py-12 text-center opacity-40">
                      <HugeiconsIcon
                        icon={Layers01Icon}
                        className="size-10 mb-3"
                      />
                      <p className="text-sm font-medium">No sets assigned</p>
                    </div>
                  ) : (
                    <div className="grid grid-cols-1 gap-2">
                      {userPermissionSets.map((set) => (
                        <div
                          key={set.id}
                          className="flex items-center justify-between gap-3 p-3 rounded-xl border border-muted-foreground/5 bg-muted/20 group"
                        >
                          <div className="flex items-center gap-3 min-w-0">
                            <div className="size-2 rounded-full bg-orange-500 shadow-[0_0_8px_rgba(249,115,22,0.4)]" />
                            <span className="text-[13px] font-semibold truncate">
                              {set.name}
                            </span>
                          </div>
                          <Button
                            variant="ghost"
                            size="icon"
                            className="size-7 rounded-md opacity-0 group-hover:opacity-100 hover:bg-destructive/10 hover:text-destructive transition-all"
                            onClick={() =>
                              unassignSet.mutate({
                                path: {
                                  staff_id: staffMember?.id || '',
                                  set_id: set.id,
                                },
                              })
                            }
                          >
                            <HugeiconsIcon
                              icon={Delete02Icon}
                              className="size-3.5"
                            />
                          </Button>
                        </div>
                      ))}
                    </div>
                  )}
                </ScrollArea>

                {staffMember && (
                  <div className="p-6 bg-muted/10 border-t border-muted-foreground/5">
                    <Select onValueChange={handleAssignPermissionSet}>
                      <SelectTrigger className="h-10 rounded-xl bg-background border-muted-foreground/10 text-xs font-medium focus:ring-primary/20">
                        <SelectValue placeholder="Add permission set..." />
                      </SelectTrigger>
                      <SelectContent className="rounded-xl border-muted-foreground/10">
                        {allPermissionSets
                          .filter(
                            (s) =>
                              !userPermissionSets.some(
                                (ups) => ups.id === s.id,
                              ),
                          )
                          .map((set) => (
                            <SelectItem
                              key={set.id}
                              value={set.id}
                              className="text-xs rounded-lg m-1"
                            >
                              {set.name}
                            </SelectItem>
                          ))}
                      </SelectContent>
                    </Select>
                  </div>
                )}
              </CardContent>
            </Card>
            {/* Inheritance Legend */}
            <div className="flex items-center justify-center gap-6 px-4 py-3 border border-muted-foreground/10 rounded-2xl bg-muted/5">
              <div className="flex items-center gap-2">
                <div className="size-2 rounded-full bg-primary shadow-[0_0_6px_rgba(var(--primary),0.4)]" />
                <span className="text-[10px] font-bold text-muted-foreground uppercase tracking-widest">
                  Direct
                </span>
              </div>
              <div className="flex items-center gap-2">
                <div className="size-2 rounded-full bg-green-500 shadow-[0_0_6px_rgba(34,197,94,0.4)]" />
                <span className="text-[10px] font-bold text-muted-foreground uppercase tracking-widest">
                  Role
                </span>
              </div>
              <div className="flex items-center gap-2">
                <div className="size-2 rounded-full bg-orange-500 shadow-[0_0_6px_rgba(249,115,22,0.4)]" />
                <span className="text-[10px] font-bold text-muted-foreground uppercase tracking-widest">
                  Set
                </span>
              </div>
            </div>
          </div>
        </div>
      </div>
    </div>
  )
}
