import {
  Copy01Icon,
  Delete02Icon,
  LockIcon,
  MoreHorizontalCircle01Icon,
  PencilEdit01Icon,
  Shield02Icon,
  UserCheckIcon,
} from '@hugeicons/core-free-icons'
import { HugeiconsIcon } from '@hugeicons/react'
import { toast } from 'sonner'
import { format } from 'date-fns'
import type { Row } from '@tanstack/react-table'
import type { User } from '../types'
import type { DataTableColumnDef } from '@/components/data-table'
import { Avatar, AvatarFallback, AvatarImage } from '@/components/ui/avatar'
import { Button } from '@/components/ui/button'
import { Badge } from '@/components/ui/badge'
import { cn } from '@/lib/utils'
import {
  DropdownMenu,
  DropdownMenuContent,
  DropdownMenuItem,
  DropdownMenuSeparator,
  DropdownMenuTrigger,
} from '@/components/ui/dropdown-menu'
import {
  ContextMenuItem,
  ContextMenuSeparator,
} from '@/components/ui/context-menu'
import { Spinner } from '@/components/ui/spinner'
import { HStack, Stack, Text } from '@/components/primitives'
import { DataTableColumnHeader } from '@/components/data-table'

interface GetColumnsProps {
  users?: Array<User>
  onToggleVerify: (user: User) => void
  onToggleLock: (user: User) => void
  setUserToDelete: (id: string | null) => void
  setUserToEdit: (user: User | null) => void
  setUserToManagePermissions: (user: User | null) => void
  isUpdating?: boolean
  updatingUserId?: string | null
  showProfilePictures?: boolean
}

export function UserContextMenuItems({
  user,
  onToggleVerify,
  onToggleLock,
  setUserToDelete,
  setUserToEdit,
  setUserToManagePermissions,
  isUpdating,
  updatingUserId,
}: Omit<GetColumnsProps, 'users' | 'showProfilePictures'> & { user: User }) {
  const isLocked =
    user.lockout_until && new Date(user.lockout_until) > new Date()
  const isBeingUpdated = isUpdating && updatingUserId === user.id

  return (
    <>
      <div className="px-2 py-1.5 text-sm font-medium text-muted-foreground truncate max-w-xs">
        {user.email}
      </div>
      <ContextMenuSeparator />

      <ContextMenuItem
        onClick={() => {
          navigator.clipboard.writeText(user.id)
          toast.success('User ID copied to clipboard')
        }}
      >
        <HStack gap={2} p={0}>
          <HugeiconsIcon icon={Copy01Icon} className="h-4 w-4" />
          <span>Copy ID</span>
        </HStack>
      </ContextMenuItem>

      <ContextMenuItem
        onClick={() => {
          navigator.clipboard.writeText(user.email)
          toast.success('User email copied to clipboard')
        }}
      >
        <HStack gap={2} p={0}>
          <HugeiconsIcon icon={Copy01Icon} className="h-4 w-4" />
          <span>Copy Email</span>
        </HStack>
      </ContextMenuItem>

      <ContextMenuSeparator />

      <ContextMenuItem onClick={() => setUserToEdit(user)}>
        <HStack gap={2} p={0}>
          <HugeiconsIcon icon={PencilEdit01Icon} className="h-4 w-4" />
          <span>Edit</span>
        </HStack>
      </ContextMenuItem>
      <ContextMenuItem onClick={() => setUserToManagePermissions(user)}>
        <HStack gap={2} p={0}>
          <HugeiconsIcon icon={Shield02Icon} className="h-4 w-4" />
          <span>Manage Permissions</span>
        </HStack>
      </ContextMenuItem>

      <ContextMenuSeparator />

      <ContextMenuItem
        onSelect={(e) => e.preventDefault()}
        onClick={() => {
          onToggleVerify(user)
        }}
        disabled={isBeingUpdated}
      >
        <HStack gap={2} p={0}>
          {isBeingUpdated ? (
            <Spinner className="h-4 w-4" />
          ) : (
            <HugeiconsIcon icon={UserCheckIcon} className="h-4 w-4" />
          )}
          <span>{user.is_verified ? 'Unverify' : 'Verify'}</span>
        </HStack>
      </ContextMenuItem>

      <ContextMenuItem
        onClick={() => onToggleLock(user)}
        disabled={isBeingUpdated}
      >
        <HStack gap={2} p={0}>
          {isBeingUpdated ? (
            <Spinner className="h-4 w-4" />
          ) : (
            <HugeiconsIcon icon={LockIcon} className="h-4 w-4" />
          )}
          <span>{isLocked ? 'Unlock' : 'Lock'}</span>
        </HStack>
      </ContextMenuItem>

      <ContextMenuSeparator />

      <ContextMenuItem
        onClick={() => setUserToDelete(user.id)}
        variant="destructive"
      >
        <HStack gap={2} p={0}>
          <HugeiconsIcon icon={Delete02Icon} className="h-4 w-4" />
          <span>Delete</span>
        </HStack>
      </ContextMenuItem>
    </>
  )
}

