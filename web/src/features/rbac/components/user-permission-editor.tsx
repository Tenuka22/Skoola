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
  CheckmarkCircle02Icon,
  Layers01Icon,
  Shield01Icon,
  ViewIcon,
} from '@hugeicons/core-free-icons'
import { rbacApi } from '../api'
import {
  ALL_ROLE_ENUM_VALUES,
  isPermissionEnum,
  isRoleEnum,
} from '../utils/permissions'
import { ALL_PERMISSION_ENUM_VALUES } from '../utils/constants'
import { PermissionList } from './permission-list'
import type {
  PermissionEnum,
  RoleEnum,
  StaffResponse as Staff,
  UserResponse,
  UserSet,
} from '@/lib/api/types.gen'
import { Badge } from '@/components/ui/badge'

import {
  Card,
  CardAction,
  CardDescription,
  CardHeader,
  CardTitle,
} from '@/components/ui/card'
import { Avatar, AvatarFallback, AvatarImage } from '@/components/ui/avatar'
import { Input } from '@/components/ui/input'
import { Checkbox } from '@/components/ui/checkbox'
import {
  Select,
  SelectContent,
  SelectItem,
  SelectTrigger,
  SelectValue,
} from '@/components/ui/select'
import {
  Box,
  Grid,
  HStack,
  Heading,
  Stack,
  Text,
} from '@/components/primitives'
import {
  Empty,
  EmptyHeader,
  EmptyMedia,
  EmptyTitle,
} from '@/components/ui/empty'
import { Tabs, TabsContent, TabsList, TabsTrigger } from '@/components/ui/tabs'
import { cn } from '@/lib/utils'

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
    select: (data) => data || [],
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
      toast.success('Permission set assigned')
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
      toast.success('Permission set unassigned')
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
      toast.info('User is not linked to a staff member.')
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

  const name = user.email
    .split('@')[0]
    .replace(/[._]/g, ' ')
    .replace(/\b\w/g, (l) => l.toUpperCase())
  const initials = name.substring(0, 2).toUpperCase()

  const [searchTerm, setSearchTerm] = React.useState('')
  const filteredPermissionSets = React.useMemo(() => {
    if (!searchTerm) {
      return allPermissionSets
    }
    const lowerCaseSearchTerm = searchTerm.toLowerCase()
    return allPermissionSets.filter((set) =>
      set.name.toLowerCase().includes(lowerCaseSearchTerm),
    )
  }, [allPermissionSets, searchTerm])

  return (
    <Stack gap={4} p={0}>
      <Card>
        <CardHeader>
          <HStack justify="between" align="center" p={0}>
            <HStack gap={3}>
              <Avatar className="h-11 w-11 border border-border/50">
                <AvatarImage
                  src={`https://api.dicebear.com/7.x/avataaars/svg?seed=${user.email}`}
                />
                <AvatarFallback className="text-[10px] font-semibold">
                  {initials}
                </AvatarFallback>
              </Avatar>
              <Stack gap={0}>
                <CardTitle>{user.email}</CardTitle>

                <HStack gap={2} align="center">
                  <Text size="xs" className="truncate" muted>
                    ID: {user.id}
                  </Text>
                  {staffMember && (
                    <Badge
                      variant="outline"
                      className="h-4 text-[10px] px-1 bg-green-500/10 text-green-600 border-green-600/20"
                    >
                      Staff Linked
                    </Badge>
                  )}
                </HStack>
              </Stack>
            </HStack>

            <CardAction>
              <Select
                value={user.role || 'Guest'}
                onValueChange={(val) => {
                  if (val && isRoleEnum(val)) handleRoleChange(val)
                }}
                disabled={updateUserRole.isPending}
              >
                <SelectTrigger className="w-35 h-8">
                  <HStack gap={2} p={0}>
                    <HugeiconsIcon icon={Shield01Icon} className="size-4" />
                    <SelectValue />
                  </HStack>
                </SelectTrigger>
                <SelectContent>
                  {ALL_ROLE_ENUM_VALUES.map((role) => (
                    <SelectItem key={role} value={role}>
                      {role}
                    </SelectItem>
                  ))}
                </SelectContent>
              </Select>
            </CardAction>
          </HStack>
        </CardHeader>
      </Card>

      {isFullAdmin ? (
        <Card className="border-primary/50 bg-primary/5 p-8">
          <Stack align="center" gap={3}>
            <HugeiconsIcon icon={Alert01Icon} className="size-8 text-primary" />
            <Heading size="h4" className="text-primary">
              Superuser Active
            </Heading>
            <Text size="sm" className="max-w-md text-center" muted>
              FullAdmin role grants absolute system authority, bypassing
              granular checks.
            </Text>
          </Stack>
        </Card>
      ) : (
        <Tabs defaultValue="overview" className="flex-1 flex flex-col min-h-0">
          <TabsList className="w-fit mb-2">
            <TabsTrigger value="overview">
              <HStack gap={2} p={0}>
                <HugeiconsIcon icon={ViewIcon} className="size-4" />
                <span>Overview</span>
              </HStack>
            </TabsTrigger>
            <TabsTrigger value="groups">
              <HStack gap={2} p={0}>
                <HugeiconsIcon icon={Layers01Icon} className="size-4" />
                <span>Groups & Sets</span>
              </HStack>
            </TabsTrigger>
            <TabsTrigger value="direct">
              <HStack gap={2} p={0}>
                <HugeiconsIcon icon={Shield01Icon} className="size-4" />
                <span>Direct Access</span>
              </HStack>
            </TabsTrigger>
          </TabsList>

          <TabsContent value="overview">
            <Card>
              <CardHeader>
                <Stack gap={4} className="h-full">
                  <HStack justify="between">
                    <Stack gap={0}>
                      <CardTitle>Permission Grid</CardTitle>
                      <CardDescription>
                        Real-time view of all effective permissions.
                      </CardDescription>
                    </Stack>
                    <HStack gap={3}>
                      <HStack gap={2}>
                        <Box className="size-2 rounded-full bg-primary" />
                        <Text size="xs" muted>
                          Active
                        </Text>
                      </HStack>
                      <HStack gap={2}>
                        <Box className="size-2 rounded-full bg-muted border" />
                        <Text size="xs" muted>
                          Denied
                        </Text>
                      </HStack>
                    </HStack>
                  </HStack>

                  <Grid cols={4} gap={2} p={0}>
                    {ALL_PERMISSION_ENUM_VALUES.map((p) => {
                      const isDirect = directPermissions.includes(p)
                      const inherited = inheritedPermissions.filter(
                        (ip) => ip.permission === p,
                      )
                      const isActive = isDirect || inherited.length > 0

                      return (
                        <Box
                          key={p}
                          p={2}
                          rounded="md"
                          className={cn(
                            'border',
                            isActive
                              ? 'bg-primary/5 border-primary/20'
                              : 'bg-muted/10 border-muted-foreground/20 opacity-40 grayscale',
                          )}
                        >
                          <Stack gap={2}>
                            <HStack justify="between">
                              <Text
                                size="xs"
                                className={cn(
                                  'font-medium truncate',
                                  isActive
                                    ? 'text-foreground'
                                    : 'text-muted-foreground',
                                )}
                              >
                                {p}
                              </Text>
                              {isActive && (
                                <HugeiconsIcon
                                  icon={CheckmarkCircle02Icon}
                                  className="size-3 text-primary shrink-0"
                                />
                              )}
                            </HStack>
                            {isActive && (
                              <HStack gap={1} className="flex-wrap">
                                {isDirect && (
                                  <Badge
                                    variant="default"
                                    className="h-3 text-[8px] px-1 font-normal bg-primary/20 text-primary border-0"
                                  >
                                    Direct
                                  </Badge>
                                )}
                                {inherited.map((inh, i) => (
                                  <Badge
                                    key={i}
                                    variant="secondary"
                                    className="h-3 text-[8px] px-1 font-normal opacity-80"
                                  >
                                    {inh.source === 'role' ? 'Role' : 'Set'}
                                  </Badge>
                                ))}
                              </HStack>
                            )}
                          </Stack>
                        </Box>
                      )
                    })}
                  </Grid>
                </Stack>
              </CardHeader>
            </Card>
          </TabsContent>

          <TabsContent value="groups">
            <Card className="p-4 flex flex-col">
              <Stack gap={4} className="flex-1">
                <Stack gap={1}>
                  <Heading size="h4">Linked Permission Sets</Heading>
                  <Text size="xs" muted>
                    Assign or remove reusable permission groups from staff.
                  </Text>
                  <Input
                    placeholder="Search permission sets..."
                    className="h-8 text-xs w-full mt-2"
                    value={searchTerm}
                    onChange={(e) => setSearchTerm(e.target.value)}
                  />
                </Stack>

                <Stack gap={2}>
                  {filteredPermissionSets.length === 0 ? (
                    <Empty className="py-8 border-0">
                      <EmptyHeader>
                        <EmptyMedia variant="icon">
                          <HugeiconsIcon icon={Layers01Icon} />
                        </EmptyMedia>
                        <EmptyTitle className="text-sm">
                          No permission sets available
                        </EmptyTitle>
                      </EmptyHeader>
                    </Empty>
                  ) : (
                    allPermissionSets.map((set) => {
                      const isLinked = userPermissionSets.some(
                        (us: UserSet) => us.id === set.id,
                      )
                      return (
                        <HStack
                          key={set.id}
                          p={2}
                          justify="between"
                          className="rounded-md bg-muted/50 group border border-transparent hover:border-border transition-all"
                        >
                          <HStack gap={2} className="min-w-0">
                            <Checkbox
                              id={`set-${set.id}`}
                              checked={isLinked}
                              onCheckedChange={(checked) => {
                                if (checked) {
                                  handleAssignSet(set.id)
                                } else {
                                  handleUnassignSet(set.id)
                                }
                              }}
                            />
                            <label
                              htmlFor={`set-${set.id}`}
                              className="text-sm font-medium leading-none peer-disabled:cursor-not-allowed peer-disabled:opacity-70 flex-1 truncate"
                            >
                              {set.name}
                            </label>
                          </HStack>
                        </HStack>
                      )
                    })
                  )}
                </Stack>
              </Stack>
            </Card>
          </TabsContent>

          <TabsContent value="direct">
            <Card>
              <CardHeader>
                <Stack gap={4} className="flex-1">
                  <HStack justify="between">
                    <Stack gap={0}>
                      <CardTitle>Direct Permissions</CardTitle>
                      <CardDescription>
                        Assign granular access directly to this user.
                      </CardDescription>
                    </Stack>
                    <Badge
                      variant="secondary"
                      className="font-mono text-[10px] h-5"
                    >
                      {directPermissions.length} Direct
                    </Badge>
                  </HStack>
                  <Box className="flex-1 min-h-0">
                    <PermissionList
                      assignedPermissions={directPermissions}
                      inheritedPermissions={inheritedPermissions}
                      onToggle={handleToggleDirectPermission}
                    />
                  </Box>
                </Stack>
              </CardHeader>
            </Card>
          </TabsContent>
        </Tabs>
      )}
    </Stack>
  )
}
