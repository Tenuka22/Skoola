import * as React from 'react'
import {
  useMutation,
  useQueries,
  useQuery,
  useQueryClient,
} from '@tanstack/react-query'
import { toast } from 'sonner'
import { HugeiconsIcon } from '@hugeicons/react'
import {
  Alert01Icon,
  Delete02Icon,
  Layers01Icon,
  UserIcon,
} from '@hugeicons/core-free-icons'
import { rbacApi } from '../api'
import {
  ALL_ROLE_ENUM_VALUES,
  isPermissionEnum,
  isRoleEnum,
} from '../utils/permissions'
import { PermissionList } from './permission-list'
import type {
  PermissionEnum,
  RoleEnum,
  StaffResponse as Staff,
  UserResponse,
  UserSet,
} from '@/lib/api/types.gen'
import { Badge } from '@/components/ui/badge'
import { Button } from '@/components/ui/button'
import { Card, CardContent, CardHeader, CardTitle } from '@/components/ui/card'
import {
  Combobox,
  ComboboxContent,
  ComboboxEmpty,
  ComboboxInput,
  ComboboxItem,
  ComboboxList,
} from '@/components/ui/combobox'
import {
  Select,
  SelectContent,
  SelectItem,
  SelectTrigger,
  SelectValue,
} from '@/components/ui/select'
import { ScrollArea } from '@/components/ui/scroll-area'
import {
  Box,
  Grid,
  HStack,
  Heading,
  Stack,
  Text,
} from '@/components/primitives'

interface UserPermissionEditorProps {
  user: UserResponse
}