export function getUserColumns({
  onToggleVerify,
  setUserToDelete,
  setUserToEdit,
  onToggleLock,
  setUserToManagePermissions,
  isUpdating,
  updatingUserId,
  showProfilePictures = true,
}: GetColumnsProps): Array<DataTableColumnDef<User>> {
  return [
    {
      accessorKey: 'email',
      header: ({ column }) => (
        <DataTableColumnHeader column={column} title="User Info" />
      ),
      cell: ({ row }) => {
        const user = row.original
        const name = user.email
          .split('@')[0]
          .replace(/[._]/g, ' ')
          .replace(/\b\w/g, (l) => l.toUpperCase())
        const initials = name.substring(0, 2).toUpperCase()

        return (
          <HStack gap={3}>
            {showProfilePictures && (
              <Avatar className="h-9 w-9 border border-border/50">
                <AvatarImage
                  src={`https://api.dicebear.com/7.x/avataaars/svg?seed=${user.email}`}
                />
                <AvatarFallback className="bg-primary/10 text-primary text-xs font-medium">
                  {initials}
                </AvatarFallback>
              </Avatar>
            )}
            <Stack gap={0}>
              <HStack gap={2} align="center">
                <Text
                  size="sm"
                  className="font-medium text-foreground capitalize"
                >
                  {name}
                </Text>
                <Badge
                  variant="secondary"
                  className="text-[10px] px-1.5 py-0 h-4"
                >
                  {user.role}
                </Badge>
              </HStack>
              <Text size="xs" muted>
                {user.email}
              </Text>
            </Stack>
          </HStack>
        )
      },
      meta: { isPinned: 'left' },
    },
    {
      accessorKey: 'is_verified',
      header: ({ column }) => (
        <DataTableColumnHeader column={column} title="Verification" />
      ),
      cell: ({ row }) => {
        const isVerified = row.original.is_verified

        return (
          <HStack gap={2}>
            <span
              className={cn('inline-flex h-2 w-2 rounded-full', {
                'bg-green-500': isVerified,
                'bg-red-500': !isVerified,
              })}
            />
            <Text
              size="xs"
              className={cn('font-medium', {
                'text-green-600': isVerified,
                'text-red-600': !isVerified,
              })}
            >
              {isVerified ? 'Verified' : 'Unverified'}
            </Text>
          </HStack>
        )
      },
    },
    {
      accessorKey: 'auth_method',
      header: ({ column }) => (
        <DataTableColumnHeader column={column} title="Auth Method" />
      ),
      cell: ({ row }: { row: Row<User> }) => {
        const method = row.original.auth_method
        return (
          <Badge variant="outline" className="capitalize">
            {method || 'Password'}
          </Badge>
        )
      },
      enableSorting: false,
    },
    {
      accessorKey: 'lockout_until',
      header: ({ column }) => (
        <DataTableColumnHeader column={column} title="Lock Status" />
      ),
      cell: ({ row }) => {
        const lockoutUntil = row.original.lockout_until
        const isLocked = lockoutUntil && new Date(lockoutUntil) > new Date()

        return (
          <Stack gap={1}>
            <HStack gap={2}>
              <span
                className={cn('inline-flex h-2 w-2 rounded-full', {
                  'bg-amber-500': isLocked,
                  'bg-muted-foreground': !isLocked,
                })}
              />
              <Text
                size="xs"
                className={cn('font-medium', {
                  'text-amber-600': isLocked,
                  'text-muted-foreground': !isLocked,
                })}
              >
                {isLocked ? 'Locked' : 'Unlocked'}
              </Text>
            </HStack>

            {isLocked && lockoutUntil && (
              <Text size="xs" muted className="whitespace-nowrap">
                Until {format(new Date(lockoutUntil), 'd MMM yyyy')}
              </Text>
            )}
          </Stack>
        )
      },
      enableSorting: false,
    },
    {
      accessorKey: 'created_at',
      header: ({ column }) => (
        <DataTableColumnHeader column={column} title="Joined date" />
      ),
      cell: ({ row }) => (
        <span className="text-sm text-muted-foreground">
          {format(new Date(row.getValue('created_at')), 'd MMM yyyy, h:mm a')}
        </span>
      ),
    },
    {
      accessorKey: 'updated_at',
      header: ({ column }) => (
        <DataTableColumnHeader column={column} title="Last Updated" />
      ),
      cell: ({ row }) => (
        <span className="text-sm text-muted-foreground">
          {format(new Date(row.getValue('updated_at')), 'd MMM yyyy, h:mm a')}
        </span>
      ),
      enableSorting: false,
    },
    {
      id: 'row-actions',
      header: 'Actions',
      cell: ({ row }) => {
        const user = row.original
        const isLocked =
          user.lockout_until && new Date(user.lockout_until) > new Date()
        const isBeingUpdated = isUpdating && updatingUserId === user.id

        return (
          <DropdownMenu>
            <DropdownMenuTrigger
              render={
                <Button
                  variant="ghost"
                  className="flex h-8 w-8 p-0 data-[state=open]:bg-muted"
                >
                  <HugeiconsIcon
                    icon={MoreHorizontalCircle01Icon}
                    className="h-4 w-4"
                  />
                  <span className="sr-only">Open menu</span>
                </Button>
              }
            />

            <DropdownMenuContent align="end" className="min-w-40 ">
              <div className="px-2 py-1.5 text-sm font-medium text-muted-foreground truncate max-w-xs">
                {user.email}
              </div>
              <DropdownMenuSeparator />

              <DropdownMenuItem
                onClick={() => {
                  navigator.clipboard.writeText(user.id)
                  toast.success('User ID copied to clipboard')
                }}
              >
                <HStack gap={2} p={0}>
                  <HugeiconsIcon icon={Copy01Icon} className="h-4 w-4" />
                  <span>Copy ID</span>
                </HStack>
              </DropdownMenuItem>

              <DropdownMenuItem
                onClick={() => {
                  navigator.clipboard.writeText(user.email)
                  toast.success('User email copied to clipboard')
                }}
              >
                <HStack gap={2} p={0}>
                  <HugeiconsIcon icon={Copy01Icon} className="h-4 w-4" />
                  <span>Copy Email</span>
                </HStack>
              </DropdownMenuItem>

              <DropdownMenuSeparator />

              <DropdownMenuItem onClick={() => setUserToEdit(user)}>
                <HStack gap={2} p={0}>
                  <HugeiconsIcon icon={PencilEdit01Icon} className="h-4 w-4" />
                  <span>Edit</span>
                </HStack>
              </DropdownMenuItem>
              <DropdownMenuItem
                onClick={() => setUserToManagePermissions(user)}
              >
                <HStack gap={2} p={0}>
                  <HugeiconsIcon icon={Shield02Icon} className="h-4 w-4" />
                  <span>Manage Permissions</span>
                </HStack>
              </DropdownMenuItem>

              <DropdownMenuSeparator />

              <DropdownMenuItem
                onSelect={(e) => e.preventDefault()}
                onClick={() => {
                  if (typeof user !== 'function') {
                    onToggleVerify(user)
                  }
                }}
                disabled={isBeingUpdated}
                closeOnClick={false}
              >
                <HStack gap={2} p={0}>
                  {isBeingUpdated ? (
                    <Spinner className="h-4 w-4" />
                  ) : (
                    <HugeiconsIcon icon={UserCheckIcon} className="h-4 w-4" />
                  )}
                  <span>{user.is_verified ? 'Unverify' : 'Verify'}</span>
                </HStack>
              </DropdownMenuItem>

              <DropdownMenuItem
                onClick={() => onToggleLock(user)}
                disabled={isBeingUpdated}
                closeOnClick={false}
              >
                <HStack gap={2} p={0}>
                  {isBeingUpdated ? (
                    <Spinner className="h-4 w-4" />
                  ) : (
                    <HugeiconsIcon icon={LockIcon} className="h-4 w-4" />
                  )}
                  <span>{isLocked ? 'Unlock' : 'Lock'}</span>
                </HStack>
              </DropdownMenuItem>

              <DropdownMenuSeparator />

              <DropdownMenuItem
                onClick={() => setUserToDelete(user.id)}
                className="text-destructive focus:text-destructive"
              >
                <HStack gap={2} p={0}>
                  <HugeiconsIcon icon={Delete02Icon} className="h-4 w-4" />
                  <span>Delete</span>
                </HStack>
              </DropdownMenuItem>
            </DropdownMenuContent>
          </DropdownMenu>
        )
      },
      meta: { isPinned: 'right' },
    },
  ]
}
