import * as React from 'react'
import { useMutation, useQuery, useQueryClient } from '@tanstack/react-query'
import { toast } from 'sonner'
import { HugeiconsIcon } from '@hugeicons/react'
import {
  Edit01Icon,
  Layers01Icon,
  UserGroupIcon,
} from '@hugeicons/core-free-icons'
import { updatePermissionSetSchema } from '../schemas'
import { rbacApi } from '../api'
import { isPermissionEnum } from '../utils/permissions'
import { PermissionList } from './permission-list'
import type { UpdatePermissionSetInput as UpdatePermissionSetValues } from '../schemas'
import type { PermissionEnum, UserSet } from '@/lib/api/types.gen'
import { FormBuilder, defineFormConfig } from '@/components/form-builder'
import { Badge } from '@/components/ui/badge'
import { Button } from '@/components/ui/button'
import { ScrollArea } from '@/components/ui/scroll-area'
import {
  Card,
  CardAction,
  CardContent,
  CardDescription,
  CardHeader,
  CardTitle,
} from '@/components/ui/card'
import { Tabs, TabsContent, TabsList, TabsTrigger } from '@/components/ui/tabs'
import { Box, HStack, Heading, Stack, Text } from '@/components/primitives'
import { Skeleton } from '@/components/ui/skeleton'
import { cn } from '@/lib/utils'
import { Avatar, AvatarFallback, AvatarImage } from '@/components/ui/avatar'

interface PermissionSetEditorProps {
  set: UserSet
}

