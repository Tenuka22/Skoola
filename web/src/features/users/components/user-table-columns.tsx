import {
    ArrowDown01Icon,
    ArrowUp01Icon,
    CheckmarkCircle01Icon,
    Delete02Icon,
    LockIcon,
    Menu01Icon,
    PencilEdit01Icon,
    UserCheckIcon
} from '@hugeicons/core-free-icons'
import { HugeiconsIcon } from '@hugeicons/react'
import { format } from 'date-fns'
import type { ColumnDef } from '@tanstack/react-table'
import type { User } from '../types'
import { Avatar, AvatarFallback } from '@/components/ui/avatar'
import { Badge } from '@/components/ui/badge'
import { Button } from '@/components/ui/button'
import { Checkbox } from '@/components/ui/checkbox'
import {
    DropdownMenu,
    DropdownMenuContent,
    DropdownMenuItem,
    DropdownMenuSeparator,
    DropdownMenuTrigger,
} from '@/components/ui/dropdown-menu'

interface GetColumnsProps {
  users?: Array<User>
  onToggleVerify: (user: User) => void
  selectedUsers: Set<string>
  setSelectedUsers: (
    users: Set<string> | ((prev: Set<string>) => Set<string>),
  ) => void
  onToggleLock: (user: User) => void
  setUserToDelete: (id: string | null) => void
  setUserToEdit: (user: User | null) => void
  setUserToManagePermissions: (user: User | null) => void
}

export function getUserColumns({
  users,
  onToggleVerify,
  selectedUsers,
  setSelectedUsers,
  setUserToDelete,
  setUserToEdit,
  setUserToManagePermissions,
  onToggleLock
}: GetColumnsProps): Array<ColumnDef<User>> {
  return [
    {
      id: 'select',
      header: ({ table }) => (
        <Checkbox
          checked={table.getIsAllPageRowsSelected()}
          onCheckedChange={(value) => {
            table.toggleAllPageRowsSelected(!!value)
            if (value) {
              const allIds = users?.map((u) => u.id) || []
              setSelectedUsers(new Set(allIds))
            } else {
              setSelectedUsers(new Set())
            }
          }}
          className="border-muted-foreground/30 data-[state=checked]:bg-primary data-[state=checked]:border-primary"
        />
      ),
      cell: ({ row }) => (
        <Checkbox
          checked={selectedUsers.has(row.original.id)}
          onCheckedChange={(value) => {
            row.toggleSelected(!!value)
            const newSelected = new Set(selectedUsers)
            if (value) newSelected.add(row.original.id)
            else newSelected.delete(row.original.id)
            setSelectedUsers(newSelected)
          }}
          className="border-muted-foreground/30 data-[state=checked]:bg-primary data-[state=checked]:border-primary"
        />
      ),
      enableSorting: false,
      enableHiding: false,
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
      header: ({ column }) => {
        return (
          <Button
            variant="ghost"
            onClick={() => column.toggleSorting(column.getIsSorted() === 'asc')}
          >
            Status
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
        const isVerified = user.is_verified
        const lockoutUntil = user.lockout_until
        const isLocked =
          lockoutUntil && new Date(lockoutUntil) > new Date()

        return (
          <div className="flex items-center gap-2">
            <span className={`relative flex h-2 w-2`}>
              <span
                className={`animate-ping absolute inline-flex h-full w-full rounded-full opacity-75 ${
                  isLocked
                    ? 'bg-amber-400'
                    : isVerified
                      ? 'bg-green-400'
                      : 'bg-red-400'
                }`}
              ></span>
              <span
                className={`relative inline-flex rounded-full h-2 w-2 ${
                  isLocked
                    ? 'bg-amber-500'
                    : isVerified
                      ? 'bg-green-500'
                      : 'bg-red-500'
                }`}
              ></span>
            </span>
            <div className="flex flex-col">
              <Badge
                variant="outline"
                className={`border-0 bg-transparent px-0 font-medium ${
                  isLocked
                    ? 'text-amber-500'
                    : isVerified
                      ? 'text-green-500'
                      : 'text-red-500'
                }`}
              >
                {isLocked ? 'Locked' : isVerified ? 'Active' : 'Inactive'}
              </Badge>
              {isLocked && lockoutUntil && (
                <span className="text-[10px] text-muted-foreground -mt-1 whitespace-nowrap">
                  Until {format(new Date(lockoutUntil), 'd MMM')}
                </span>
              )}
            </div>
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
      id: 'actions',
      header: 'Actions',
      cell: ({ row }) => {
        const user = row.original
        const isLocked =
          user.lockout_until && new Date(user.lockout_until) > new Date()
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
            <DropdownMenuContent align="end" className="w-[160px]">
              <DropdownMenuItem onClick={() => setUserToEdit(user)}>
                <HugeiconsIcon
                  icon={PencilEdit01Icon}
                  className="mr-2 h-4 w-4"
                />
                Edit
              </DropdownMenuItem>
              <DropdownMenuItem onClick={() => onToggleVerify(user)}>
                <HugeiconsIcon
                  icon={CheckmarkCircle01Icon}
                  className="mr-2 h-4 w-4"
                />
                {user.is_verified ? 'Unverify' : 'Verify'}
              </DropdownMenuItem>
              <DropdownMenuItem
                onClick={() => setUserToManagePermissions(user)}
              >
                <HugeiconsIcon icon={UserCheckIcon} className="mr-2 h-4 w-4" />
                Permissions
              </DropdownMenuItem>
              <DropdownMenuItem onClick={() => onToggleLock(user)}>
                <HugeiconsIcon icon={LockIcon} className="mr-2 h-4 w-4" />
                {isLocked ? 'Unlock' : 'Lock'}
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
