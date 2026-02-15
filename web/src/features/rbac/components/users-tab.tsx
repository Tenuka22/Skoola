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

export function UsersTab() {
  const [search, setSearch] = React.useState('')
  const { selectedUserId, setSelectedUserId } = useRBACStore()

  const { data: usersData, isLoading } = useQuery(
    getAllUsersOptions({
      client: authClient,
      query: {
        limit: 100, // For now, list more users for easier selection
        search: search || undefined,
      },
    }),
  )

  const users = usersData?.data || []
  const selectedUser = users.find((u) => u.id === selectedUserId)

  return (
    <div className="flex h-[calc(100vh-200px)] gap-6 overflow-hidden">
      {/* Left Panel: User List (60%) */}
      <div className="flex flex-col w-[60%] gap-4 border rounded-xl bg-card p-4">
        <div className="relative">
          <HugeiconsIcon
            icon={Search01Icon}
            className="absolute left-3 top-1/2 -translate-y-1/2 size-4 text-muted-foreground"
          />
          <Input
            placeholder="Search users..."
            className="pl-9 h-10"
            value={search}
            onChange={(e) => setSearch(e.target.value)}
          />
        </div>

        <ScrollArea className="flex-1 -mx-2 px-2">
          <div className="space-y-1">
            {isLoading ? (
              Array.from({ length: 5 }).map((_, i) => (
                <div
                  key={i}
                  className="h-16 rounded-lg bg-muted animate-pulse mb-2"
                />
              ))
            ) : users.length === 0 ? (
              <div className="flex flex-col items-center justify-center h-40 text-muted-foreground">
                <HugeiconsIcon
                  icon={UserGroupIcon}
                  className="size-8 mb-2 opacity-20"
                />
                <p className="text-sm">No users found</p>
              </div>
            ) : (
              users.map((user) => (
                <button
                  key={user.id}
                  onClick={() => setSelectedUserId(user.id)}
                  className={cn(
                    'w-full flex flex-col items-start gap-1 p-3 rounded-lg text-left transition-colors',
                    selectedUserId === user.id
                      ? 'bg-primary/10 border-primary/20 border'
                      : 'hover:bg-muted/50 border-transparent border',
                  )}
                >
                  <div className="flex items-center justify-between w-full">
                    <span className="font-semibold text-sm truncate max-w-[300px]">
                      {user.email}
                    </span>
                    <Badge
                      variant="secondary"
                      className="text-[10px] px-1.5 h-5"
                    >
                      {user.role}
                    </Badge>
                  </div>
                  <span className="text-xs text-muted-foreground truncate w-full">
                    ID: {user.id}
                  </span>
                </button>
              ))
            )}
          </div>
        </ScrollArea>
      </div>

      {/* Right Panel: Permission Editor (40%) */}
      <div className="flex-1 overflow-hidden">
        {selectedUser ? (
          <UserPermissionEditor user={selectedUser} />
        ) : (
          <div className="flex flex-col items-center justify-center h-full text-muted-foreground border-2 border-dashed rounded-xl bg-muted/5">
            <HugeiconsIcon
              icon={UserIcon}
              className="size-12 mb-4 opacity-10"
            />
            <h3 className="text-lg font-medium">No User Selected</h3>
            <p className="text-sm max-w-[250px] text-center mt-1">
              Select a user from the list on the left to manage their roles and
              permissions.
            </p>
          </div>
        )}
      </div>
    </div>
  )
}
