import {
  ArrowDown01Icon,
  ArrowUp01Icon,
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
import type { ColumnDef } from '@tanstack/react-table'
import type { User } from '../types'
import { Avatar, AvatarFallback, AvatarImage } from '@/components/ui/avatar'
import { Button } from '@/components/ui/button'
import { Checkbox } from '@/components/ui/checkbox'
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
}: GetColumnsProps): Array<ColumnDef<User>> {
  return [
    {
      id: 'select',
      header: ({ table }) => (
        <div className="pl-4">
          <Checkbox
            checked={table.getIsAllPageRowsSelected()}
            indeterminate={
              !table.getIsAllPageRowsSelected() && table.getIsSomeRowsSelected()
            }
            onCheckedChange={(value) => {
              table.toggleAllPageRowsSelected(!!value)
            }}
          />
        </div>
      ),
      cell: ({ row }) => (
        <div className="pl-4">
          <Checkbox
            checked={row.getIsSelected()}
            onCheckedChange={(value) => row.toggleSelected(!!value)}
          />
        </div>
      ),
      enableSorting: false,
      enableHiding: false,
      size: 16,
    },
    {
      accessorKey: 'email',
      header: ({ column }) => {
        return (
          <Button
            variant="ghost"
            onClick={() => column.toggleSorting(column.getIsSorted() === 'asc')}
          >
            <HStack gap={2} p={0}>
              <span>User Info</span>
              {column.getIsSorted() === 'asc' ? (
                <HugeiconsIcon icon={ArrowUp01Icon} className="h-4 w-4" />
              ) : column.getIsSorted() === 'desc' ? (
                <HugeiconsIcon icon={ArrowDown01Icon} className="h-4 w-4" />
              ) : null}
            </HStack>
          </Button>
        )
      },
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
    },
    {
      accessorKey: 'is_verified',
      header: ({ column }) => (
        <Button
          variant="ghost"
          onClick={() => column.toggleSorting(column.getIsSorted() === 'asc')}
        >
          <HStack gap={2} p={0}>
            <span>
              {column.id === 'is_verified' ? 'Verification' : 'Lock Status'}
            </span>
            {column.getIsSorted() === 'asc' ? (
              <HugeiconsIcon icon={ArrowUp01Icon} className="h-4 w-4" />
            ) : column.getIsSorted() === 'desc' ? (
              <HugeiconsIcon icon={ArrowDown01Icon} className="h-4 w-4" />
            ) : null}
          </HStack>
        </Button>
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
      accessorKey: 'lockout_until',
      header: ({ column }) => (
        <Button
          variant="ghost"
          onClick={() => column.toggleSorting(column.getIsSorted() === 'asc')}
        >
          <HStack gap={2} p={0}>
            <span>Lock Status</span>
            {column.getIsSorted() === 'asc' ? (
              <HugeiconsIcon icon={ArrowUp01Icon} className="h-4 w-4" />
            ) : column.getIsSorted() === 'desc' ? (
              <HugeiconsIcon icon={ArrowDown01Icon} className="h-4 w-4" />
            ) : null}
          </HStack>
        </Button>
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
    },
    {
      accessorKey: 'created_at',
      header: ({ column }) => {
        return (
          <Button
            variant="ghost"
            onClick={() => column.toggleSorting(column.getIsSorted() === 'asc')}
          >
            <HStack gap={2} p={0}>
              <span>Joined date</span>
              {column.getIsSorted() === 'asc' ? (
                <HugeiconsIcon icon={ArrowUp01Icon} className="h-4 w-4" />
              ) : column.getIsSorted() === 'desc' ? (
                <HugeiconsIcon icon={ArrowDown01Icon} className="h-4 w-4" />
              ) : null}
            </HStack>
          </Button>
        )
      },
      cell: ({ row }) => (
        <span className="text-sm text-muted-foreground">
          {format(new Date(row.getValue('created_at')), 'd MMM yyyy, h:mm a')}
        </span>
      ),
    },
    {
      accessorKey: 'updated_at',
      header: ({ column }) => {
        return (
          <Button
            variant="ghost"
            onClick={() => column.toggleSorting(column.getIsSorted() === 'asc')}
          >
            <HStack gap={2} p={0}>
              <span>Last Updated</span>
              {column.getIsSorted() === 'asc' ? (
                <HugeiconsIcon icon={ArrowUp01Icon} className="h-4 w-4" />
              ) : column.getIsSorted() === 'desc' ? (
                <HugeiconsIcon icon={ArrowDown01Icon} className="h-4 w-4" />
              ) : null}
            </HStack>
          </Button>
        )
      },
      cell: ({ row }) => (
        <span className="text-sm text-muted-foreground">
          {format(new Date(row.getValue('updated_at')), 'd MMM yyyy, h:mm a')}
        </span>
      ),
    },
    {
      id: 'actions',
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
                  onToggleVerify(user)
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
    },
  ]
}
