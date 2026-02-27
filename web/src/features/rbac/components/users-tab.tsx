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
import { Input } from '@/components/ui/input'
import { ScrollArea } from '@/components/ui/scroll-area'
import { Badge } from '@/components/ui/badge'
import { cn } from '@/lib/utils'
import { Button } from '@/components/ui/button'
import { Box, HStack, Heading, Stack, Text } from '@/components/primitives'
import { Skeleton } from '@/components/ui/skeleton'

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
    <div className="h-full flex flex-col overflow-hidden rounded-xl border border-border/60 bg-background shadow-sm">
      <HStack className="h-full" align="start">
        {/* Left Panel: Users List */}
        <Stack gap={4} className="w-[350px] shrink-0 h-full border-r">
          <Stack gap={1} p={4} className="border-b">
            <Heading size="h4">Users Directory</Heading>
            <Text size="sm" muted>
              Select a user to manage their access.
            </Text>
          </Stack>

          <Box px={4}>
            <Box className="relative">
              <HugeiconsIcon
                icon={Search01Icon}
                className="absolute left-3 top-1/2 -translate-y-1/2 size-4 text-muted-foreground"
              />
              <Input
                placeholder="Search users by email..."
                className="pl-9"
                value={search}
                onChange={(e) => setSearch(e.target.value)}
              />
            </Box>
          </Box>

          <ScrollArea className="flex-1">
            <Stack p={4} className="pt-0">
              {isLoading ? (
                <Stack gap={2}>
                  {Array.from({ length: PAGE_SIZE }).map((_, i) => (
                    <Skeleton key={i} className="h-16 rounded-lg" />
                  ))}
                </Stack>
              ) : users.length === 0 ? (
                <Stack
                  align="center"
                  className="justify-center py-12 text-center"
                  gap={2}
                >
                  <HugeiconsIcon
                    icon={UserGroupIcon}
                    className="size-8 text-muted-foreground"
                  />
                  <Text size="sm" className="font-medium text-muted-foreground">
                    No users found
                  </Text>
                </Stack>
              ) : (
                <Stack gap={2}>
                  {users.map((user) => (
                    <button
                      key={user.id}
                      onClick={() => setSelectedUserId(user.id)}
                      className={cn(
                        'w-full text-left p-3 rounded-lg transition-colors',
                        selectedUserId === user.id
                          ? 'bg-muted'
                          : 'hover:bg-muted/50',
                      )}
                    >
                      <HStack justify="between">
                        <Stack gap={0} className="min-w-0">
                          <Text className="font-semibold text-sm truncate">
                            {user.email}
                          </Text>
                          <Text
                            size="xs"
                            muted
                            className="font-mono truncate opacity-70"
                          >
                            ID: {user.id}
                          </Text>
                        </Stack>
                        <Badge
                          variant={
                            selectedUserId === user.id ? 'default' : 'secondary'
                          }
                          className="text-[10px] h-5 font-mono uppercase"
                        >
                          {user.role}
                        </Badge>
                      </HStack>
                    </button>
                  ))}
                </Stack>
              )}
            </Stack>
          </ScrollArea>

          {totalPages > 1 && (
            <HStack align="center" justify="between" p={4} className="border-t">
              <Button
                variant="outline"
                size="sm"
                onClick={() => setPage((p) => Math.max(0, p - 1))}
                disabled={page === 0}
              >
                Previous
              </Button>
              <Text size="sm" muted>
                Page {page + 1} of {totalPages}
              </Text>
              <Button
                variant="outline"
                size="sm"
                onClick={() => setPage((p) => p + 1)}
                disabled={page + 1 >= totalPages}
              >
                Next
              </Button>
            </HStack>
          )}
        </Stack>

        {/* Right Panel: User Editor */}
        <Box className="flex-1 h-full overflow-y-auto">
          <Box p={6}>
            {selectedUser ? (
              <UserPermissionEditor user={selectedUser} key={selectedUser.id} />
            ) : isLoading ? (
              <Stack gap={4}>
                <Skeleton className="h-24 w-full" />
                <Skeleton className="h-64 w-full" />
                <Skeleton className="h-64 w-full" />
              </Stack>
            ) : (
              <Stack
                align="center"
                justify="center"
                className="h-[60vh] text-center"
                gap={2}
              >
                <HugeiconsIcon
                  icon={UserIcon}
                  className="size-12 text-muted-foreground/50"
                />
                <Heading size="h3">No User Selected</Heading>
                <Text muted>
                  Select a user from the directory to manage them.
                </Text>
              </Stack>
            )}
          </Box>
        </Box>
      </HStack>
    </div>
  )
}
