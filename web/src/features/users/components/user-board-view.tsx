import { format } from 'date-fns'
import { HugeiconsIcon } from '@hugeicons/react'
import {
  Calendar01Icon,
  CloudCog,
  Copy01Icon,
  Delete02Icon,
  LockIcon,
  MoreVerticalIcon,
  PencilEdit01Icon,
  Shield02Icon,
  Tick01Icon,
} from '@hugeicons/core-free-icons'
import { toast } from 'sonner'
import type { UserResponse } from '@/lib/api/types.gen'

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
import { Spinner } from '@/components/ui/spinner'
import {
  Card,
  CardContent,
  CardFooter,
  CardHeader,
  CardTitle,
} from '@/components/ui/card'
import {
  Empty,
  EmptyContent,
  EmptyDescription,
  EmptyHeader,
  EmptyMedia,
  EmptyTitle,
} from "@/components/ui/empty"


import {  Grid, HStack, Stack, Text } from '@/components/primitives'
import { Link } from '@tanstack/react-router'
import { useUsersStore } from '../store'

interface UserBoardViewProps {
  users: Array<UserResponse> 
  isLoading?: boolean
  onEdit: (user: UserResponse) => void
  onDelete: (id: string) => void
  onToggleVerify: (user: UserResponse) => void
  onToggleLock: (user: UserResponse) => void
  onManagePermissions: (user: UserResponse) => void
  isUpdating?: boolean
  updatingUserId?: string | null
}

