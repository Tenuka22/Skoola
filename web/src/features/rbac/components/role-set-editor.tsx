import * as React from 'react'
import { useQuery } from '@tanstack/react-query'
import { HugeiconsIcon } from '@hugeicons/react'
import {
  Edit01Icon,
  Layers01Icon,
  Shield01Icon,
} from '@hugeicons/core-free-icons'
import { updateRoleSetSchema } from '../schemas'
import { isPermissionEnum } from '../utils/permissions'
import {
  getRolePermissionsQueryOptions,
  useAssignPermissionToRole,
  useUnassignPermissionFromRole,
  useUpdateRoleSet,
} from '../api'
import { PermissionList } from './permission-list'
import type { UpdateRoleSetInput as UpdateRoleSetValues } from '../schemas'
import type { PermissionEnum, RoleSet } from '@/lib/api/types.gen'
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
      ? candidate.filter((p: unknown) => typeof p === 'string')
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

  React.useEffect(() => {
    setIsEditingInfo(false)
  }, [set.id])

  const allRoles = RoleEnumSchema.enum
  const [selectedRole, setSelectedRole] = React.useState<string>(allRoles[0] ?? '')

  const updateSet = useUpdateRoleSet()

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

      <Tabs defaultValue="permissions" className="flex-1 flex flex-col min-h-0">
        <TabsList className="w-fit mb-2">
          <TabsTrigger value="permissions">
            <HStack gap={2} p={0}>
              <HugeiconsIcon icon={Layers01Icon} className="size-4" />
              <span>Role Permissions</span>
            </HStack>
          </TabsTrigger>
        </TabsList>
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
                    <SelectValue placeholder="Select a role..." />
                  </SelectTrigger>
                  <SelectContent>
                    {allRoles.map((role) => (
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