export function UserPermissionEditor({ user }: UserPermissionEditorProps) {
  const queryClient = useQueryClient()
  const isFullAdmin = user.role === 'FullAdmin'

  const { data: staffList } = useQuery(rbacApi.getAllStaffOptions())
  const staffMember = staffList?.data.find((s: Staff) => s.email === user.email)

  const { data: rawDirectPermissions } = useQuery({
    ...rbacApi.getUserPermissionsOptions(user.id),
    enabled: !!user.id,
  })

  const { data: allPermissionSets = [] } = useQuery({
    ...rbacApi.getSetsOptions(),
    select: (data) => data || [], // Ensure it fits UserSet[]
  })

  const { data: userPermissionSets = [] } = useQuery({
    ...rbacApi.getStaffPermissionSetsOptions(staffMember?.id || ''),
    enabled: !!staffMember,
  })

  const directPermissions = React.useMemo(
    () =>
      (rawDirectPermissions?.permissions || [])
        .filter(isPermissionEnum)
        .filter(Boolean),
    [rawDirectPermissions],
  )

  const { data: rolePermissionsRes } = useQuery({
    ...rbacApi.getRolePermissionsOptions(user.role),
    enabled: !!user.role,
  })

  const setPermissionsResults = useQueries({
    queries: userPermissionSets.map((set: UserSet) => ({
      ...rbacApi.getSetPermissionsOptions(set.id),
      enabled: !!set.id,
    })),
  })

  const inheritedPermissions = React.useMemo(() => {
    const inherited: Array<{
      permission: PermissionEnum
      source: 'role' | 'set'
      sourceName?: string
    }> = []

    if (rolePermissionsRes?.permissions) {
      rolePermissionsRes.permissions.filter(isPermissionEnum).forEach((p) => {
        inherited.push({ permission: p, source: 'role', sourceName: user.role })
      })
    }

    setPermissionsResults.forEach((res, index) => {
      const set = userPermissionSets[index]
      if (res.data?.permissions) {
        res.data.permissions.filter(isPermissionEnum).forEach((p) => {
          inherited.push({
            permission: p,
            source: 'set',
            sourceName: set.name,
          })
        })
      }
    })

    return inherited
  }, [rolePermissionsRes, setPermissionsResults, userPermissionSets, user.role])

  const updateUserRole = useMutation({
    ...rbacApi.updateUserMutation(),
    onSuccess: () => {
      toast.success("User's role updated successfully")
      queryClient.invalidateQueries({ queryKey: ['getAllUsers'] })
    },
    onError: (err: Error) => toast.error(err.message),
  })

  const assignDirectPermission = useMutation({
    ...rbacApi.assignPermissionToUserMutation(),
    onSuccess: () => {
      toast.success('Direct permission assigned')
      queryClient.invalidateQueries({
        queryKey: rbacApi.getUserPermissionsOptions(user.id).queryKey,
      })
    },
    onError: (err: Error) => toast.error(err.message),
  })

  const unassignDirectPermission = useMutation({
    ...rbacApi.unassignPermissionFromUserMutation(),
    onSuccess: () => {
      toast.success('Direct permission removed')
      queryClient.invalidateQueries({
        queryKey: rbacApi.getUserPermissionsOptions(user.id).queryKey,
      })
    },
    onError: (err: Error) => toast.error(err.message),
  })

  const assignSetToStaff = useMutation({
    ...rbacApi.assignSetToStaffMutation(),
    onSuccess: () => {
      toast.success('Permission set assigned to staff member')
      queryClient.invalidateQueries({
        queryKey: rbacApi.getStaffPermissionSetsOptions(staffMember?.id || '')
          .queryKey,
      })
    },
    onError: (err: Error) => toast.error(err.message),
  })

  const unassignSetFromStaff = useMutation({
    ...rbacApi.unassignSetFromStaffMutation(),
    onSuccess: () => {
      toast.success('Permission set unassigned from staff member')
      queryClient.invalidateQueries({
        queryKey: rbacApi.getStaffPermissionSetsOptions(staffMember?.id || '')
          .queryKey,
      })
    },
    onError: (err: Error) => toast.error(err.message),
  })

  const handleRoleChange = (role: RoleEnum) => {
    updateUserRole.mutate({ path: { user_id: user.id }, body: { role } })
  }

  const handleToggleDirectPermission = (
    permission: PermissionEnum,
    checked: boolean,
  ) => {
    if (isFullAdmin) return
    if (checked) {
      assignDirectPermission.mutate({
        path: { user_id: user.id },
        body: { permission },
      })
    } else {
      unassignDirectPermission.mutate({
        path: { user_id: user.id },
        body: { permission },
      })
    }
  }

  const handleAssignSet = (setId: string) => {
    if (!staffMember) {
      toast.info('Cannot assign set: User is not linked to a staff member.')
      return
    }
    assignSetToStaff.mutate({
      path: { staff_id: staffMember.id, set_id: setId },
    })
  }

  const handleUnassignSet = (setId: string) => {
    if (!staffMember) return
    unassignSetFromStaff.mutate({
      path: { staff_id: staffMember.id, set_id: setId },
    })
  }

  const availableSets =
    allPermissionSets?.filter(
      (s: UserSet) => !userPermissionSets.some((us: UserSet) => us.id === s.id),
    ) || []

  return (
    <Stack gap={6} className="animate-in fade-in duration-300">
      <Card>
        <CardHeader>
          <HStack justify="between" align="start">
            <HStack gap={4}>
              <Box
                p={3}
                rounded="lg"
                className="bg-muted border dark:bg-zinc-800"
              >
                <HugeiconsIcon
                  icon={UserIcon}
                  className="size-8 text-primary"
                />
              </Box>
              <Stack gap={1}>
                <Heading size="h3">{user.email}</Heading>
                <Text size="sm" muted>
                  ID: {user.id}
                </Text>
                {staffMember && (
                  <Badge
                    variant="outline"
                    className="w-fit border-green-600/50 bg-green-500/10 text-green-600"
                  >
                    Linked to Staff Member
                  </Badge>
                )}
              </Stack>
            </HStack>

            <Stack gap={2} className="items-end">
              <Text
                size="xs"
                className="font-bold uppercase tracking-wider text-muted-foreground"
              >
                Security Role
              </Text>
              <Select
                value={user.role || 'Guest'}
                onValueChange={(val) => {
                  if (val && isRoleEnum(val)) handleRoleChange(val)
                }}
                disabled={updateUserRole.isPending}
              >
                <SelectTrigger className="w-[180px]">
                  <SelectValue />
                </SelectTrigger>
                <SelectContent>
                  {ALL_ROLE_ENUM_VALUES.map((role) => (
                    <SelectItem key={role} value={role}>
                      {role}
                    </SelectItem>
                  ))}
                </SelectContent>
              </Select>
            </Stack>
          </HStack>
        </CardHeader>
      </Card>

      {isFullAdmin ? (
        <Card className="border-primary/50 bg-primary/5 text-center">
          <CardContent>
            <Stack align="center" p={8} gap={4}>
              <HugeiconsIcon
                icon={Alert01Icon}
                className="size-10 text-primary"
              />
              <Heading size="h3" className="text-primary">
                Superuser Privilege Active
              </Heading>
              <Text className="max-w-md text-primary/90" muted>
                This user is a FullAdmin. They possess absolute system
                authority, bypassing all granular permission checks.
              </Text>
            </Stack>
          </CardContent>
        </Card>
      ) : (
        <Grid cols={2} gap={6}>
          <Card className="h-full flex flex-col">
            <Box p={6} className="border-b">
              <HStack justify="between">
                <Heading size="h4">User Permissions</Heading>
                <Badge variant="secondary" className="font-mono">
                  {directPermissions.length} Direct
                </Badge>
              </HStack>
            </Box>
            <Box p={6} className="flex-1 min-h-0">
              <PermissionList
                assignedPermissions={directPermissions}
                inheritedPermissions={inheritedPermissions}
                onToggle={handleToggleDirectPermission}
              />
            </Box>
          </Card>

          {/* Linked Permission Sets */}
          <Stack gap={6}>
            <Card>
              <CardHeader>
                <HStack justify="between">
                  <CardTitle>Linked Sets</CardTitle>
                  <Badge variant="secondary" className="font-mono">
                    {userPermissionSets.length} Linked
                  </Badge>
                </HStack>
              </CardHeader>
              <CardContent>
                <Stack gap={4}>
                  <ScrollArea className="h-48">
                    <Stack gap={2}>
                      {userPermissionSets.length === 0 ? (
                        <Stack
                          align="center"
                          className="py-8 text-center"
                          gap={2}
                        >
                          <HugeiconsIcon
                            icon={Layers01Icon}
                            className="size-8 text-muted-foreground/70"
                          />
                          <Text
                            size="sm"
                            className="font-medium text-muted-foreground"
                          >
                            No sets linked
                          </Text>
                        </Stack>
                      ) : (
                        userPermissionSets.map((set: UserSet) => (
                          <HStack
                            key={set.id}
                            p={2}
                            justify="between"
                            className="rounded-md bg-muted/50 group"
                          >
                            <HStack gap={2}>
                              <HugeiconsIcon
                                icon={Layers01Icon}
                                className="size-4 text-muted-foreground"
                              />
                              <Text className="font-semibold" size="sm">
                                {set.name}
                              </Text>
                            </HStack>
                            <Button
                              variant="ghost"
                              size="icon"
                              className="size-7 opacity-0 group-hover:opacity-100"
                              onClick={() => handleUnassignSet(set.id)}
                            >
                              <HugeiconsIcon
                                icon={Delete02Icon}
                                className="size-4 text-destructive"
                              />
                            </Button>
                          </HStack>
                        ))
                      )}
                    </Stack>
                  </ScrollArea>

                  {staffMember && (
                    <Combobox
                      onValueChange={(setId) => {
                        if (typeof setId === 'string') handleAssignSet(setId)
                      }}
                    >
                      <ComboboxInput
                        placeholder="Link a permission set..."
                        className="h-9 px-3 text-sm"
                        showTrigger={true}
                      />
                      <ComboboxContent>
                        <ComboboxList>
                          {availableSets.length === 0 ? (
                            <ComboboxEmpty>
                              No more sets available
                            </ComboboxEmpty>
                          ) : (
                            availableSets.map((set: UserSet) => (
                              <ComboboxItem key={set.id} value={set.id}>
                                {set.name}
                              </ComboboxItem>
                            ))
                          )}
                        </ComboboxList>
                      </ComboboxContent>
                    </Combobox>
                  )}
                </Stack>
              </CardContent>
            </Card>

            <Card className="border-dashed">
              <Box p={6}>
                <Stack gap={4}>
                  <Heading size="h4">Permission Inheritance</Heading>
                  <HStack
                    justify="around"
                    className="bg-background/50 p-4 rounded-lg border"
                  >
                    <HStack gap={2}>
                      <Box className="size-2.5 rounded-full bg-primary" />
                      <Text size="sm">Direct</Text>
                    </HStack>
                    <HStack gap={2}>
                      <Box className="size-2.5 rounded-full bg-zinc-600" />
                      <Text size="sm">Inherent (Role/Set)</Text>
                    </HStack>
                    <HStack gap={2}>
                      <Badge
                        variant="outline"
                        className="text-[10px] py-0 h-4 bg-muted/30"
                      >
                        Label
                      </Badge>
                      <Text size="sm">Source info</Text>
                    </HStack>
                  </HStack>
                  <Text size="xs" muted>
                    Permissions inherited from roles or sets are locked and
                    cannot be removed directly from the user editor.
                  </Text>
                </Stack>
              </Box>
            </Card>
          </Stack>
        </Grid>
      )}
    </Stack>
  )
}
