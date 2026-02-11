import { HugeiconsIcon } from '@hugeicons/react'
import {
  Cancel01Icon,
  Delete02Icon,
  Menu01Icon,
  PencilEdit01Icon,
  Shield01Icon,
  Sorting05Icon,
  SquareLock02Icon,
  Tick01Icon,
} from '@hugeicons/core-free-icons'
import { format } from 'date-fns'
import type { ColumnDef } from '@tanstack/react-table'
import type { User } from '../types'
import { Checkbox } from '@/components/ui/checkbox'
import { Button } from '@/components/ui/button'
import {
  DropdownMenu,
  DropdownMenuContent,
  DropdownMenuItem,
  DropdownMenuSeparator,
  DropdownMenuTrigger,
} from '@/components/ui/dropdown-menu'

interface GetColumnsProps {
  selectedUsers: Set<string>
  setSelectedUsers: (ids: Set<string>) => void
  setUserToDelete: (id: string) => void
  onToggleVerify: (user: User) => void
  onToggleLock: (user: User) => void
  onEditUser: (user: User) => void
  onManagePermissions: (user: User) => void
  users?: Array<User>
}

export function getUserColumns({
  selectedUsers,
  setSelectedUsers,
  setUserToDelete,
  onToggleVerify,
  onToggleLock,
  onEditUser,
  onManagePermissions,
  users,
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
        />
      ),
    },
    {
      accessorKey: 'email',
      header: ({ column }) => (
        <Button
          variant="ghost"
          className="-ml-4 h-8 data-[state=open]:bg-accent"
          onClick={() => column.toggleSorting(column.getIsSorted() === 'asc')}
        >
          <span>Email</span>
          <HugeiconsIcon icon={Sorting05Icon} className="ml-2 size-4" />
        </Button>
      ),
    },
    {
      accessorKey: 'is_verified',
      header: ({ column }) => (
        <Button
          variant="ghost"
          className="-ml-4 h-8 data-[state=open]:bg-accent"
          onClick={() => column.toggleSorting(column.getIsSorted() === 'asc')}
        >
          <span>Status</span>
          <HugeiconsIcon icon={Sorting05Icon} className="ml-2 size-4" />
        </Button>
      ),
      cell: ({ row }) =>
        row.getValue('is_verified') ? (
          <div className="flex items-center gap-2 text-green-600">
            <HugeiconsIcon icon={Tick01Icon} className="size-3" />
            <span className="text-[11px] font-bold uppercase tracking-wider">
              Verified
            </span>
          </div>
        ) : (
          <div className="flex items-center gap-2 text-amber-600">
            <HugeiconsIcon icon={Cancel01Icon} className="size-3" />
            <span className="text-[11px] font-bold uppercase tracking-wider">
              Pending
            </span>
          </div>
        ),
    },
    {
      accessorKey: 'created_at',
      header: ({ column }) => (
        <Button
          variant="ghost"
          className="-ml-4 h-8 data-[state=open]:bg-accent"
          onClick={() => column.toggleSorting(column.getIsSorted() === 'asc')}
        >
          <span>Joined Date</span>
          <HugeiconsIcon icon={Sorting05Icon} className="ml-2 size-4" />
        </Button>
      ),
      cell: ({ row }) =>
        format(new Date(row.getValue('created_at')), 'MMM d, yyyy HH:mm'),
    },
    {
      id: 'actions',
      cell: ({ row }) => {
        const user = row.original
        return (
          <div className="text-right">
            <DropdownMenu>
              <DropdownMenuTrigger
                render={
                  <Button variant="ghost" size="icon" className="h-8 w-8">
                    <HugeiconsIcon icon={Menu01Icon} className="size-4" />
                  </Button>
                }
              />
              <DropdownMenuContent align="end" className="w-48 rounded-xl p-2">
                <DropdownMenuItem
                  className="gap-2 rounded-lg py-2"
                  onClick={() => onEditUser(user)}
                >
                  <HugeiconsIcon
                    icon={PencilEdit01Icon}
                    className="size-4 text-muted-foreground"
                  />
                  <span>Edit User</span>
                </DropdownMenuItem>
                <DropdownMenuItem
                  className="gap-2 rounded-lg py-2"
                  onClick={() => onToggleVerify(user)}
                >
                  <HugeiconsIcon
                    icon={Tick01Icon}
                    className="size-4 text-primary"
                  />
                  <span>
                    {user.is_verified ? 'Mark Unverified' : 'Mark Verified'}
                  </span>
                </DropdownMenuItem>

                <DropdownMenuItem
                  className="gap-2 rounded-lg py-2"
                  onClick={() => onToggleLock(user)}
                >
                  <HugeiconsIcon
                    icon={SquareLock02Icon}
                    className="size-4 text-amber-500"
                  />
                  <span>Toggle Lock Status</span>
                </DropdownMenuItem>

                <DropdownMenuItem
                  className="gap-2 rounded-lg py-2"
                  onClick={() => onManagePermissions(user)}
                >
                  <HugeiconsIcon
                    icon={Shield01Icon}
                    className="size-4 text-primary"
                  />
                  <span>Manage Permissions</span>
                </DropdownMenuItem>

                <DropdownMenuSeparator />

                <DropdownMenuItem
                  className="gap-2 rounded-lg py-2 text-destructive focus:bg-destructive/10 focus:text-destructive"
                  onClick={() => setUserToDelete(user.id)}
                >
                  <HugeiconsIcon icon={Delete02Icon} className="size-4" />
                  <span>Purge Account</span>
                </DropdownMenuItem>
              </DropdownMenuContent>
            </DropdownMenu>
          </div>
        )
      },
    },
  ]
}
