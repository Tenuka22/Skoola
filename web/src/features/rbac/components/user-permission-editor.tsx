import * as React from 'react'
import { useQuery } from '@tanstack/react-query'
import { HugeiconsIcon } from '@hugeicons/react'
import {
  Alert01Icon,
  CheckmarkCircle02Icon,
  Shield01Icon,
  ViewIcon,
} from '@hugeicons/core-free-icons'
import {
  ALL_ROLE_ENUM_VALUES,
  isPermissionEnum,
  isRoleEnum,
} from '../utils/permissions'
import { ALL_PERMISSION_ENUM_VALUES } from '../utils/constants'
import {
  getRolePermissionsQueryOptions,
  getUserPermissionsQueryOptions,
  useAssignPermissionToUser,
  useUnassignPermissionFromUser,
} from '../api'
import { PermissionList } from './permission-list'
import type {
  PermissionEnum,
  RoleEnum,
  UserResponse,
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
import { Tabs, TabsContent, TabsList, TabsTrigger } from '@/components/ui/tabs'
import { cn } from '@/lib/utils'
import { useUpdateUser } from '@/features/users/api'

interface UserPermissionEditorProps {
  user: UserResponse
}

export function UserPermissionEditor({ user }: UserPermissionEditorProps) {
  const isFullAdmin = user.role === 'FullAdmin'

  const { data: rawDirectPermissions } = useQuery(
    getUserPermissionsQueryOptions({ path: { user_id: user.id } }),
  )

  const directPermissionsFromResponse = React.useMemo(() => {
    if (!rawDirectPermissions || typeof rawDirectPermissions !== 'object') {
      return []
    }

    if (!('permissions' in rawDirectPermissions)) {
      return []
    }

    const candidate = rawDirectPermissions.permissions
    return Array.isArray(candidate)
      ? candidate.filter((p: unknown) => typeof p === 'string')
      : []
  }, [rawDirectPermissions])

  const directPermissions = React.useMemo(
    () => directPermissionsFromResponse.filter(isPermissionEnum),
    [directPermissionsFromResponse],
  )

  const { data: rawRolePermissions } = useQuery({
    ...getRolePermissionsQueryOptions({ path: { role_id: user.role } }),
    enabled: !!user.role,
  })

  const rolePermissionsFromResponse = React.useMemo(() => {
    if (!rawRolePermissions || typeof rawRolePermissions !== 'object') {
      return []
    }

    if (!('permissions' in rawRolePermissions)) {
      return []
    }

    const candidate = rawRolePermissions.permissions
    return Array.isArray(candidate)
      ? candidate.filter((p: unknown) => typeof p === 'string')
      : []
  }, [rawRolePermissions])

  const inheritedPermissions = React.useMemo(() => {
    const inherited: Array<{
      permission: PermissionEnum
      source: 'role'
      sourceName?: string
    }> = []

    rolePermissionsFromResponse
      .filter(isPermissionEnum)
      .forEach((permission) => {
        inherited.push({
          permission,
          source: 'role',
          sourceName: user.role,
        })
      })

    return inherited
  }, [rolePermissionsFromResponse, user.role])

  const updateUserRole = useUpdateUser()

  const assignDirectPermission = useAssignPermissionToUser()
  const unassignDirectPermission = useUnassignPermissionFromUser()

  const handleRoleChange = (role: RoleEnum) => {
    updateUserRole.mutate({ path: { id: user.id }, body: { role } })
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

  const name = user.email
    .split('@')[0]
    .replace(/[._]/g, ' ')
    .replace(/\b\w/g, (l) => l.toUpperCase())
  const initials = name.substring(0, 2).toUpperCase()

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
