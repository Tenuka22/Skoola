import * as React from 'react'
import { useMutation, useQuery, useQueryClient } from '@tanstack/react-query'
import { toast } from 'sonner'
import { HugeiconsIcon } from '@hugeicons/react'
import {
  Edit01Icon,
  Layers01Icon,
  Search01Icon,
  Shield01Icon,
} from '@hugeicons/core-free-icons'
import { updateRoleSetSchema } from '../schemas'
import { rbacApi } from '../api'
import { isPermissionEnum } from '../utils/permissions'
import { PermissionList } from './permission-list'
import type { UpdateRoleSetInput as UpdateRoleSetValues } from '../schemas'
import type { PermissionEnum, RoleSet } from '@/lib/api/types.gen'
import { FormBuilder, defineFormConfig } from '@/components/form-builder'
import { Badge } from '@/components/ui/badge'
import { Button } from '@/components/ui/button'
import {
  Card,
  CardAction,
  CardContent,
  CardDescription,
  CardHeader,
  CardTitle,
} from '@/components/ui/card'
import { Box, HStack, Stack, Text } from '@/components/primitives'
import { Skeleton } from '@/components/ui/skeleton'
import {
  Empty,
  EmptyHeader,
  EmptyMedia,
  EmptyTitle,
} from '@/components/ui/empty'
import { RoleEnumSchema } from '@/lib/api/schemas.gen'
import { Input } from '@/components/ui/input'
import { Checkbox } from '@/components/ui/checkbox'
import { ScrollArea } from '@/components/ui/scroll-area'
import {
  Select,
  SelectContent,
  SelectItem,
  SelectTrigger,
  SelectValue,
} from '@/components/ui/select'
import { Tabs, TabsContent, TabsList, TabsTrigger } from '@/components/ui/tabs'

interface RoleSetEditorProps {
  set: RoleSet
}

