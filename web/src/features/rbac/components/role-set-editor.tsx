import * as React from 'react'
import { useQuery } from '@tanstack/react-query'
import { HugeiconsIcon } from '@hugeicons/react'
import {
  Add01Icon,
  Delete02Icon,
  Edit01Icon,
  Layers01Icon,
  Shield01Icon,
} from '@hugeicons/core-free-icons'
import { updateRoleSetSchema } from '../schemas'
import { isPermissionEnum } from '../utils/permissions'
import {
  getAllRoleSetRoleQueryOptions,
  getRolePermissionsQueryOptions,
  useAssignPermissionToRole,
  useCreateRoleSetRole,
  useDeleteRoleSetRole,
  useUnassignPermissionFromRole,
  useUpdateRoleSet,
} from '../api'
import { PermissionList } from './permission-list'
import type { UpdateRoleSetInput as UpdateRoleSetValues } from '../schemas'
import type { PermissionEnum, RoleSet, RoleSetRole } from '@/lib/api/types.gen'
import { FormBuilder, defineFormConfig } from '@/components/form-builder'
import { Button } from '@/components/ui/button'
import {
  Card,
  CardAction,
  CardContent,
  CardDescription,
  CardHeader,
  CardTitle,
} from '@/components/ui/card'
import { HStack, Stack, Text } from '@/components/primitives'
import { Skeleton } from '@/components/ui/skeleton'
import { RoleEnumSchema } from '@/lib/api/schemas.gen'
import {
  Select,
  SelectContent,
  SelectItem,
  SelectTrigger,
  SelectValue,
} from '@/components/ui/select'
import { Tabs, TabsContent, TabsList, TabsTrigger } from '@/components/ui/tabs'
import { Badge } from '@/components/ui/badge'
import { Avatar, AvatarFallback } from '@/components/ui/avatar'
import { ScrollArea } from '@/components/ui/scroll-area'
import { cn } from '@/lib/utils'

interface RoleSetEditorProps {
  set: RoleSet
}

function PermissionsForRoleEditor({ role }: { role: string }) {
  const { data: rawPermissions, isLoading } = useQuery({
    ...getRolePermissionsQueryOptions({ path: { role_id: role } }),
    enabled: !!role,
  })

  const permissionsFromResponse = React.useMemo(() => {
    if (!rawPermissions || typeof rawPermissions !== 'object') {
      return []
    }

    if (!('permissions' in rawPermissions)) {
      return []
    }

    const candidate = rawPermissions.permissions
    return Array.isArray(candidate)
      ? candidate.filter((p: unknown): p is string => typeof p === 'string')
      : []
  }, [rawPermissions])

  const assignedPermissions = React.useMemo(() => {
    return permissionsFromResponse.filter(isPermissionEnum)
  }, [permissionsFromResponse])

  const assignPerm = useAssignPermissionToRole()
  const unassignPerm = useUnassignPermissionFromRole()

  const handleTogglePermission = (
    permission: PermissionEnum,
    checked: boolean,
  ) => {
    if (checked) {
      assignPerm.mutate({
        path: { role_id: role },
        body: { permission },
      })
    } else {
      unassignPerm.mutate({
        path: { role_id: role },
        body: { permission },
      })
    }
  }

  if (isLoading) {
    return (
      <Stack gap={2} className="mt-4">
        {Array.from({ length: 5 }).map((_, i) => (
          <Skeleton key={i} className="h-10 w-full" />
        ))}
      </Stack>
    )
  }

  return (
    <PermissionList
      assignedPermissions={assignedPermissions}
      onToggle={handleTogglePermission}
    />
  )
}

