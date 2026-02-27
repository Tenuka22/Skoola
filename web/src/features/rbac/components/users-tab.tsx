import * as React from 'react'
import { useQuery } from '@tanstack/react-query'
import { HugeiconsIcon } from '@hugeicons/react'
import {
  Search01Icon,
  UserGroupIcon,
  UserIcon,
} from '@hugeicons/core-free-icons'
import { useRBACStore } from '../store'
import { UserPermissionEditor } from './user-permission-editor'
import { authClient } from '@/lib/clients'
import { getAllUsersOptions } from '@/lib/api/@tanstack/react-query.gen'
import { Input } from '@/components/ui/input'
import { ScrollArea } from '@/components/ui/scroll-area'
import { Badge } from '@/components/ui/badge'
import { cn } from '@/lib/utils'
import { Button } from '@/components/ui/button'

const PAGE_SIZE = 10 // Define page size

export function UsersTab() {
  const [search, setSearch] = React.useState('')
  const { selectedUserId, setSelectedUserId } = useRBACStore()
  const [page, setPage] = React.useState(0) // Add page state

  const { data: usersData, isLoading } = useQuery(
    getAllUsersOptions({
      client: authClient,
      query: {
        limit: PAGE_SIZE, // Use PAGE_SIZE
        page: page, // Use 'page' parameter for pagination
        search: search || undefined,
      },
    }),
  )

  const users = usersData?.data || []
  const totalUsers = usersData?.total || 0 // Get total users from response
  const selectedUser = users.find((u) => u.id === selectedUserId)

  const handlePreviousPage = () => {
    setPage((prev) => Math.max(0, prev - 1))
  }

  const handleNextPage = () => {
    setPage((prev) => prev + 1)
  }

  const totalPages = Math.ceil(totalUsers / PAGE_SIZE)

  // Reset selected user when page or search changes
  React.useEffect(() => {
    setSelectedUserId(null)
  }, [page, search, setSelectedUserId])

  return (
    <div className="flex h-full gap-6 overflow-hidden">
      {/* Left Panel: User List (40%) */}
      <div className="flex flex-col w-[400px] shrink-0 gap-4 p-2">
        <div className="flex flex-col gap-1.5">
          <h2 className="text-sm font-semibold flex items-center gap-2">
            <HugeiconsIcon
              icon={UserGroupIcon}
              className="size-4 text-primary"
            />
            Select User
          </h2>
          <p className="text-xs text-muted-foreground">
            Search and select a user to manage their access.
          </p>
        </div>

        <div className="relative">
          <HugeiconsIcon
            icon={Search01Icon}
            className="absolute left-3 top-1/2 -translate-y-1/2 size-4 text-muted-foreground"
          />
          <Input
            placeholder="Search users..."
            className="pl-9 h-11"
            value={search}
            onChange={(e) => {
              setSearch(e.target.value)
              setPage(0) // Reset page on search
            }}
          />
        </div>

        <ScrollArea className="flex-1 -mx-2 px-2">
          <div className="space-y-1.5 pb-4">
            {isLoading ? (
              Array.from({ length: PAGE_SIZE }).map((_, i) => (
                <div
                  key={i}
                  className="h-16 rounded-xl bg-muted animate-pulse mb-2"
                />
              ))
            ) : users.length === 0 ? (
              <div className="flex flex-col items-center justify-center py-12 text-muted-foreground">
                <HugeiconsIcon
                  icon={UserGroupIcon}
                  className="size-8 mb-3 opacity-20"
                />
                <p className="text-sm font-medium">No users found</p>
                <p className="text-xs opacity-60">
                  Try a different search term
                </p>
              </div>
            ) : (
              users.map((user) => (
                <button
                  key={user.id}
                  onClick={() => setSelectedUserId(user.id)}
                  className={cn(
                    'w-full flex flex-col items-start gap-1 p-2 text-left transition-all relative overflow-hidden group',
                    selectedUserId === user.id ? 'bg-primary/5' : '',
                  )}
                >
                  {selectedUserId === user.id && (
                    <div className="absolute left-0 top-0 bottom-0 w-1 bg-primary" />
                  )}
                  <div className="flex items-center justify-between w-full">
                    <span className="font-semibold text-[13px] truncate max-w-[200px]">
                      {user.email}
                    </span>
                    <Badge
                      variant={
                        selectedUserId === user.id ? 'default' : 'secondary'
                      }
                      className="text-[10px] px-1.5 h-5 font-mono uppercase tracking-tight"
                    >
                      {user.role}
                    </Badge>
                  </div>
                  <div className="flex items-center gap-1.5 text-xs text-muted-foreground mt-0.5">
                    <span className="truncate opacity-70">ID: {user.id}</span>
                  </div>
                </button>
              ))
            )}
          </div>
        </ScrollArea>
        {/* Pagination Controls */}
        {totalPages > 1 && (
          <div className="flex items-center justify-between px-2 pt-2">
            <Button
              variant="outline"
              size="sm"
              onClick={handlePreviousPage}
              disabled={page === 0}
            >
              Previous
            </Button>
            <span className="text-sm text-muted-foreground">
              Page {page + 1} of {totalPages}
            </span>
            <Button
              variant="outline"
              size="sm"
              onClick={handleNextPage}
              disabled={page + 1 >= totalPages}
            >
              Next
            </Button>
          </div>
        )}
      </div>

      {/* Right Panel: Permission Editor (Rest) */}
      <div className="flex-1 overflow-hidden h-full">
        {selectedUser ? (
          <UserPermissionEditor user={selectedUser} />
        ) : (
          <div className="flex flex-col items-center justify-center h-full text-muted-foreground">
            <div className="size-16 flex items-center justify-center mb-6">
              <HugeiconsIcon icon={UserIcon} className="size-10 opacity-20" />
            </div>
            <h3 className="text-xl font-semibold text-foreground">
              No User Selected
            </h3>
            <p className="text-sm max-w-[280px] text-center mt-2 leading-relaxed opacity-70">
              Select a user from the directory on the left to review and manage
              their individual permissions and system roles.
            </p>
          </div>
        )}
      </div>
    </div>
  )
}
