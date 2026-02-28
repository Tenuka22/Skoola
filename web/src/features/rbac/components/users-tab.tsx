import * as React from 'react'
import { useQuery } from '@tanstack/react-query'
import { HugeiconsIcon } from '@hugeicons/react'
import {
  Search01Icon,
  UserGroupIcon,
  UserIcon,
} from '@hugeicons/core-free-icons'
import { useRBACStore } from '../store'
import { rbacApi } from '../api'
import { UserPermissionEditor } from './user-permission-editor'
import { useDebounce } from '@/hooks/use-debounce'
import { Badge } from '@/components/ui/badge'
import { cn } from '@/lib/utils'
import { Button } from '@/components/ui/button'
import { Box, HStack, Stack, Text } from '@/components/primitives'
import { Skeleton } from '@/components/ui/skeleton'
import {
  Empty,
  EmptyDescription,
  EmptyHeader,
  EmptyMedia,
  EmptyTitle,
} from '@/components/ui/empty'
import {
  InputGroup,
  InputGroupAddon,
  InputGroupInput,
} from '@/components/ui/input-group'
import { Avatar, AvatarFallback, AvatarImage } from '@/components/ui/avatar'
import {
  Card,
  CardDescription,
  CardHeader,
  CardTitle,
} from '@/components/ui/card'

const PAGE_SIZE = 15

export function UsersTab() {
  const [search, setSearch] = React.useState('')
  const debouncedSearch = useDebounce(search, 300)
  const { selectedUserId, setSelectedUserId } = useRBACStore()
  const [page, setPage] = React.useState(0)

  const { data: usersData, isLoading } = useQuery(
    rbacApi.getAllUsersOptions({
      limit: PAGE_SIZE,
      page,
      search: debouncedSearch || undefined,
    }),
  )

  const users = React.useMemo(() => usersData?.data || [], [usersData])
  const totalUsers = usersData?.total || 0
  const selectedUser = users.find((u) => u.id === selectedUserId)
  const totalPages = Math.ceil(totalUsers / PAGE_SIZE)

  React.useEffect(() => {
    setPage(0)
    setSelectedUserId(null)
  }, [debouncedSearch, setSelectedUserId])

  React.useEffect(() => {
    if (users.length > 0 && !selectedUser) {
      setSelectedUserId(users[0].id)
    }
  }, [users, selectedUser, setSelectedUserId])

  return (
    <Card>
      <CardHeader>
        <HStack className="h-full" align="start" p={0} gap={0}>
          <Stack gap={4}>
            <Stack gap={0} p={0}>
              <CardTitle>Users Directory</CardTitle>
              <CardDescription>Manage granular access.</CardDescription>
            </Stack>

            <Box p={0}>
              <InputGroup>
                <InputGroupInput
                  placeholder="Search email..."
                  value={search}
                  onChange={(e) => setSearch(e.target.value)}
                  className="h-9 text-xs"
                />
                <InputGroupAddon align="inline-start">
                  <HugeiconsIcon icon={Search01Icon} className="size-3.5" />
                </InputGroupAddon>
              </InputGroup>
            </Box>

            <Stack gap={1} p={0} className="min-w-72">
              {isLoading ? (
                <Stack gap={2} p={2}>
                  {Array.from({ length: 8 }).map((_, i) => (
                    <Skeleton key={i} className="h-12 rounded-lg" />
                  ))}
                </Stack>
              ) : users.length === 0 ? (
                <Empty className="border-0 w-full justify-center py-12">
                  <EmptyHeader>
                    <EmptyMedia variant="icon">
                      <HugeiconsIcon icon={UserGroupIcon} />
                    </EmptyMedia>
                    <EmptyTitle className="text-sm">No users found</EmptyTitle>
                  </EmptyHeader>
                </Empty>
              ) : (
                users.map((user) => {
                  const name = user.email
                    .split('@')[0]
                    .replace(/[._]/g, ' ')
                    .replace(/\b\w/g, (l) => l.toUpperCase())
                  const initials = name.substring(0, 2).toUpperCase()

                  return (
                    <Button
                      variant={
                        selectedUserId === user.id ? 'secondary' : 'ghost'
                      }
                      key={user.id}
                      onClick={() => setSelectedUserId(user.id)}
                      className={cn('w-full justify-start h-11')}
                    >
                      <HStack justify="between" p={1} className="w-full">
                        <HStack>
                          <Avatar className="h-8 w-8 border border-border/50">
                            <AvatarImage
                              src={`https://api.dicebear.com/7.x/avataaars/svg?seed=${user.email}`}
                            />
                            <AvatarFallback className="text-[10px] font-semibold">
                              {initials}
                            </AvatarFallback>
                          </Avatar>
                          <Stack gap={0} className="text-left">
                            <Text size="sm" className="capitalize truncate">
                              {user.email.split('@')[0]}
                            </Text>
                            <Text
                              size="xs"
                              muted
                              className="truncate font-mono"
                            >
                              {user.email.split('@')[1]}
                            </Text>
                          </Stack>
                        </HStack>
                        <Badge
                          variant={
                            selectedUserId === user.id ? 'default' : 'outline'
                          }
                          className="text-[10px] h-4 px-1 uppercase font-bold shrink-0 ml-2"
                        >
                          {user.role === 'FullAdmin' ? 'Admin' : user.role}
                        </Badge>
                      </HStack>
                    </Button>
                  )
                })
              )}
            </Stack>

            {totalPages > 1 && (
              <HStack
                align="center"
                justify="between"
                p={3}
                className="border-t"
              >
                <Button
                  variant="outline"
                  onClick={() => setPage((p) => Math.max(0, p - 1))}
                  disabled={page === 0}
                >
                  Prev
                </Button>
                <Text className="text-[10px] font-medium" muted>
                  {page + 1} / {totalPages}
                </Text>
                <Button
                  variant="outline"
                  onClick={() => setPage((p) => p + 1)}
                  disabled={page + 1 >= totalPages}
                >
                  Next
                </Button>
              </HStack>
            )}
          </Stack>

          <Box className="flex-1 h-full" p={4}>
            {selectedUser ? (
              <UserPermissionEditor user={selectedUser} key={selectedUser.id} />
            ) : isLoading ? (
              <Stack gap={4}>
                <Skeleton className="h-16 w-full" />
                <Skeleton className="h-64 w-full" />
              </Stack>
            ) : (
              <Empty className="border-0 w-full h-full justify-center">
                <EmptyHeader>
                  <EmptyMedia variant="icon">
                    <HugeiconsIcon
                      icon={UserIcon}
                      className="size-12 opacity-20"
                    />
                  </EmptyMedia>
                  <EmptyTitle className="text-lg">Select a user</EmptyTitle>
                  <EmptyDescription className="text-sm">
                    Pick a directory entry to manage permissions.
                  </EmptyDescription>
                </EmptyHeader>
              </Empty>
            )}
          </Box>
        </HStack>
      </CardHeader>
    </Card>
  )
}
