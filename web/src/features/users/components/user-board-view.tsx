import { format } from 'date-fns'
import { HugeiconsIcon } from '@hugeicons/react'
import {
  Calendar01Icon,
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

import { Box, Grid, HStack, Stack, Text } from '@/components/primitives'

interface UserBoardViewProps {
  users: Array<UserResponse> | undefined
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
      <Box
        p={6}
        rounded="xl"
        className="flex h-64 items-center justify-center border border-dashed"
      >
        <Text muted>No users found</Text>
      </Box>
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
        const updatedAt = new Date(user.updated_at)
        const lockUntil = user.lockout_until
          ? new Date(user.lockout_until)
          : null

        const isLocked = lockUntil ? lockUntil > new Date() : false
        const isBeingUpdated = isUpdating && updatingUserId === user.id

        return (
          <Card key={user.id}>
            <CardHeader className="flex flex-row items-start justify-between">
              <Stack gap={1} align="center">
                <Avatar className="h-10 w-10">
                  <AvatarImage
                    src={`https://api.dicebear.com/7.x/avataaars/svg?seed=${user.email}`}
                  />
                  <AvatarFallback className="text-xs font-bold">
                    {initials}
                  </AvatarFallback>
                </Avatar>
                <Badge variant="secondary" className="text-xs">
                  {user.role}
                </Badge>
              </Stack>
              <Stack p={0} gap={0}>
                <CardTitle>{name}</CardTitle>
                <Text size="xs" muted className="truncate">
                  {user.email}
                </Text>
              </Stack>

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
            </CardHeader>

            <CardContent>
              <Stack gap={1}>
                <HStack gap={2}>
                  <HugeiconsIcon
                    icon={Calendar01Icon}
                    className="size-3.5 text-muted-foreground"
                  />
                  <Text size="xs" muted>
                    Joined {format(createdAt, 'MMM d, yyyy')}
                  </Text>
                </HStack>

                <Text size="xs" muted>
                  Updated {format(updatedAt, 'MMM d, yyyy')}
                </Text>

                <Text size="xs" muted className="font-mono">
                  ID: {user.id.slice(0, 8)}...
                </Text>
              </Stack>
            </CardContent>

            <CardFooter className="flex flex-row flex-wrap items-center justify-between">
              {isLocked && lockUntil && (
                <Badge variant="destructive" className="text-xs">
                  Locked until {format(lockUntil, 'MMM d, yyyy')}
                </Badge>
              )}

              <Badge
                variant="outline"
                className={`text-xs ${
                  isLocked
                    ? 'text-red-500'
                    : user.is_verified
                      ? 'text-green-500'
                      : 'text-amber-500'
                }`}
              >
                {isLocked ? 'Locked' : user.is_verified ? 'Active' : 'Pending'}
              </Badge>
            </CardFooter>
          </Card>
        )
      })}
    </Grid>
  )
}

/* ---------------- Skeleton ---------------- */

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