export function PermissionSetEditor({ set }: PermissionSetEditorProps) {
  const queryClient = useQueryClient()
  const [isEditingInfo, setIsEditingInfo] = React.useState(false)

  React.useEffect(() => {
    setIsEditingInfo(false)
  }, [set.id])

  const { data: rawPermissions } = useQuery({
    ...rbacApi.getSetPermissionsOptions(set.id),
    enabled: !!set.id,
  })

  const assignedPermissions = React.useMemo(() => {
    const perms = rawPermissions?.permissions || []
    return perms.filter(isPermissionEnum)
  }, [rawPermissions])

  const { data: members = [], isLoading: isLoadingMembers } = useQuery(
    rbacApi.getSetMembersOptions(set.id),
  )

  const updateSet = useMutation({
    ...rbacApi.updateSetMutation(),
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: ['getAllPermissionSets'] })
      setIsEditingInfo(false)
      toast.success('Permission set updated')
    },
    onError: (err) => toast.error(err.message),
  })

  const assignPerm = useMutation({
    ...rbacApi.assignPermissionToSetMutation(),
    onSuccess: (_, variables) => {
      queryClient.invalidateQueries({
        queryKey: rbacApi.getSetPermissionsOptions(variables.path.user_set_id)
          .queryKey,
      })
      toast.success('Permission added')
    },
    onError: (err) => toast.error(err.message),
  })

  const unassignPerm = useMutation({
    ...rbacApi.unassignPermissionFromSetMutation(),
    onSuccess: (_, variables) => {
      queryClient.invalidateQueries({
        queryKey: rbacApi.getSetPermissionsOptions(variables.path.user_set_id)
          .queryKey,
      })
      toast.success('Permission removed')
    },
    onError: (err) => toast.error(err.message),
  })

  const handleTogglePermission = (
    permission: PermissionEnum,
    checked: boolean,
  ) => {
    if (checked) {
      assignPerm.mutate({
        path: { user_set_id: set.id },
        body: { permission },
      })
    } else {
      unassignPerm.mutate({
        path: { user_set_id: set.id },
        body: { permission },
      })
    }
  }

  const handleSaveInfo = (values: UpdatePermissionSetValues) => {
    updateSet.mutate({
      path: { user_set_id: set.id },
      body: { name: values.name, description: values.description },
    })
  }

  const formConfig = defineFormConfig(updatePermissionSetSchema, {
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
          <Box p={6} className="border-b">
            <Heading size="h3">Edit Set Details</Heading>
          </Box>
          <Box p={6}>
            <FormBuilder
              schema={updatePermissionSetSchema}
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
              className="space-y-4"
            />
          </Box>
        </Card>
      ) : (
        <Card>
          <CardHeader>
            <HStack justify="between" align="center">
              <HStack gap={3}>
                <HugeiconsIcon
                  icon={Layers01Icon}
                  className="size-11 text-primary"
                />
                <Stack gap={0}>
                  <CardTitle>{set.name}</CardTitle>
                  <HStack gap={2} align="center">
                    <Text size="xs" className="truncate" muted>
                      ID: {set.id}
                    </Text>
                  </HStack>
                </Stack>
              </HStack>

              <CardAction>
                <Button
                  variant="ghost"
                  size="icon"
                  className="size-8"
                  onClick={() => setIsEditingInfo(true)}
                >
                  <HugeiconsIcon
                    icon={Edit01Icon}
                    className="size-4 text-primary"
                  />
                </Button>
              </CardAction>
            </HStack>
            <CardDescription className="max-w-3xl leading-relaxed pt-2">
              {set.description || 'No description provided.'}
            </CardDescription>
            <HStack gap={6} className="pt-2">
              <HStack gap={2}>
                <Box className="p-1 rounded bg-primary/10">
                  <HugeiconsIcon
                    icon={Layers01Icon}
                    className="size-3.5 text-primary"
                  />
                </Box>
                <Text size="sm" muted>
                  <Text className="font-bold text-foreground" size="sm">
                    {assignedPermissions.length}
                  </Text>{' '}
                  permissions
                </Text>
              </HStack>
              <HStack gap={2}>
                <Box className="p-1 rounded bg-secondary/80">
                  <HugeiconsIcon
                    icon={UserGroupIcon}
                    className="size-3.5 text-secondary-foreground"
                  />
                </Box>
                <Text size="sm" muted>
                  <Text className="font-bold text-foreground" size="sm">
                    {members.length}
                  </Text>{' '}
                  assigned users
                </Text>
              </HStack>
            </HStack>
          </CardHeader>
        </Card>
      )}

      <Tabs defaultValue="permissions" className="flex-1 flex flex-col min-h-0">
        <TabsList className="w-fit mb-2">
          <TabsTrigger value="permissions">
            <HStack gap={2} p={0}>
              <HugeiconsIcon icon={Layers01Icon} className="size-4" />
              <span>Permissions</span>
            </HStack>
          </TabsTrigger>
          <TabsTrigger value="assigned-users">
            <HStack gap={2} p={0}>
              <HugeiconsIcon icon={UserGroupIcon} className="size-4" />
              <span>Assigned Users</span>
            </HStack>
          </TabsTrigger>
        </TabsList>

        <TabsContent value="permissions" className="min-h-0">
          <Card className="flex flex-col h-full">
            <CardHeader>
              <HStack justify="between">
                <CardTitle>Set Permissions</CardTitle>
                <Badge variant="secondary" className="font-mono">
                  {assignedPermissions.length} Assigned
                </Badge>
              </HStack>
            </CardHeader>
            <CardContent>
              <PermissionList
                assignedPermissions={assignedPermissions}
                onToggle={handleTogglePermission}
              />
            </CardContent>
          </Card>
        </TabsContent>

        <TabsContent value="assigned-users" className="min-h-0">
          <Card className="flex flex-col h-full">
            <CardHeader className="border-b">
              <Stack gap={2}>
                <HStack justify="between">
                  <CardTitle>Assigned Users</CardTitle>
                  <Badge variant="secondary" className="font-mono">
                    {members.length} Total
                  </Badge>
                </HStack>
                {isLoadingMembers ? (
                  <Stack gap={2}>
                    <Skeleton className="h-10 w-full" />
                    <Skeleton className="h-10 w-full" />
                    <Skeleton className="h-10 w-full" />
                  </Stack>
                ) : (
                  <ScrollArea className="h-60">
                    <Stack gap={1} p={2}>
                      {members.length === 0 ? (
                        <Text className="text-center text-sm text-muted-foreground py-4">
                          No users assigned to this set.
                        </Text>
                      ) : (
                        members.map((user) => {
                          const name = user.email
                            .split('@')[0]
                            .replace(/[._]/g, ' ')
                            .replace(/\b\w/g, (l) => l.toUpperCase())
                          const initials = name.substring(0, 2).toUpperCase()

                          return (
                            <HStack
                              key={user.id}
                              align="center"
                              gap={3}
                              className={cn(
                                'p-2 rounded-md transition-colors hover:bg-muted/50',
                              )}
                            >
                              <Avatar className="h-8 w-8 border border-border/50">
                                <AvatarImage
                                  src={`https://api.dicebear.com/7.x/avataaars/svg?seed=${user.email}`}
                                />
                                <AvatarFallback className="text-[10px] font-semibold">
                                  {initials}
                                </AvatarFallback>
                              </Avatar>
                              <label
                                htmlFor={`user-${user.id}`}
                                className="text-sm font-medium leading-none cursor-pointer"
                              >
                                {user.email}
                              </label>
                            </HStack>
                          )
                        })
                      )}
                    </Stack>
                  </ScrollArea>
                )}
              </Stack>
            </CardHeader>
          </Card>
        </TabsContent>
      </Tabs>
    </Stack>
  )
}
