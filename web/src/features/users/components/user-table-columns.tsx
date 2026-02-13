import {
  ArrowDown01Icon,
  ArrowUp01Icon,
  Copy01Icon,
  Delete02Icon,
  LockIcon,
  Menu01Icon,
  PencilEdit01Icon,
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
import { cn } from '@/lib/utils'
import {
  DropdownMenu,
  DropdownMenuContent,
  DropdownMenuItem,
  DropdownMenuSeparator,
  DropdownMenuTrigger,
} from '@/components/ui/dropdown-menu'
import { Spinner } from '@/components/ui/spinner'

interface GetColumnsProps {
  users?: Array<User>
  onToggleVerify: (user: User) => void
  onToggleLock: (user: User) => void
  setUserToDelete: (id: string | null) => void
  setUserToEdit: (user: User | null) => void
  setUserToManagePermissions: (user: User | null) => void
  isUpdating?: boolean
  updatingUserId?: string | null
}

export function getUserColumns({
  onToggleVerify,
  setUserToDelete,
  setUserToEdit,
  setUserToManagePermissions,
  onToggleLock,
  isUpdating,
  updatingUserId,
}: GetColumnsProps): Array<ColumnDef<User>> {
  return [
    {
      id: 'select',
      header: ({ table }) => (
        <Checkbox
          checked={table.getIsAllPageRowsSelected()}
          indeterminate={
            !table.getIsAllPageRowsSelected() && table.getIsSomeRowsSelected()
          }
          onCheckedChange={(value) => {
            table.toggleAllPageRowsSelected(!!value)
          }}
        />
      ),
      cell: ({ row }) => (
        <Checkbox
          checked={row.getIsSelected()}
          onCheckedChange={(value) => row.toggleSelected(!!value)}
        />
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
            User Info
            {column.getIsSorted() === 'asc' ? (
              <HugeiconsIcon icon={ArrowUp01Icon} className="ml-2 h-4 w-4" />
            ) : column.getIsSorted() === 'desc' ? (
              <HugeiconsIcon icon={ArrowDown01Icon} className="ml-2 h-4 w-4" />
            ) : null}
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
          <div className="flex items-center gap-3">
            <Avatar className="h-9 w-9 border border-border/50">
              <AvatarImage
                src={`https://api.dicebear.com/7.x/avataaars/svg?seed=${user.email}`}
              />
              <AvatarFallback className="bg-primary/10 text-primary text-xs font-medium">
                {initials}
              </AvatarFallback>
            </Avatar>
            <div className="flex flex-col">
              <span className="text-sm font-medium text-foreground">
                {name}
              </span>
              <span className="text-xs text-muted-foreground">
                {user.email}
              </span>
            </div>
          </div>
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
          Verification
          {column.getIsSorted() === 'asc' ? (
            <HugeiconsIcon icon={ArrowUp01Icon} className="ml-2 h-4 w-4" />
          ) : column.getIsSorted() === 'desc' ? (
            <HugeiconsIcon icon={ArrowDown01Icon} className="ml-2 h-4 w-4" />
          ) : null}
        </Button>
      ),
      cell: ({ row }) => {
        const isVerified = row.original.is_verified

        return (
          <div className="flex items-center gap-2">
            <span
              className={cn('inline-flex h-2 w-2 rounded-full', {
                'bg-green-500': isVerified,
                'bg-red-500': !isVerified,
              })}
            />
            <span
              className={cn('text-xs font-medium', {
                'text-green-600': isVerified,
                'text-red-600': !isVerified,
              })}
            >
              {isVerified ? 'Verified' : 'Unverified'}
            </span>
          </div>
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
          Lock Status
          {column.getIsSorted() === 'asc' ? (
            <HugeiconsIcon icon={ArrowUp01Icon} className="ml-2 h-4 w-4" />
          ) : column.getIsSorted() === 'desc' ? (
            <HugeiconsIcon icon={ArrowDown01Icon} className="ml-2 h-4 w-4" />
          ) : null}
        </Button>
      ),
      cell: ({ row }) => {
        const lockoutUntil = row.original.lockout_until
        const isLocked = lockoutUntil && new Date(lockoutUntil) > new Date()

        return (
          <div className="flex flex-col">
            <div className="flex items-center gap-2">
              <span
                className={cn('inline-flex h-2 w-2 rounded-full', {
                  'bg-amber-500': isLocked,
                  'bg-muted-foreground': !isLocked,
                })}
              />
              <span
                className={cn('text-xs font-medium', {
                  'text-amber-600': isLocked,
                  'text-muted-foreground': !isLocked,
                })}
              >
                {isLocked ? 'Locked' : 'Unlocked'}
              </span>
            </div>

            {isLocked && lockoutUntil && (
              <span className="text-[10px] text-muted-foreground whitespace-nowrap">
                Until {format(new Date(lockoutUntil), 'd MMM yyyy')}
              </span>
            )}
          </div>
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
            Joined date
            {column.getIsSorted() === 'asc' ? (
              <HugeiconsIcon icon={ArrowUp01Icon} className="ml-2 h-4 w-4" />
            ) : column.getIsSorted() === 'desc' ? (
              <HugeiconsIcon icon={ArrowDown01Icon} className="ml-2 h-4 w-4" />
            ) : null}
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
            Last Updated
            {column.getIsSorted() === 'asc' ? (
              <HugeiconsIcon icon={ArrowUp01Icon} className="ml-2 h-4 w-4" />
            ) : column.getIsSorted() === 'desc' ? (
              <HugeiconsIcon icon={ArrowDown01Icon} className="ml-2 h-4 w-4" />
            ) : null}
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
                  <HugeiconsIcon icon={Menu01Icon} className="h-4 w-4" />
                  <span className="sr-only">Open menu</span>
                </Button>
              }
            />

            <DropdownMenuContent align="end" className="min-w-40">
              <DropdownMenuItem
                onClick={() => {
                  navigator.clipboard.writeText(user.id)
                  toast.success('User ID copied to clipboard')
                }}
              >
                <HugeiconsIcon icon={Copy01Icon} className="mr-2 h-4 w-4" />
                Copy ID
              </DropdownMenuItem>

              <DropdownMenuItem
                onClick={() => {
                  navigator.clipboard.writeText(user.email)
                  toast.success('User email copied to clipboard')
                }}
              >
                <HugeiconsIcon icon={Copy01Icon} className="mr-2 h-4 w-4" />
                Copy Email
              </DropdownMenuItem>

              <DropdownMenuSeparator />

              <DropdownMenuItem onClick={() => setUserToEdit(user)}>
                <HugeiconsIcon
                  icon={PencilEdit01Icon}
                  className="mr-2 h-4 w-4"
                />
                Edit
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
                {isBeingUpdated ? (
                  <Spinner className="mr-2 h-4 w-4" />
                ) : (
                  <HugeiconsIcon
                    icon={UserCheckIcon}
                    className="mr-2 h-4 w-4"
                  />
                )}
                {user.is_verified ? 'Unverify' : 'Verify'}
              </DropdownMenuItem>

              <DropdownMenuItem
                onClick={() => onToggleLock(user)}
                disabled={isBeingUpdated}
                closeOnClick={false}
              >
                {isBeingUpdated ? (
                  <Spinner className="mr-2 h-4 w-4" />
                ) : (
                  <HugeiconsIcon icon={LockIcon} className="mr-2 h-4 w-4" />
                )}
                {isLocked ? 'Unlock' : 'Lock'}
              </DropdownMenuItem>

              <DropdownMenuSeparator />

              <DropdownMenuItem
                onClick={() => setUserToManagePermissions(user)}
              >
                <HugeiconsIcon icon={UserCheckIcon} className="mr-2 h-4 w-4" />
                Permissions
              </DropdownMenuItem>

              <DropdownMenuSeparator />

              <DropdownMenuItem
                onClick={() => setUserToDelete(user.id)}
                className="text-destructive focus:text-destructive"
              >
                <HugeiconsIcon icon={Delete02Icon} className="mr-2 h-4 w-4" />
                Delete
              </DropdownMenuItem>
            </DropdownMenuContent>
          </DropdownMenu>
        )
      },
    },
  ]
}
