import { format } from 'date-fns'
import { HugeiconsIcon } from '@hugeicons/react'
import {
  CloudCog,
  Delete02Icon,
  MoreVerticalIcon,
  PencilEdit01Icon,
} from '@hugeicons/core-free-icons'
import type { StaffResponse } from '@/lib/api/types.gen'
import { Avatar, AvatarFallback, AvatarImage } from '@/components/ui/avatar'
import { Badge } from '@/components/ui/badge'
import { Button } from '@/components/ui/button'
import {
  CardContent,
  CardHeader,
  Card as CardPrimitive,
} from '@/components/ui/card'
import {
  DropdownMenu,
  DropdownMenuContent,
  DropdownMenuItem,
  DropdownMenuSeparator,
  DropdownMenuTrigger,
} from '@/components/ui/dropdown-menu'
import { Skeleton } from '@/components/ui/skeleton'
import { Grid, HStack, Stack, Text } from '@/components/primitives'
import {
  Empty,
  EmptyContent,
  EmptyDescription,
  EmptyHeader,
  EmptyMedia,
  EmptyTitle,
} from '@/components/ui/empty'
import { useStaffStore } from '../store'

interface StaffBoardViewProps {
  staff: Array<StaffResponse> | undefined
  isLoading?: boolean
  onEdit: (staff: StaffResponse) => void
  onDelete: (id: string) => void
}

export function StaffBoardView({
  staff,
  isLoading,
  onEdit,
  onDelete,
}: StaffBoardViewProps) {
  const { setIsCreateStaffOpen } = useStaffStore()


  if (isLoading) {
    return (
      <Grid
        gap={4}
        className="grid-cols-1 sm:grid-cols-2 lg:grid-cols-3 xl:grid-cols-4"
      >
        {Array.from({ length: 8 }).map((_, i) => (
          <CardPrimitive
            key={i}
            className="p-0 overflow-hidden border-border/60 shadow-sm bg-card"
          >
            <CardHeader>
              <HStack gap={4} className="p-4">
                <Skeleton className="h-10 w-10 rounded-full" />
                <Stack gap={2}>
                  <Skeleton className="h-4 w-32" />
                  <Skeleton className="h-3 w-24" />
                </Stack>
              </HStack>
            </CardHeader>
            <CardContent className="p-4 pt-0">
              <Stack gap={2}>
                <Skeleton className="h-3 w-full" />
                <Skeleton className="h-3 w-2/3" />
              </Stack>
            </CardContent>
          </CardPrimitive>
        ))}
      </Grid>
    )
  }

  if (!staff?.length) {
    return (
      <Empty className="border border-dashed w-auto">
        <EmptyHeader>
          <EmptyMedia variant="icon">
            <HugeiconsIcon icon={CloudCog} />
          </EmptyMedia>
          <EmptyTitle>No Staff Found</EmptyTitle>
          <EmptyDescription>
            Add staff members to get started.
          </EmptyDescription>
        </EmptyHeader>

        <EmptyContent className="flex-row justify-center">

          <Button
            variant="default"
            size="sm"
            onClick={() => setIsCreateStaffOpen(true)}
          >
            Create a staff member
          </Button>
        </EmptyContent>
      </Empty>
    )
  }

  return (
    <Grid
      gap={4}
      className="grid-cols-1 sm:grid-cols-2 lg:grid-cols-3 xl:grid-cols-4"
    >
      {staff.map((member) => {
        const initials = member.name
          .split(' ')
          .map((n) => n[0])
          .join('')
          .toUpperCase()

        return (
          <CardPrimitive key={member.id} className="p-3">
            <HStack align="start" justify="between" gap={3}>
              <HStack align="start" gap={3}>
                <Avatar className="h-8 w-8 border border-border/50">
                  <AvatarImage
                    src={
                      member.photo_url ||
                      `https://api.dicebear.com/7.x/avataaars/svg?seed=${member.email}`
                    }
                  />
                  <AvatarFallback className="text-[10px] font-semibold">
                    {initials}
                  </AvatarFallback>
                </Avatar>

                <Stack gap={1}>
                  <HStack gap={2} align="center">
                    <Text size="sm" className="truncate font-medium">
                      {member.name}
                    </Text>
                    <Badge
                      variant="secondary"
                      className="text-[10px] px-1.5 py-0"
                    >
                      {member.staff_type}
                    </Badge>
                  </HStack>

                  <Text size="xs" muted className="truncate">
                    {member.email}
                  </Text>

                  <Text size="xs" muted>
                    {format(new Date(member.created_at), 'MMM d, yyyy')}
                  </Text>

                </Stack>
              </HStack>

              <DropdownMenu>
                <DropdownMenuTrigger
                  render={
                    <Button
                      variant="ghost"
                      size="icon"
                      className="h-fit w-fit p-1"
                    >
                      <HugeiconsIcon
                        icon={MoreVerticalIcon}
                        className="size-4"
                      />
                    </Button>
                  }
                />
                <DropdownMenuContent align="end">
                  <DropdownMenuItem onClick={() => onEdit(member)}>
                    <HStack gap={2} p={0}>
                      <HugeiconsIcon
                        icon={PencilEdit01Icon}
                        className="size-4 opacity-70"
                      />
                      <span>Edit</span>
                    </HStack>
                  </DropdownMenuItem>
                  <DropdownMenuSeparator />
                  <DropdownMenuItem
                    onClick={() => onDelete(member.id)}
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
          </CardPrimitive>
        )
      })}
    </Grid>
  )
}
