import { format } from 'date-fns'
import { HugeiconsIcon } from '@hugeicons/react'
import {
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
import { Link } from '@tanstack/react-router'
import { useUsersStore } from '../store'
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
import { Card } from '@/components/ui/card'
import { Box, Grid, HStack, Stack, Text } from '@/components/primitives'
import {
  Empty,
  EmptyContent,
  EmptyDescription,
  EmptyHeader,
  EmptyMedia,
  EmptyTitle,
} from '@/components/ui/empty'

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
  const { setIsCreateUserOpen } = useUsersStore()

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

        <EmptyContent className="flex-row justify-center">
          <Link to="/sign-up">
            <Button variant="outline" size="sm">
              Sign Up A User
            </Button>
          </Link>
          <Button
            variant="default"
            size="sm"
            onClick={() => setIsCreateUserOpen(true)}
          >
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

                <Stack gap={1}>
                  <HStack gap={2} align="center">
                    <Text size="sm" className="truncate">
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
                      className={`text-[10px] px-1.5 py-0 ${isLocked
                          ? 'text-red-500'
                          : user.is_verified
                            ? 'text-green-500'
                            : 'text-amber-500'
                        }`}
                    >
                      {isLocked
                        ? 'Locked'
                        : user.is_verified
                          ? 'Active'
                          : 'Pending'}
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
                      <HStack gap={2} p={0}>
                        <HugeiconsIcon
                          icon={Copy01Icon}
                          className="size-4 opacity-70"
                        />
                        <span>Copy ID</span>
                      </HStack>
                    </DropdownMenuItem>
                    <DropdownMenuItem
                      onClick={() => {
                        navigator.clipboard.writeText(user.email)
                        toast.success('Email copied to clipboard')
                      }}
                    >
                      <HStack gap={2} p={0}>
                        <HugeiconsIcon
                          icon={Copy01Icon}
                          className="size-4 opacity-70"
                        />
                        <span>Copy email</span>
                      </HStack>
                    </DropdownMenuItem>

                    <DropdownMenuSeparator />

                    <DropdownMenuItem onClick={() => onEdit(user)}>
                      <HStack gap={2} p={0}>
                        <HugeiconsIcon
                          icon={PencilEdit01Icon}
                          className="size-4 opacity-70"
                        />
                        <span>Edit</span>
                      </HStack>
                    </DropdownMenuItem>

                    <DropdownMenuItem onClick={() => onManagePermissions(user)}>
                      <HStack gap={2} p={0}>
                        <HugeiconsIcon
                          icon={Shield02Icon}
                          className="size-4 opacity-70"
                        />
                        <span>Permissions</span>
                      </HStack>
                    </DropdownMenuItem>

                    <DropdownMenuSeparator />

                    <DropdownMenuItem
                      onClick={() => onToggleVerify(user)}
                      disabled={isBeingUpdated}
                    >
                      <HStack gap={2} p={0}>
                        {isBeingUpdated ? (
                          <Spinner className="size-4" />
                        ) : (
                          <HugeiconsIcon
                            icon={Tick01Icon}
                            className="size-4 opacity-70"
                          />
                        )}
                        <span>{user.is_verified ? 'Unverify' : 'Verify'}</span>
                      </HStack>
                    </DropdownMenuItem>

                    <DropdownMenuItem
                      onClick={() => onToggleLock(user)}
                      disabled={isBeingUpdated}
                    >
                      <HStack gap={2} p={0}>
                        {isBeingUpdated ? (
                          <Spinner className="size-4" />
                        ) : (
                          <HugeiconsIcon
                            icon={LockIcon}
                            className="size-4 opacity-70"
                          />
                        )}
                        <span>
                          {isLocked ? 'Unlock account' : 'Lock account'}
                        </span>
                      </HStack>
                    </DropdownMenuItem>

                    <DropdownMenuSeparator />

                    <DropdownMenuItem
                      onClick={() => onDelete(user.id)}
                      className="text-destructive focus:text-destructive"
                    >
                      <HStack gap={2} p={0}>
                        <HugeiconsIcon
                          icon={Delete02Icon}
                          className="size-4 opacity-70"
                        />
                        <span>Delete</span>
                      </HStack>
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
    <Card className="p-3">
      <HStack align="start" gap={3}>
        <Box className="h-8 w-8 rounded-full bg-muted animate-pulse" />
        <Stack gap={1} className="flex-1">
          <HStack gap={2} align="center">
            <Box className="h-4 w-24 rounded bg-muted animate-pulse" />
            <Box className="h-4 w-12 rounded bg-muted animate-pulse" />
          </HStack>
          <Box className="h-3 w-32 rounded bg-muted animate-pulse" />
          <HStack gap={2}>
            <Box className="h-3 w-20 rounded bg-muted animate-pulse" />
            <Box className="h-4 w-16 rounded bg-muted animate-pulse" />
          </HStack>
        </Stack>
      </HStack>
    </Card>
  )
}
