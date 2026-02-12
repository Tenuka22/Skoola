import { HugeiconsIcon } from '@hugeicons/react'
import { Delete02Icon, PencilEdit01Icon } from '@hugeicons/core-free-icons'
import { format } from 'date-fns'
import type { ColumnDef } from '@tanstack/react-table'
import type { User } from '../types'
import { Checkbox } from '@/components/ui/checkbox'
import { Button } from '@/components/ui/button'
import { Avatar, AvatarFallback } from '@/components/ui/avatar'
import { Badge } from '@/components/ui/badge'

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
  onEditUser,
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
      accessorKey: 'full_name',
      header: 'Full name',
      cell: ({ row }) => {
        const user = row.original
        // Mock name from email since API doesn't return it yet
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
      accessorKey: 'role',
      header: 'Role',
      cell: ({ row }) => {
        // Mock role since API doesn't return it yet
        const role = (row.original as any).role || 'Member'
        return (
          <span className="text-sm text-foreground/80 font-medium">{role}</span>
        )
      },
    },
    {
      accessorKey: 'is_verified',
      header: 'Status',
      cell: ({ row }) => {
        const isActive = row.getValue('is_verified')
        return (
          <div className="flex items-center gap-2">
            <span className={`relative flex h-2 w-2`}>
              <span
                className={`animate-ping absolute inline-flex h-full w-full rounded-full opacity-75 ${isActive ? 'bg-green-400' : 'bg-red-400'}`}
              ></span>
              <span
                className={`relative inline-flex rounded-full h-2 w-2 ${isActive ? 'bg-green-500' : 'bg-red-500'}`}
              ></span>
            </span>
            <Badge
              variant="outline"
              className={`border-0 bg-transparent px-0 font-medium ${isActive ? 'text-green-500' : 'text-red-500'}`}
            >
              {isActive ? 'Active' : 'Inactive'}
            </Badge>
          </div>
        )
      },
    },
    {
      accessorKey: 'created_at',
      header: 'Joined date',
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
        return (
          <div className="flex items-center gap-2">
            <Button
              variant="ghost"
              size="sm"
              className="h-8 px-2 text-muted-foreground hover:text-foreground"
              onClick={() => onEditUser(user)}
            >
              <HugeiconsIcon
                icon={PencilEdit01Icon}
                className="mr-1.5 size-3.5"
              />
              Edit
            </Button>
            <Button
              variant="ghost"
              size="icon"
              className="h-8 w-8 text-muted-foreground hover:text-destructive hover:bg-destructive/10"
              onClick={() => setUserToDelete(user.id)}
            >
              <HugeiconsIcon icon={Delete02Icon} className="size-4" />
            </Button>
          </div>
        )
      },
    },
  ]
}