export function RoleSetEditor({ set }: RoleSetEditorProps) {
  const [isEditingInfo, setIsEditingInfo] = React.useState(false)
  const [selectedRole, setSelectedRole] = React.useState<string>('')

  React.useEffect(() => {
    setIsEditingInfo(false)
  }, [set.id])

  const { data: roleSetRolesData } = useQuery({
    ...getAllRoleSetRoleQueryOptions({
      query: { limit: 1000 },
    }),
    enabled: !!set.id,
  })

  const assignedRoles = React.useMemo(() => {
    const all: Array<RoleSetRole> = roleSetRolesData?.data ?? []
    return all.filter((r) => r.role_set_id === set.id).map((r) => r.role_id)
  }, [roleSetRolesData, set.id])

  const allRoles = RoleEnumSchema.enum
  const availableRoles = allRoles.filter((r) => !assignedRoles.includes(r))

  const updateSet = useUpdateRoleSet()
  const addRoleToSet = useCreateRoleSetRole()
  const removeRoleFromSet = useDeleteRoleSetRole()

  const handleSaveInfo = (values: UpdateRoleSetValues) => {
    updateSet.mutate(
      {
        path: { id: set.id },
        body: { name: values.name, description: values.description },
      },
      {
        onSuccess: () => {
          setIsEditingInfo(false)
        },
      },
    )
  }

  const formConfig = defineFormConfig(updateRoleSetSchema, {
    structure: [
      [{ field: 'name', type: 'input', label: 'Set Name' }],
      [
        {
          field: 'description',
          type: 'textarea',
          label: 'Description',
          rows: 3,
          textareaClassName: 'resize-none',
        },
      ],
    ],
  })

  return (
    <Stack gap={6} className="animate-in fade-in duration-300">
      {isEditingInfo ? (
        <Card>
          <CardHeader className="border-b">
            <CardTitle>Edit Role Set Details</CardTitle>
          </CardHeader>
          <CardContent>
            <FormBuilder
              schema={updateRoleSetSchema}
              config={formConfig}
              defaultValues={{
                name: set.name,
                description: set.description || '',
              }}
              onSubmit={handleSaveInfo}
              isLoading={updateSet.isPending}
              actions={[
                {
                  label: 'Save Changes',
                  type: 'submit',
                  variant: 'default',
                },
                {
                  label: 'Cancel',
                  type: 'button',
                  variant: 'outline',
                  onClick: () => setIsEditingInfo(false),
                },
              ]}
              showSuccessAlert={false}
              showErrorSummary={false}
            />
          </CardContent>
        </Card>
      ) : (
        <Card>
          <CardHeader>
            <HStack justify="between" align="center">
              <HStack gap={3}>
                <HugeiconsIcon
                  icon={Shield01Icon}
                  className="size-11 text-primary"
                />
                <Stack gap={0}>
                  <CardTitle>{set.name}</CardTitle>
                  <Text size="xs" className="truncate" muted>
                    ID: {set.id}
                  </Text>
                  <CardDescription>
                    {set.description || 'No description provided.'}
                  </CardDescription>

                  <HStack gap={6} className="pt-2">
                    <Text size="sm" muted>
                      Role permissions are managed separately.
                    </Text>
                  </HStack>
                </Stack>
              </HStack>

              <CardAction>
                <Button
                  variant="secondary"
                  className="gap-2"
                  onClick={() => setIsEditingInfo(true)}
                >
                  Edit
                  <HugeiconsIcon
                    icon={Edit01Icon}
                    className="size-4 text-primary"
                  />
                </Button>
              </CardAction>
            </HStack>
          </CardHeader>
        </Card>
      )}

      <Tabs defaultValue="roles" className="flex-1 flex flex-col min-h-0">
        <TabsList className="w-fit mb-2">
          <TabsTrigger value="roles">
            <HStack gap={2} p={0}>
              <HugeiconsIcon icon={Shield01Icon} className="size-4" />
              <span>Assigned Roles</span>
            </HStack>
          </TabsTrigger>
          <TabsTrigger value="permissions">
            <HStack gap={2} p={0}>
              <HugeiconsIcon icon={Layers01Icon} className="size-4" />
              <span>Role Permissions</span>
            </HStack>
          </TabsTrigger>
        </TabsList>

        <TabsContent value="roles" className="min-h-0">
          <Card className="flex flex-col h-full">
            <CardHeader className="border-b">
              <Stack gap={2}>
                <HStack justify="between">
                  <CardTitle>Assigned Roles</CardTitle>
                  <HStack gap={2}>
                    <Badge variant="secondary" className="font-mono">
                      {assignedRoles.length} Total
                    </Badge>
                    {availableRoles.length > 0 && (
                      <Select
                        onValueChange={(role) => {
                          if (role) {
                            addRoleToSet.mutate(
                              {
                                body: {
                                  role_id: role,
                                  role_set_id: set.id,
                                },
                              },
                              {
                                onSuccess: () => setSelectedRole(''),
                              },
                            )
                          }
                        }}
                        value=""
                      >
                        <SelectTrigger className="w-[180px]">
                          <HugeiconsIcon
                            icon={Add01Icon}
                            className="size-4 mr-2"
                          />
                          <SelectValue placeholder="Add role..." />
                        </SelectTrigger>
                        <SelectContent>
                          {availableRoles.map((role) => (
                            <SelectItem key={role} value={role}>
                              {role}
                            </SelectItem>
                          ))}
                        </SelectContent>
                      </Select>
                    )}
                  </HStack>
                </HStack>
                <ScrollArea className="h-60">
                  <Stack gap={1} p={2}>
                    {assignedRoles.length === 0 ? (
                      <Text className="text-center text-sm text-muted-foreground py-4">
                        No roles assigned to this role set yet.
                      </Text>
                    ) : (
                      assignedRoles.map((roleId) => {
                        const initials = roleId.substring(0, 2).toUpperCase()

                        return (
                          <HStack
                            key={roleId}
                            align="center"
                            justify="between"
                            gap={3}
                            className={cn(
                              'p-2 rounded-md transition-colors hover:bg-muted/50',
                            )}
                          >
                            <HStack align="center" gap={3}>
                              <Avatar className="h-8 w-8 border border-border/50">
                                <AvatarFallback className="text-[10px] font-semibold bg-primary/10 text-primary">
                                  {initials}
                                </AvatarFallback>
                              </Avatar>
                              <Stack gap={0}>
                                <Text size="sm" className="font-medium">
                                  {roleId}
                                </Text>
                                <Text size="xs" muted>
                                  Role ID
                                </Text>
                              </Stack>
                            </HStack>
                            <Button
                              variant="ghost"
                              size="icon"
                              className="size-8 text-destructive hover:text-destructive"
                              onClick={() => {
                                // Use role_id as the identifier since backend doesn't return separate ID
                                removeRoleFromSet.mutate({
                                  path: { id: roleId },
                                })
                              }}
                              disabled={removeRoleFromSet.isPending}
                            >
                              <HugeiconsIcon
                                icon={Delete02Icon}
                                className="size-4"
                              />
                            </Button>
                          </HStack>
                        )
                      })
                    )}
                  </Stack>
                </ScrollArea>
              </Stack>
            </CardHeader>
          </Card>
        </TabsContent>

        <TabsContent value="permissions" className="min-h-0">
          <Card className="flex flex-col h-full">
            <CardHeader>
              <CardTitle>Permissions for Role</CardTitle>
              <CardDescription>
                Select a role from this set to view and manage its directly
                assigned permissions.
              </CardDescription>
            </CardHeader>
            <CardContent>
              <Stack gap={4}>
                <Select
                  onValueChange={(v) => {
                    if (v) setSelectedRole(v)
                  }}
                  value={selectedRole}
                >
                  <SelectTrigger className="w-[280px]">
                    <SelectValue placeholder="Select a role from assigned..." />
                  </SelectTrigger>
                  <SelectContent>
                    {assignedRoles.map((role) => (
                      <SelectItem key={role} value={role}>
                        {role}
                      </SelectItem>
                    ))}
                  </SelectContent>
                </Select>

                {selectedRole ? (
                  <PermissionsForRoleEditor role={selectedRole} />
                ) : null}
              </Stack>
            </CardContent>
          </Card>
        </TabsContent>
      </Tabs>
    </Stack>
  )
}