function PermissionsForRoleEditor({ role }: { role: string }) {
  const queryClient = useQueryClient()

  const { data: rawPermissions, isLoading } = useQuery({
    ...rbacApi.getRolePermissionsOptions(role),
    enabled: !!role,
  })

  const assignedPermissions = React.useMemo(() => {
    const perms = rawPermissions?.permissions || []
    return perms.filter(isPermissionEnum)
  }, [rawPermissions])

  const assignPerm = useMutation({
    ...rbacApi.assignPermissionToRoleMutation(),
    onSuccess: (_, variables) => {
      queryClient.invalidateQueries({
        queryKey: rbacApi.getRolePermissionsOptions(variables.path.role_id)
          .queryKey,
      })
      toast.success('Permission assigned to role')
    },
    onError: (err) => toast.error(err.message),
  })

  const unassignPerm = useMutation({
    ...rbacApi.unassignPermissionFromRoleMutation(),
    onSuccess: (_, variables) => {
      queryClient.invalidateQueries({
        queryKey: rbacApi.getRolePermissionsOptions(variables.path.role_id)
          .queryKey,
      })
      toast.success('Permission removed from role')
    },
    onError: (err) => toast.error(err.message),
  })

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
  const queryClient = useQueryClient()
  const [isEditingInfo, setIsEditingInfo] = React.useState(false)

  React.useEffect(() => {
    setIsEditingInfo(false)
  }, [set.id])

  const { data: rawRoles, isLoading: isLoadingRoles } = useQuery({
    ...rbacApi.getRoleSetRolesOptions(set.id),
    enabled: !!set.id,
  })

  const assignedRoles: Array<string> = React.useMemo(() => {
    if (!rawRoles) return []

    // Ensure rawRoles is treated as an array of strings
    return Array.isArray(rawRoles)
      ? rawRoles.filter((role: unknown) => typeof role === 'string')
      : []
  }, [rawRoles])

  const [selectedRole, setSelectedRole] = React.useState<string | undefined>()

  React.useEffect(() => {
    if (!selectedRole && assignedRoles.length > 0) {
      setSelectedRole(assignedRoles[0])
    }
    if (assignedRoles.length === 0) {
      setSelectedRole(undefined)
    }
  }, [assignedRoles, selectedRole])

  const updateSet = useMutation({
    ...rbacApi.updateRoleSetMutation(),
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: ['getAllRoleSets'] })
      setIsEditingInfo(false)
      toast.success('Role set updated')
    },
    onError: (err) => toast.error(err.message),
  })

  const assignRole = useMutation({
    ...rbacApi.assignRoleToRoleSetMutation(),
    onSuccess: () => {
      queryClient.invalidateQueries({
        queryKey: rbacApi.getRoleSetRolesOptions(set.id).queryKey,
      })
      toast.success('Role added to set')
    },
    onError: (err) => toast.error(err.message),
  })

  const unassignRole = useMutation({
    ...rbacApi.unassignRoleFromRoleSetMutation(),
    onSuccess: () => {
      queryClient.invalidateQueries({
        queryKey: rbacApi.getRoleSetRolesOptions(set.id).queryKey,
      })
      toast.success('Role removed from set')
    },
    onError: (err) => toast.error(err.message),
  })

  const allRoles = RoleEnumSchema.enum
  const assignedRoleSet = new Set(assignedRoles)

  const [searchRoles, setSearchRoles] = React.useState<string>('')

  const filteredRoles = React.useMemo(() => {
    if (!searchRoles) return allRoles

    return allRoles.filter((role) =>
      role.toLowerCase().includes(searchRoles.toLowerCase()),
    )
  }, [searchRoles, allRoles])

  const handleSaveInfo = (values: UpdateRoleSetValues) => {
    updateSet.mutate({
      path: { role_set_id: set.id },
      body: { name: values.name, description: values.description },
    })
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
                      <Text size="sm">{assignedRoles.length}</Text> roles
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
              <span>Roles in Set</span>
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
              <CardTitle>Roles in this Set</CardTitle>
              <CardAction>
                <Badge variant="secondary" className="font-mono">
                  {assignedRoles.length} Total
                </Badge>
              </CardAction>{' '}
            </CardHeader>
            <CardContent>
              <Stack gap={2}>
                <Box className="relative">
                  <HugeiconsIcon
                    icon={Search01Icon}
                    className="absolute left-3 top-1/2 -translate-y-1/2 size-4 text-muted-foreground"
                  />
                  // eslint-disable-next-line
                  @typescript-eslint/no-unsafe-assignment
                  <Input
                    placeholder="Search roles..."
                    className="pl-9 h-10"
                    value={searchRoles || ''}
                    onChange={(e: React.ChangeEvent<HTMLInputElement>) =>
                      setSearchRoles(e.target.value)
                    }
                  />
                </Box>
                <ScrollArea className="h-[25rem] rounded-md border">
                  <Stack gap={1} p={2}>
                    {isLoadingRoles ? (
                      Array.from({ length: 8 }).map((_, i) => (
                        <Skeleton key={i} className="h-10 w-full" />
                      ))
                    ) : filteredRoles.length === 0 ? (
                      <Text className="text-center text-sm text-muted-foreground py-4">
                        No roles found.
                      </Text>
                    ) : (
                      filteredRoles.map((role) => {
                        const isAssigned = assignedRoleSet.has(role)
                        return (
                          <HStack
                            key={role}
                            align="center"
                            gap={3}
                            className="p-2 rounded-md hover:bg-muted/50 transition-colors"
                          >
                            <Checkbox
                              id={`role-${role}`}
                              checked={isAssigned}
                              onCheckedChange={(checked: boolean) => {
                                if (checked) {
                                  assignRole.mutate({
                                    path: { role_set_id: set.id },
                                    body: { role_id: role },
                                  })
                                } else {
                                  unassignRole.mutate({
                                    path: { role_set_id: set.id },
                                    body: { role_id: role },
                                  })
                                }
                              }}
                            />
                            <label
                              htmlFor={`role-${role}`}
                              className="text-sm font-medium leading-none cursor-pointer flex-1"
                            >
                              {role}
                            </label>
                          </HStack>
                        )
                      })
                    )}
                  </Stack>
                </ScrollArea>
              </Stack>
            </CardContent>
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
                  onValueChange={(v) => setSelectedRole(v ?? undefined)}
                  value={selectedRole}
                  disabled={assignedRoles.length === 0}
                >
                  <SelectTrigger className="w-[280px]">
                    <SelectValue placeholder="Select a role..." />
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
                ) : (
                  <Empty className="border-dashed rounded-lg p-8">
                    <EmptyHeader>
                      <EmptyMedia variant="icon">
                        <HugeiconsIcon icon={Shield01Icon} />
                      </EmptyMedia>
                      <EmptyTitle>No Role Selected</EmptyTitle>
                      <Text>
                        {assignedRoles.length > 0
                          ? 'Select a role to see its permissions.'
                          : 'Add roles to this set first to manage their permissions.'}
                      </Text>
                    </EmptyHeader>
                  </Empty>
                )}
              </Stack>
            </CardContent>
          </Card>
        </TabsContent>
      </Tabs>
    </Stack>
  )
}
