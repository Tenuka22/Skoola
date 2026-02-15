import * as React from 'react'
import { useMutation, useQuery, useQueryClient } from '@tanstack/react-query'
import { toast } from 'sonner'
import { HugeiconsIcon } from '@hugeicons/react'
import { Alert01Icon, Delete02Icon, UserIcon } from '@hugeicons/core-free-icons'
import { rbacApi } from '../api'
import { PermissionPalette } from './permission-palette'
import type { PermissionEnum, RoleEnum, UserResponse } from '@/lib/api/types.gen'
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

  const { data: rawPermissions = [] } = useQuery(
    rbacApi.getUserPermissionsOptions(user.id),
  )

  const directPermissions = React.useMemo(
    () =>
      Array.isArray(rawPermissions) ? (rawPermissions as Array<PermissionEnum>) : [],
    [rawPermissions],
  )

  const { data: allPermissionSets = [] } = useQuery(rbacApi.getSetsOptions())

  const { data: userPermissionSets = [] } = useQuery({
    ...rbacApi.getStaffPermissionSetsOptions(staffMember?.id as string),
    enabled: !!staffMember?.id,
  })

  const assignPerm = useMutation({
    ...rbacApi.assignPermissionToUserMutation(),
    onSuccess: () => {
      queryClient.invalidateQueries({
        queryKey: ['getUserPermissions', { user_id: user.id }],
      })
      toast.success('Permission assigned')
    },
    onError: (err) => toast.error(err.message),
  })

  const unassignPerm = useMutation({
    ...rbacApi.unassignPermissionFromUserMutation(),
    onSuccess: () => {
      queryClient.invalidateQueries({
        queryKey: ['getUserPermissions', { user_id: user.id }],
      })
      toast.success('Permission removed')
    },
    onError: (err) => toast.error(err.message),
  })

  const assignSet = useMutation({
    ...rbacApi.assignSetToStaffMutation(),
    onSuccess: () => {
      queryClient.invalidateQueries({
        queryKey: ['getStaffPermissionSets', { staff_id: staffMember?.id }],
      })
      toast.success('Permission set assigned')
    },
    onError: (err) => toast.error(err.message),
  })

  const unassignSet = useMutation({
    ...rbacApi.unassignSetFromStaffMutation(),
    onSuccess: () => {
      queryClient.invalidateQueries({
        queryKey: ['getStaffPermissionSets', { staff_id: staffMember?.id }],
      })
      toast.success('Permission set removed')
    },
    onError: (err) => toast.error(err.message),
  })

  const updateRole = useMutation({
    ...updateUserMutation({ client: authClient }),
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: ['getAllUsers'] })
      toast.success('Role updated')
    },
    onError: (err) => toast.error(err.message),
  })

  const handleDrop = (e: React.DragEvent) => {
    e.preventDefault()
    const permission = e.dataTransfer.getData('permission') as PermissionEnum
    if (permission) {
      if (directPermissions.includes(permission)) {
        toast.info('User already has this permission assigned directly.')
        return
      }

      // Basic inheritance check (simplified)
      // In a real app, we'd check if the role already has it
      assignPerm.mutate({
        path: { user_id: user.id },
        body: { permission },
      })
    }
  }

  const handleDragOver = (e: React.DragEvent) => {
    e.preventDefault()
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
    <div className="flex flex-col h-full gap-4">
      <Card>
        <CardHeader className="pb-3">
          <div className="flex items-center justify-between">
            <div className="flex items-center gap-3">
              <div className="size-10 rounded-full bg-primary/10 flex items-center justify-center">
                <HugeiconsIcon
                  icon={UserIcon}
                  className="size-5 text-primary"
                />
              </div>
              <div>
                <CardTitle className="text-lg">{user.email}</CardTitle>
                <p className="text-sm text-muted-foreground">ID: {user.id}</p>
              </div>
            </div>
            <div className="flex flex-col items-end gap-1">
              <span className="text-xs font-medium text-muted-foreground uppercase tracking-wider">
                Primary Role
              </span>
              <Select
                value={user.role || 'Guest'}
                onValueChange={(val) =>
                  updateRole.mutate({
                    path: { user_id: user.id },
                    body: { role: val as RoleEnum },
                  } as any)
                }
              >
                <SelectTrigger className="w-[180px]">
                  <SelectValue />
                </SelectTrigger>
                <SelectContent>
                  {roles.map((role) => (
                    <SelectItem key={role} value={role}>
                      {role}
                    </SelectItem>
                  ))}
                </SelectContent>
              </Select>
            </div>
          </div>
        </CardHeader>
      </Card>

      <div className="grid grid-cols-1 gap-4 flex-1 min-h-0">
        <div className="flex flex-col gap-4 overflow-hidden">
          {isFullAdmin && (
            <Card className="bg-primary/5 border-primary/20 border-2 border-dashed overflow-hidden">
              <CardContent className="p-6 flex flex-col items-center text-center gap-2">
                <div className="size-12 rounded-full bg-primary/10 flex items-center justify-center mb-2">
                  <HugeiconsIcon
                    icon={Alert01Icon}
                    className="size-6 text-primary"
                  />
                </div>
                <h3 className="text-lg font-bold text-primary italic">
                  Full System Administrative Access
                </h3>
                <p className="text-sm text-muted-foreground max-w-md">
                  This user has the <span className="font-bold">FullAdmin</span>{' '}
                  role, granting them absolute permissions across every module
                  in the system. Individual permission management is bypassed.
                </p>
              </CardContent>
            </Card>
          )}

          <Card
            className="border-dashed border-2 bg-muted/5 flex-1 flex flex-col overflow-hidden"
            onDrop={handleDrop}
            onDragOver={handleDragOver}
          >
            <CardHeader className="py-3">
              <CardTitle className="text-sm flex items-center gap-2">
                Direct Permissions
                <Badge variant="outline" className="font-mono">
                  {directPermissions.length}
                </Badge>
              </CardTitle>
            </CardHeader>
            <CardContent className="flex-1 overflow-hidden p-0 px-6 pb-6">
              <ScrollArea className="h-full pr-4">
                {directPermissions.length === 0 ? (
                  <div className="flex flex-col items-center justify-center h-32 text-muted-foreground border-2 border-dashed rounded-lg">
                    <HugeiconsIcon
                      icon={Alert01Icon}
                      className="size-8 mb-2 opacity-20"
                    />
                    <p className="text-sm">
                      Drag permissions here to assign directly
                    </p>
                  </div>
                ) : (
                  <div className="flex flex-wrap gap-2">
                    {directPermissions.map((perm) => (
                      <Badge
                        key={perm}
                        variant="secondary"
                        className="flex items-center gap-1 pl-2 pr-1 py-1"
                      >
                        {perm}
                        <Button
                          variant="ghost"
                          size="icon"
                          className="size-4 p-0 h-4 w-4 hover:bg-destructive/20 hover:text-destructive"
                          onClick={() =>
                            unassignPerm.mutate({
                              path: { user_id: user.id },
                              body: { permission: perm },
                            })
                          }
                        >
                          <HugeiconsIcon
                            icon={Delete02Icon}
                            className="size-3"
                          />
                        </Button>
                      </Badge>
                    ))}
                  </div>
                )}
              </ScrollArea>
            </CardContent>
          </Card>

          <Card className="bg-muted/5 border-dashed border-2">
            <CardHeader className="py-3">
              <CardTitle className="text-sm flex items-center justify-between">
                Permission Sets
                <div className="flex items-center gap-2">
                  {staffMember ? (
                    <Badge
                      variant="outline"
                      className="text-[10px] text-green-500 border-green-500/20"
                    >
                      Staff Link Active
                    </Badge>
                  ) : (
                    <Badge
                      variant="outline"
                      className="text-[10px] text-muted-foreground"
                    >
                      No Staff Link
                    </Badge>
                  )}
                </div>
              </CardTitle>
            </CardHeader>
            <CardContent className="px-6 pb-6">
              <div className="flex flex-col gap-4">
                <div className="flex flex-wrap gap-2">
                  {userPermissionSets.length === 0 ? (
                    <p className="text-xs text-muted-foreground italic">
                      No permission sets assigned.
                    </p>
                  ) : (
                    userPermissionSets.map((set) => (
                      <Badge
                        key={set.id}
                        variant="secondary"
                        className="gap-1.5 pl-2 pr-1 py-1"
                      >
                        <div className="size-1.5 rounded-full bg-orange-500" />
                        {set.name}
                        <Button
                          variant="ghost"
                          size="icon"
                          className="size-4 p-0 h-4 w-4 hover:bg-destructive/20 hover:text-destructive"
                          onClick={() =>
                            unassignSet.mutate({
                              path: {
                                staff_id: staffMember?.id as string,
                                set_id: set.id,
                              },
                            } as any)
                          }
                        >
                          <HugeiconsIcon
                            icon={Delete02Icon}
                            className="size-3"
                          />
                        </Button>
                      </Badge>
                    ))
                  )}
                </div>

                {staffMember && (
                  <div className="pt-2 border-t border-dashed">
                    <Select
                      onValueChange={(setId) => {
                        if (userPermissionSets.some((s) => s.id === setId)) {
                          toast.error('Set already assigned')
                          return
                        }
                        assignSet.mutate({
                          path: { staff_id: staffMember.id, set_id: setId },
                        } as any)
                      }}
                    >
                      <SelectTrigger className="h-8 text-xs">
                        <SelectValue placeholder="Assign a permission set..." />
                      </SelectTrigger>
                      <SelectContent>
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
                              className="text-xs"
                            >
                              {set.name}
                            </SelectItem>
                          ))}
                      </SelectContent>
                    </Select>
                  </div>
                )}
              </div>
            </CardContent>
          </Card>

          {/* Inheritance Indicators Legend */}
          <div className="flex items-center gap-4 px-2 text-[10px] font-medium uppercase tracking-wider text-muted-foreground">
            <div className="flex items-center gap-1.5">
              <div className="size-2 rounded-full bg-primary" />
              Direct
            </div>
            <div className="flex items-center gap-1.5">
              <div className="size-2 rounded-full bg-green-500" />
              Role
            </div>
            <div className="flex items-center gap-1.5">
              <div className="size-2 rounded-full bg-orange-500" />
              Set
            </div>
          </div>

          <Card className="flex flex-col h-[250px]">
            <CardHeader className="py-3">
              <CardTitle className="text-sm">
                Available Permissions Palette
              </CardTitle>
            </CardHeader>
            <CardContent className="flex-1 overflow-hidden p-0 px-6 pb-6">
              <PermissionPalette />
            </CardContent>
          </Card>

          {/* Audit Trail Widget Placeholder */}
          <Card className="bg-muted/30 border-none shadow-none mt-auto">
            <CardContent className="p-4 flex items-center justify-between text-xs">
              <div className="flex items-center gap-2">
                <div className="size-2 rounded-full bg-green-500 animate-pulse" />
                <span className="text-muted-foreground font-medium">
                  Last modified: 2 hours ago by Admin
                </span>
              </div>
              <Button
                variant="link"
                className="h-auto p-0 text-xs text-primary font-semibold"
              >
                View Audit Trail
              </Button>
            </CardContent>
          </Card>
        </div>
      </div>
    </div>
  )
}