export function UserBoardView({
  users,
  isLoading,
  onEdit,
  onDelete,
  onToggleVerify,
  onToggleLock,
  onManagePermissions,
  isUpdating,
  updatingUserId,
}: UserBoardViewProps) {
    const {  setIsCreateUserOpen } =
      useUsersStore()

  if (isLoading) {
    return (
      <Grid cols={4} gap={4}>
        {Array.from({ length: 8 }).map((_, i) => (
          <CardSkeleton key={i} />
        ))}
      </Grid>
    )
  }

if (!users?.length) {
  return (
    <Empty className="border border-dashed w-auto">
      <EmptyHeader>
        <EmptyMedia variant="icon">
          <HugeiconsIcon icon={CloudCog} />
        </EmptyMedia>
        <EmptyTitle>No User Found</EmptyTitle>
        <EmptyDescription>
          Add users or share your app to get started.
        </EmptyDescription>
      </EmptyHeader>

      <EmptyContent className='flex-row justify-center'>
        <Link to="/sign-up">
          <Button variant="outline" size="sm">
            Sign Up A User
          </Button>
        </Link>
        <Button variant="default" size="sm" onClick={() => setIsCreateUserOpen(true)}>
          Create a user
          </Button>
      </EmptyContent>
    </Empty>
  )
}

  return (
    <Grid cols={4} gap={4}>
      {users.map((user) => {
        const name = user.email
          .split('@')[0]
          .replace(/[._]/g, ' ')
          .replace(/\b\w/g, (l: string) => l.toUpperCase())

        const initials = name.substring(0, 2).toUpperCase()

        const createdAt = new Date(user.created_at)
        const lockUntil = user.lockout_until
          ? new Date(user.lockout_until)
          : null


        const isBeingUpdated = isUpdating && updatingUserId === user.id

        const isLocked = lockUntil ? lockUntil > new Date() : false

        return (
       <Card key={user.id} className="p-3">
  <HStack align="start" justify="between" gap={3}>
    <HStack align="start" gap={3} className="">
      <Avatar className="h-8 w-8">
        <AvatarImage
          src={`https://api.dicebear.com/7.x/avataaars/svg?seed=${user.email}`}
        />
        <AvatarFallback className="text-[10px] font-semibold">
          {initials}
        </AvatarFallback>
      </Avatar>

      <Stack gap={1} >
        <HStack gap={2} align="center">
          <Text size="sm" className='truncate'>
            {name}
          </Text>
          <Badge
            variant="secondary"
            className="text-[10px] px-1.5 py-0"
          >
            {user.role}
          </Badge>
        </HStack>

        <Text size="xs" muted className="truncate">
          {user.email}
        </Text>
<HStack>

          <Text size="xs" muted>
            {format(createdAt, 'MMM d, yyyy')}
          </Text>
       <Badge
        variant="outline"
        className={`text-[10px] px-1.5 py-0 ${
          isLocked
            ? 'text-red-500'
            : user.is_verified
              ? 'text-green-500'
              : 'text-amber-500'
        }`}
      >
        {isLocked ? 'Locked' : user.is_verified ? 'Active' : 'Pending'}
      </Badge>
</HStack>
      </Stack>
    </HStack>

    <HStack align="center" gap={2}>
     

   
              <DropdownMenu>
                <DropdownMenuTrigger
                  render={
                    <Button variant="ghost" size="icon" className="h-fit">
                      <HugeiconsIcon icon={MoreVerticalIcon} />
                    </Button>
                  }
                />
                <DropdownMenuContent align="end">
                  <DropdownMenuItem
                    onClick={() => {
                      navigator.clipboard.writeText(user.id)
                      toast.success('User ID copied to clipboard')
                    }}
                  >
                    <HugeiconsIcon
                      icon={Copy01Icon}
                      className="mr-2 size-4 opacity-70"
                    />
                    Copy ID
                  </DropdownMenuItem>

                  <DropdownMenuItem
                    onClick={() => {
                      navigator.clipboard.writeText(user.email)
                      toast.success('User email copied to clipboard')
                    }}
                  >
                    <HugeiconsIcon
                      icon={Copy01Icon}
                      className="mr-2 size-4 opacity-70"
                    />
                    Copy Email
                  </DropdownMenuItem>

                  <DropdownMenuSeparator />

                  <DropdownMenuItem onClick={() => onEdit(user)}>
                    <HugeiconsIcon
                      icon={PencilEdit01Icon}
                      className="mr-2 size-4 opacity-70"
                    />
                    Edit
                  </DropdownMenuItem>

                  <DropdownMenuItem onClick={() => onManagePermissions(user)}>
                    <HugeiconsIcon
                      icon={Shield02Icon}
                      className="mr-2 size-4 opacity-70"
                    />
                    Manage Permissions
                  </DropdownMenuItem>

                  <DropdownMenuSeparator />

                  <DropdownMenuItem
                    onClick={() => onToggleVerify(user)}
                    disabled={isBeingUpdated}
                  >
                    {isBeingUpdated ? (
                      <Spinner className="mr-2 size-4" />
                    ) : (
                      <HugeiconsIcon
                        icon={Tick01Icon}
                        className="mr-2 size-4 opacity-70"
                      />
                    )}
                    {user.is_verified ? 'Unverify' : 'Verify'}
                  </DropdownMenuItem>

                  <DropdownMenuItem
                    onClick={() => onToggleLock(user)}
                    disabled={isBeingUpdated}
                  >
                    {isBeingUpdated ? (
                      <Spinner className="mr-2 size-4" />
                    ) : (
                      <HugeiconsIcon
                        icon={LockIcon}
                        className="mr-2 size-4 opacity-70"
                      />
                    )}
                    {isLocked ? 'Unlock' : 'Lock'}
                  </DropdownMenuItem>

                  <DropdownMenuSeparator />

                  <DropdownMenuItem
                    onClick={() => onDelete(user.id)}
                    className="text-destructive focus:text-destructive"
                  >
                    <HugeiconsIcon
                      icon={Delete02Icon}
                      className="mr-2 size-4 opacity-70"
                    />
                    Delete
                  </DropdownMenuItem>
                </DropdownMenuContent>
              </DropdownMenu>
    </HStack>
  </HStack>
</Card>
        )
      })}
    </Grid>
  )
}


function CardSkeleton() {
  return (
    <Card className="rounded-xl border bg-card">
      <CardHeader className="flex flex-row items-start space-x-3 p-4">
        <div className="h-10 w-10 animate-pulse rounded-full bg-muted" />
        <Stack gap={2}>
          <div className="h-4 w-24 animate-pulse rounded bg-muted" />
          <div className="h-3 w-16 animate-pulse rounded bg-muted" />
        </Stack>
      </CardHeader>
      <CardContent className="space-y-2 px-4 pb-4">
        <div className="h-3 w-full animate-pulse rounded bg-muted" />
        <div className="h-3 w-2/3 animate-pulse rounded bg-muted" />
      </CardContent>
    </Card>
  )
}
