import * as React from 'react'
import { useQuery } from '@tanstack/react-query'
import { HugeiconsIcon } from '@hugeicons/react'
import {
  Add01Icon,
  Delete02Icon,
  Edit01Icon,
  Layers01Icon,
  UserGroupIcon,
} from '@hugeicons/core-free-icons'
import { updatePermissionSetSchema } from '../schemas'
import {
  getAllUserSetUserQueryOptions,
  useCreateUserSetUser,
  useDeleteUserSetUser,
  useUpdatePermissionSet,
} from '../api'
import type { UpdatePermissionSetInput as UpdatePermissionSetValues } from '../schemas'
import type { UserResponse, UserSet, UserSetUser } from '@/lib/api/types.gen'
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
import { cn } from '@/lib/utils'
import { Avatar, AvatarFallback, AvatarImage } from '@/components/ui/avatar'
import {
  Command,
  CommandEmpty,
  CommandGroup,
  CommandInput,
  CommandItem,
  CommandList,
} from '@/components/ui/command'
import {
  Popover,
  PopoverContent,
  PopoverTrigger,
} from '@/components/ui/popover'
import { authClient } from '@/lib/clients'
import { userGetAllOptions } from '@/lib/api/@tanstack/react-query.gen'

interface PermissionSetEditorProps {
  set: UserSet
}

export function PermissionSetEditor({ set }: PermissionSetEditorProps) {
  const [isEditingInfo, setIsEditingInfo] = React.useState(false)
  const [isAddingUser, setIsAddingUser] = React.useState(false)
  const [searchQuery, setSearchQuery] = React.useState('')

  React.useEffect(() => {
    setIsEditingInfo(false)
    setIsAddingUser(false)
  }, [set.id])

  const { data: membersData } = useQuery({
    ...getAllUserSetUserQueryOptions({
      query: { limit: 1000 },
    }),
    enabled: !!set.id,
  })

  const members = React.useMemo(() => {
    const all: Array<UserSetUser> = membersData?.data ?? []
    return all.filter((m) => m.user_set_id === set.id)
  }, [membersData, set.id])

  const { data: usersData } = useQuery({
    ...userGetAllOptions({
      client: authClient,
      query: { limit: 100, search: searchQuery || undefined },
    }),
    enabled: isAddingUser,
  })

  const allUsers: Array<UserResponse> = usersData?.data ?? []

  const assignedPermissions: Array<never> = []

  const updateSet = useUpdatePermissionSet()
  const addUserToSet = useCreateUserSetUser()
  const removeUserFromSet = useDeleteUserSetUser()

  const handleSaveInfo = (values: UpdatePermissionSetValues) => {
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
              <Text size="sm" muted>
                Permission assignments are not available in the current API.
              </Text>
            </CardContent>
          </Card>
        </TabsContent>

        <TabsContent value="assigned-users" className="min-h-0">
          <Card className="flex flex-col h-full">
            <CardHeader className="border-b">
              <Stack gap={2}>
                <HStack justify="between">
                  <CardTitle>Assigned Users</CardTitle>
                  <HStack gap={2}>
                    <Badge variant="secondary" className="font-mono">
                      {members.length} Total
                    </Badge>
                    <Popover open={isAddingUser} onOpenChange={setIsAddingUser}>
                      <PopoverTrigger>
                        <Button variant="outline" size="sm" className="gap-2">
                          <HugeiconsIcon icon={Add01Icon} className="size-4" />
                          Add User
                        </Button>
                      </PopoverTrigger>
                      <PopoverContent className="w-[300px] p-0" align="end">
                        <Command>
                          <CommandInput
                            placeholder="Search users..."
                            value={searchQuery}
                            onValueChange={setSearchQuery}
                          />
                          <CommandList>
                            <CommandEmpty>No users found.</CommandEmpty>
                            <CommandGroup>
                              {allUsers
                                .filter(
                                  (u) =>
                                    !members.some((m) => m.user_id === u.id),
                                )
                                .map((user) => (
                                  <CommandItem
                                    key={user.id}
                                    onSelect={() => {
                                      addUserToSet.mutate(
                                        {
                                          body: {
                                            user_id: user.id,
                                            user_set_id: set.id,
                                          },
                                        },
                                        {
                                          onSuccess: () => {
                                            setIsAddingUser(false)
                                            setSearchQuery('')
                                          },
                                        },
                                      )
                                    }}
                                  >
                                    <HStack gap={2}>
                                      <Avatar className="h-6 w-6">
                                        <AvatarImage
                                          src={`https://api.dicebear.com/7.x/avataaars/svg?seed=${user.email}`}
                                        />
                                        <AvatarFallback className="text-[10px]">
                                          {user.email
                                            .substring(0, 2)
                                            .toUpperCase()}
                                        </AvatarFallback>
                                      </Avatar>
                                      <Text size="sm">{user.email}</Text>
                                    </HStack>
                                  </CommandItem>
                                ))}
                            </CommandGroup>
                          </CommandList>
                        </Command>
                      </PopoverContent>
                    </Popover>
                  </HStack>
                </HStack>
                <ScrollArea className="h-60">
                  <Stack gap={1} p={2}>
                    {members.length === 0 ? (
                      <Text className="text-center text-sm text-muted-foreground py-4">
                        No users assigned to this permission set yet.
                      </Text>
                    ) : (
                      members.map((member) => {
                        const userId = member.user_id
                        const user = allUsers.find((u) => u.id === userId)
                        const email: string = user?.email ?? member.user_set_id
                        const name = email
                          .split('@')[0]
                          .replace(/[._]/g, ' ')
                          .replace(/\b\w/g, (l) => l.toUpperCase())
                        const initials = name.substring(0, 2).toUpperCase()

                        return (
                          <HStack
                            key={member.user_id}
                            align="center"
                            justify="between"
                            gap={3}
                            className={cn(
                              'p-2 rounded-md transition-colors hover:bg-muted/50',
                            )}
                          >
                            <HStack align="center" gap={3}>
                              <Avatar className="h-8 w-8 border border-border/50">
                                <AvatarImage
                                  src={`https://api.dicebear.com/7.x/avataaars/svg?seed=${email}`}
                                />
                                <AvatarFallback className="text-[10px] font-semibold">
                                  {initials}
                                </AvatarFallback>
                              </Avatar>
                              <Stack gap={0}>
                                <Text size="sm" className="font-medium">
                                  {email}
                                </Text>
                                <Text size="xs" muted>
                                  ID: {member.user_id}
                                </Text>
                              </Stack>
                            </HStack>
                            <Button
                              variant="ghost"
                              size="icon"
                              className="size-8 text-destructive hover:text-destructive"
                              onClick={() => {
                                // Use user_id as the identifier for deletion
                                removeUserFromSet.mutate({
                                  path: { id: member.user_id },
                                })
                              }}
                              disabled={removeUserFromSet.isPending}
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
      </Tabs>
    </Stack>
  )
}
