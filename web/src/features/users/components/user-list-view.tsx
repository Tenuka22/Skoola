import { format } from 'date-fns'
import { HugeiconsIcon } from '@hugeicons/react'
import {
  Delete02Icon,
  MoreVerticalIcon,
  PencilEdit01Icon,
  Shield01Icon,
  Tick01Icon,
} from '@hugeicons/core-free-icons'
import { Avatar, AvatarFallback, AvatarImage } from '@/components/ui/avatar'
import { Badge } from '@/components/ui/badge'
import { Button } from '@/components/ui/button'
import {
  DropdownMenu,
  DropdownMenuContent,
  DropdownMenuItem,
  DropdownMenuSeparator,
  DropdownMenuTrigger,
} from '@/components/ui/dropdown-menu'
import { Skeleton } from '@/components/ui/skeleton'
import type { UserResponse } from '@/lib/api/types.gen'

interface UserListViewProps {
  users: UserResponse[] | undefined
  isLoading?: boolean
  onEdit: (user: UserResponse) => void
  onDelete: (id: string) => void
  onToggleVerify: (user: UserResponse) => void
  onManagePermissions: (user: UserResponse) => void
}

export function UserListView({
  users,
  isLoading,
  onEdit,
  onDelete,
  onToggleVerify,
  onManagePermissions,
}: UserListViewProps) {
  if (isLoading) {
    return (
      <div className="space-y-2">
        {Array.from({ length: 5 }).map((_, i) => (
          <div key={i} className="flex items-center gap-4 rounded-xl border border-border/60 p-3">
            <Skeleton className="h-10 w-10 rounded-full" />
            <div className="space-y-2 flex-1">
              <Skeleton className="h-4 w-1/4" />
              <Skeleton className="h-3 w-1/3" />
            </div>
          </div>
        ))}
      </div>
    )
  }

  if (!users?.length) {
    return (
      <div className="flex h-64 flex-col items-center justify-center rounded-xl border border-dashed bg-muted/10">
        <p className="text-muted-foreground">No users found</p>
      </div>
    )
  }

  return (
    <div className="space-y-2">
      {users.map((user) => {
        const name = user.email.split('@')[0].replace(/[._]/g, ' ').replace(/\b\w/g, l => l.toUpperCase())
        const initials = name.substring(0, 2).toUpperCase()
        const role = (user as any).role || 'Member'

        return (
          <div 
            key={user.id} 
            className="flex items-center justify-between rounded-xl border border-border/60 bg-background p-3 shadow-none"
          >
            <div className="flex items-center gap-4">
              <Avatar className="h-10 w-10 border border-border/50">
                <AvatarImage src={`https://api.dicebear.com/7.x/avataaars/svg?seed=${user.email}`} />
                <AvatarFallback className="bg-primary/10 text-primary text-xs font-bold">{initials}</AvatarFallback>
              </Avatar>
              
              <div className="flex flex-col sm:flex-row sm:items-center sm:gap-6">
                <div>
                  <h3 className="text-sm font-semibold leading-none">{name}</h3>
                  <p className="text-xs text-muted-foreground mt-1 sm:hidden">{user.email}</p>
                </div>
                
                <div className="hidden sm:block text-xs text-muted-foreground w-48 truncate">
                  {user.email}
                </div>

                <Badge variant="secondary" className="hidden sm:inline-flex rounded-md px-2 py-0 text-[10px] font-medium opacity-80">
                  {role}
                </Badge>
              </div>
            </div>

            <div className="flex items-center gap-4">
              <div className="hidden sm:flex flex-col items-end gap-1 text-xs text-muted-foreground">
                <span>{format(new Date(user.created_at), 'MMM d, yyyy')}</span>
                <Badge 
                  variant="outline" 
                  className={`border-0 bg-transparent px-0 font-medium ${user.is_verified ? 'text-green-500' : 'text-amber-500'}`}
                >
                  {user.is_verified ? 'Active' : 'Pending'}
                </Badge>
              </div>

              <div className="flex items-center gap-1">
                <DropdownMenu>
                  <DropdownMenuTrigger
                    render={
                      <Button variant="ghost" size="icon" className="h-8 w-8 text-muted-foreground">
                        <HugeiconsIcon icon={MoreVerticalIcon} className="size-4" />
                      </Button>
                    }
                  />
                  <DropdownMenuContent align="end">
                    <DropdownMenuItem onClick={() => onEdit(user)}>
                      <HugeiconsIcon icon={PencilEdit01Icon} className="mr-2 size-4 opacity-70" />
                      Edit Details
                    </DropdownMenuItem>
                    <DropdownMenuItem onClick={() => onToggleVerify(user)}>
                      <HugeiconsIcon icon={Tick01Icon} className="mr-2 size-4 opacity-70" />
                      {user.is_verified ? 'Mark Pending' : 'Mark Verified'}
                    </DropdownMenuItem>
                    <DropdownMenuItem onClick={() => onManagePermissions(user)}>
                      <HugeiconsIcon icon={Shield01Icon} className="mr-2 size-4 opacity-70" />
                      Permissions
                    </DropdownMenuItem>
                    <DropdownMenuSeparator />
                    <DropdownMenuItem onClick={() => onDelete(user.id)} className="text-destructive focus:text-destructive">
                      <HugeiconsIcon icon={Delete02Icon} className="mr-2 size-4 opacity-70" />
                      Delete User
                    </DropdownMenuItem>
                  </DropdownMenuContent>
                </DropdownMenu>
              </div>
            </div>
          </div>
        )
      })}
    </div>
  )
}
