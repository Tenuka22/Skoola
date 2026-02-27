import { format } from 'date-fns'
import { HugeiconsIcon } from '@hugeicons/react'
import {
  Calendar01Icon,
  Delete02Icon,
  Mail01Icon,
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
import {
  Box,
  Grid,
  HStack,
  Heading,
  Stack,
  Text,
} from '@/components/primitives'

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
      <Box className="flex h-64 flex-col items-center justify-center rounded-xl border border-dashed bg-muted/10">
        <Text muted>No staff found</Text>
      </Box>
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
          <CardPrimitive
            key={member.id}
            className="p-0 overflow-hidden border-border/60 shadow-none bg-card"
          >
            <CardHeader>
              <HStack align="start" className="justify-between p-4">
                <HStack gap={3}>
                  <Avatar className="h-10 w-10 border border-border/50">
                    <AvatarImage
                      src={
                        member.photo_url ||
                        `https://api.dicebear.com/7.x/avataaars/svg?seed=${member.email}`
                      }
                    />
                    <AvatarFallback className="bg-primary/10 text-primary text-xs font-bold">
                      {initials}
                    </AvatarFallback>
                  </Avatar>
                  <Stack gap={1}>
                    <Heading
                      size="h4"
                      className="text-base leading-none tracking-tight"
                    >
                      {member.name}
                    </Heading>
                    <Text size="xs" muted>
                      {member.employment_status}
                    </Text>
                  </Stack>
                </HStack>
                <DropdownMenu>
                  <DropdownMenuTrigger
                    render={
                      <Button
                        variant="ghost"
                        size="icon"
                        className="h-8 w-8 -mr-2 text-muted-foreground"
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
                      <HugeiconsIcon
                        icon={PencilEdit01Icon}
                        className="mr-2 size-4 opacity-70"
                      />
                      Edit
                    </DropdownMenuItem>
                    <DropdownMenuSeparator />
                    <DropdownMenuItem
                      onClick={() => onDelete(member.id)}
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
            </CardHeader>

            <CardContent className="p-4 pt-0">
              <Stack gap={3}>
                <HStack gap={2}>
                  <HugeiconsIcon
                    icon={Mail01Icon}
                    className="size-3.5 text-muted-foreground"
                  />
                  <Text size="xs" muted className="truncate">
                    {member.email}
                  </Text>
                </HStack>
                <HStack gap={2}>
                  <HugeiconsIcon
                    icon={Calendar01Icon}
                    className="size-3.5 text-muted-foreground"
                  />
                  <Text size="xs" muted>
                    Joined {format(new Date(member.created_at), 'MMM d, yyyy')}
                  </Text>
                </HStack>
              </Stack>
            </CardContent>

            <Box className="m-4 mt-0">
              <Badge
                variant="outline"
                className="border-0 bg-transparent px-0 font-medium text-blue-500"
              >
                <span className="mr-1.5 inline-block h-1.5 w-1.5 rounded-full bg-blue-500" />
                {member.employment_status}
              </Badge>
            </Box>
          </CardPrimitive>
        )
      })}
    </Grid>
  )
}
