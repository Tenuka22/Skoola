import { format } from 'date-fns'
import { HugeiconsIcon } from '@hugeicons/react'
import {
  Calendar03Icon,
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
import {
  ContextMenu,
  ContextMenuContent,
  ContextMenuItem,
  ContextMenuSeparator,
  ContextMenuTrigger,
} from '@/components/ui/context-menu'
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

interface UserGridViewProps {
  users: Array<UserResponse>
  limit: number
  isLoading?: boolean
  onEdit: (user: UserResponse) => void
  onDelete: (id: string) => void
  onToggleVerify: (user: UserResponse) => void
  onToggleLock: (user: UserResponse) => void
  onManagePermissions: (user: UserResponse) => void
  isUpdating?: boolean
  updatingUserId?: string | null
}

export function UserGridView({
  users,
  limit,
  isLoading,
  onEdit,
  onDelete,
  onToggleVerify,
  onToggleLock,
  onManagePermissions,
  isUpdating,
  updatingUserId,
}: UserGridViewProps) {
  const { setIsCreateUserOpen } = useUsersStore()

  if (isLoading) {
    return (
      <Grid cols={4} gap={4}>
        {Array.from({ length: limit || 12 }).map((_, i) => (
          <CardSkeleton key={i} index={i} />
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

        const statusColor = isLocked
          ? 'text-red-400 bg-red-500/10 border-red-500/20'
          : user.is_verified
            ? 'text-emerald-400 bg-emerald-500/10 border-emerald-500/20'
            : 'text-amber-400 bg-amber-500/10 border-amber-500/20'

        const statusDot = isLocked
          ? 'bg-red-500'
          : user.is_verified
            ? 'bg-emerald-500'
            : 'bg-amber-500'

        const statusLabel = isLocked
          ? 'Locked'
          : user.is_verified
            ? 'Active'
            : 'Pending'

        const contextMenuItems = (
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
                <HugeiconsIcon icon={Copy01Icon} className="size-4" />
                <span>Copy ID</span>
              </HStack>
            </ContextMenuItem>

            <ContextMenuItem
              onClick={() => {
                navigator.clipboard.writeText(user.email)
                toast.success('Email copied to clipboard')
              }}
            >
              <HStack gap={2} p={0}>
                <HugeiconsIcon icon={Copy01Icon} className="size-4" />
                <span>Copy Email</span>
              </HStack>
            </ContextMenuItem>

            <ContextMenuSeparator />

            <ContextMenuItem onClick={() => onEdit(user)}>
              <HStack gap={2} p={0}>
                <HugeiconsIcon icon={PencilEdit01Icon} className="size-4" />
                <span>Edit</span>
              </HStack>
            </ContextMenuItem>

            <ContextMenuItem onClick={() => onManagePermissions(user)}>
              <HStack gap={2} p={0}>
                <HugeiconsIcon icon={Shield02Icon} className="size-4" />
                <span>Manage Permissions</span>
              </HStack>
            </ContextMenuItem>

            <ContextMenuSeparator />

            <ContextMenuItem
              onClick={() => onToggleVerify(user)}
              disabled={isBeingUpdated}
            >
              <HStack gap={2} p={0}>
                {isBeingUpdated ? (
                  <Spinner className="size-4" />
                ) : (
                  <HugeiconsIcon icon={Tick01Icon} className="size-4" />
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
                  <Spinner className="size-4" />
                ) : (
                  <HugeiconsIcon icon={LockIcon} className="size-4" />
                )}
                <span>{isLocked ? 'Unlock' : 'Lock'}</span>
              </HStack>
            </ContextMenuItem>

            <ContextMenuSeparator />

            <ContextMenuItem
              onClick={() => onDelete(user.id)}
              variant="destructive"
            >
              <HStack gap={2} p={0}>
                <HugeiconsIcon icon={Delete02Icon} className="size-4" />
                <span>Delete</span>
              </HStack>
            </ContextMenuItem>
          </>
        )

        return (
          <ContextMenu key={user.id}>
            <ContextMenuTrigger
              render={
                <Card className="group relative overflow-hidden border-border/50 hover:border-border hover:shadow-md transition-all duration-200 cursor-default">
                  {/* Top section: Avatar + Info */}
                  <div className="p-4 pb-3">
                    <HStack align="start" justify="between" gap={3}>
                      <HStack align="center" gap={3} className="min-w-0 flex-1">
                        <Avatar className="h-10 w-10 ring-2 ring-border/30 shrink-0">
                          <AvatarImage
                            src={`https://api.dicebear.com/7.x/avataaars/svg?seed=${user.email}`}
                          />
                          <AvatarFallback className="text-xs font-semibold bg-muted">
                            {initials}
                          </AvatarFallback>
                        </Avatar>

                        <Stack gap={1} className="min-w-0 flex-1">
                          <HStack gap={2} align="center">
                            <Text
                              size="sm"
                              className="font-semibold truncate capitalize"
                            >
                              {name}
                            </Text>
                          </HStack>
                          <Text size="xs" muted className="truncate">
                            {user.email}
                          </Text>
                        </Stack>
                      </HStack>

                      <DropdownMenu>
                        <DropdownMenuTrigger
                          render={
                            <Button
                              variant="ghost"
                              size="icon-sm"
                              className="opacity-0 group-hover:opacity-100 transition-opacity -mr-1 -mt-1 shrink-0"
                            >
                              <HugeiconsIcon
                                icon={MoreVerticalIcon}
                                size={16}
                              />
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
                              <span>Copy Email</span>
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
                          <DropdownMenuItem
                            onClick={() => onManagePermissions(user)}
                          >
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
                              <span>
                                {user.is_verified ? 'Unverify' : 'Verify'}
                              </span>
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
                              <span>{isLocked ? 'Unlock' : 'Lock'}</span>
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
                  </div>

                  {/* Bottom section: Meta info */}
                  <div className="border-t border-border/40 bg-muted/20 px-4 py-2.5">
                    <HStack justify="between" align="center">
                      <HStack gap={2} align="center">
                        <Badge
                          variant="secondary"
                          className="text-[10px] px-1.5 py-0 h-5 font-medium"
                        >
                          {user.role}
                        </Badge>
                        <Badge
                          variant="outline"
                          className={`text-[10px] px-1.5 py-0 h-5 font-medium border ${statusColor}`}
                        >
                          <span
                            className={`inline-block h-1.5 w-1.5 rounded-full mr-1 ${statusDot}`}
                          />
                          {statusLabel}
                        </Badge>
                      </HStack>

                      <HStack
                        gap={1}
                        align="center"
                        className="text-muted-foreground"
                      >
                        <HugeiconsIcon icon={Calendar03Icon} size={12} />
                        <Text size="xs" muted className="text-[11px]">
                          {format(createdAt, 'MMM d, yyyy')}
                        </Text>
                      </HStack>
                    </HStack>
                  </div>
                </Card>
              }
            />
            <ContextMenuContent className="min-w-40" alignOffset={-5}>
              {contextMenuItems}
            </ContextMenuContent>
          </ContextMenu>
        )
      })}
    </Grid>
  )
}

function CardSkeleton({ index = 0 }: { index?: number }) {
  const widthClasses = [
    'w-[110px]',
    'w-[90px]',
    'w-[140px]',
    'w-[120px]',
    'w-[100px]',
    'w-[130px]',
  ]
  const emailWidths = [
    'w-[160px]',
    'w-[140px]',
    'w-[180px]',
    'w-[150px]',
    'w-[170px]',
    'w-[130px]',
  ]
  const w1 = widthClasses[index % widthClasses.length]
  const w2 = emailWidths[(index + 3) % emailWidths.length]

  return (
    <Card className="overflow-hidden border-border/50">
      <div className="p-4 pb-3">
        <HStack align="center" gap={3}>
          <Box className="h-10 w-10 rounded-full bg-muted animate-pulse shrink-0" />
          <Stack gap={1} className="flex-1">
            <Box className={`h-4 ${w1} rounded bg-muted animate-pulse`} />
            <Box className={`h-3 ${w2} rounded bg-muted animate-pulse`} />
          </Stack>
        </HStack>
      </div>
      <div className="border-t border-border/40 bg-muted/20 px-4 py-2.5">
        <HStack justify="between" align="center">
          <HStack gap={2}>
            <Box className="h-5 w-14 rounded bg-muted animate-pulse" />
            <Box className="h-5 w-14 rounded bg-muted animate-pulse" />
          </HStack>
          <Box className="h-3 w-20 rounded bg-muted animate-pulse" />
        </HStack>
      </div>
    </Card>
  )
}
