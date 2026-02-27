import * as React from 'react'
import { useMutation, useQuery, useQueryClient } from '@tanstack/react-query'
import { toast } from 'sonner'
import { HugeiconsIcon } from '@hugeicons/react'
import {
  Delete02Icon,
  Edit01Icon,
  Layers01Icon,
  UserGroupIcon,
  UserIcon,
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
import {
  Combobox,
  ComboboxContent,
  ComboboxEmpty,
  ComboboxInput,
  ComboboxItem,
  ComboboxList,
} from '@/components/ui/combobox'
import { Card } from '@/components/ui/card'
import { ScrollArea } from '@/components/ui/scroll-area'
import {
  Box,
  Grid,
  HStack,
  Heading,
  Stack,
  Text,
} from '@/components/primitives'
import { Skeleton } from '@/components/ui/skeleton'

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

  const assignSetToUser = useMutation({
    ...rbacApi.assignSetToStaffMutation(),
    onSuccess: () => {
      queryClient.invalidateQueries({
        queryKey: rbacApi.getSetMembersOptions(set.id).queryKey,
      })
      toast.success('User assigned to set')
    },
    onError: (err) => toast.error(err.message),
  })

  const unassignSetFromUser = useMutation({
    ...rbacApi.unassignSetFromStaffMutation(),
    onSuccess: () => {
      queryClient.invalidateQueries({
        queryKey: rbacApi.getSetMembersOptions(set.id).queryKey,
      })
      toast.success('User removed from set')
    },
    onError: (err) => toast.error(err.message),
  })

  // To add a member, we need the list of all users to pick from
  // We fetch a larger batch to support the combobox picker
  const { data: allUsersRes } = useQuery(
    rbacApi.getAllUsersOptions({ limit: 100 }),
  )
  const allUsers = allUsersRes?.data || []

  const availableUsers = allUsers.filter(
    (u) => !members.some((m) => m.id === u.id),
  )

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
      path: { permission_set_id: set.id },
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
        <Box p={6} rounded="xl" bg="bg-card" className="border shadow-sm">
          <Stack gap={3}>
            <HStack align="center" justify="between">
              <Heading size="h2" className="group">
                {set.name}
                <Button
                  variant="ghost"
                  size="icon"
                  className="size-8 ml-2 opacity-0 group-hover:opacity-100 transition-opacity"
                  onClick={() => setIsEditingInfo(true)}
                >
                  <HugeiconsIcon
                    icon={Edit01Icon}
                    className="size-4 text-primary"
                  />
                </Button>
              </Heading>
            </HStack>
            <Text size="base" muted className="max-w-3xl leading-relaxed">
              {set.description || 'No description provided.'}
            </Text>
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
          </Stack>
        </Box>
      )}

      <Grid cols={5} gap={6} className="min-h-0">
        <Card className="col-span-3 flex flex-col h-[70vh]">
          <Box p={6} className="border-b">
            <HStack justify="between">
              <Heading size="h4">Set Permissions</Heading>
              <Badge variant="secondary" className="font-mono">
                {assignedPermissions.length} Assigned
              </Badge>
            </HStack>
          </Box>
          <Box p={6} className="flex-1 min-h-0">
            <PermissionList
              assignedPermissions={assignedPermissions}
              onToggle={handleTogglePermission}
            />
          </Box>
        </Card>

        <Card className="col-span-2 flex flex-col h-[70vh]">
          <Box p={6} className="border-b">
            <Stack gap={4}>
              <HStack justify="between">
                <Heading size="h4">Assigned Users</Heading>
                <Badge variant="secondary" className="font-mono">
                  {members.length} Total
                </Badge>
              </HStack>
              <Combobox
                onValueChange={(userId) => {
                  if (typeof userId === 'string') {
                    assignSetToUser.mutate({
                      path: { staff_id: userId, set_id: set.id },
                    })
                  }
                }}
              >
                <ComboboxInput
                  placeholder="Assign a user by email..."
                  className="h-9 px-3 text-sm bg-muted/30"
                  showTrigger={true}
                />
                <ComboboxContent>
                  <ComboboxList className="text-sm">
                    {availableUsers.length === 0 ? (
                      <ComboboxEmpty>No more users available</ComboboxEmpty>
                    ) : (
                      availableUsers.map((u) => (
                        <ComboboxItem key={u.id} value={u.id}>
                          {u.email}
                        </ComboboxItem>
                      ))
                    )}
                  </ComboboxList>
                </ComboboxContent>
              </Combobox>
            </Stack>
          </Box>
          <Box className="flex-1 min-h-0">
            <ScrollArea className="h-full">
              <Stack p={6} gap={2}>
                {isLoadingMembers ? (
                  Array.from({ length: 3 }).map((_, i) => (
                    <Skeleton key={i} className="h-12" />
                  ))
                ) : members.length === 0 ? (
                  <Stack align="center" className="py-12 text-center" gap={2}>
                    <HugeiconsIcon
                      icon={UserIcon}
                      className="size-8 text-muted-foreground"
                    />
                    <Text
                      size="sm"
                      className="font-medium text-muted-foreground"
                    >
                      No users assigned
                    </Text>
                  </Stack>
                ) : (
                  members.map((member) => (
                    <HStack
                      key={member.id}
                      gap={3}
                      p={2}
                      align="center"
                      className="group/member"
                    >
                      <Box
                        p={2}
                        rounded="full"
                        className="bg-muted/70 dark:bg-zinc-800"
                      >
                        <HugeiconsIcon
                          icon={UserIcon}
                          className="size-4 text-primary"
                        />
                      </Box>
                      <Stack gap={0} className="min-w-0 flex-1">
                        <Text className="text-sm font-medium truncate">
                          {member.email}
                        </Text>
                        <Text
                          size="xs"
                          muted
                          className="uppercase font-mono tracking-tighter"
                        >
                          ID: {member.id.split('-')[0]}...
                        </Text>
                      </Stack>
                      <Button
                        variant="ghost"
                        size="icon"
                        className="size-8 opacity-0 group-hover/member:opacity-100 transition-opacity hover:bg-destructive/10"
                        onClick={() => {
                          if (
                            confirm(`Remove ${member.email} from this set?`)
                          ) {
                            unassignSetFromUser.mutate({
                              path: { staff_id: member.id, set_id: set.id },
                            })
                          }
                        }}
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
          </Box>
        </Card>
      </Grid>
    </Stack>
  )
}
