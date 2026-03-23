import { GridViewIcon } from '@hugeicons/core-free-icons'
import { HugeiconsIcon } from '@hugeicons/react'
import type { Staff } from '../types'
import { Card } from '@/components/ui/card'
import { Button } from '@/components/ui/button'
import { Avatar, AvatarFallback, AvatarImage } from '@/components/ui/avatar'
import { Badge } from '@/components/ui/badge'
import {
  DropdownMenu,
  DropdownMenuContent,
  DropdownMenuItem,
  DropdownMenuTrigger,
} from '@/components/ui/dropdown-menu'
import { HStack, Stack, Text } from '@/components/primitives'
import { cn } from '@/lib/utils'

interface StaffGridViewProps {
  staff: Array<Staff>
  limit: number
  isLoading: boolean
  onEdit: (staff: Staff) => void
  onDelete: (id: string) => void
  onViewProfile?: (staff: Staff) => void
  onCreateStaff: () => void
}

export function StaffGridView({
  staff,
  limit,
  isLoading,
  onEdit,
  onDelete,
  onViewProfile,
  onCreateStaff,
}: StaffGridViewProps) {
  if (isLoading) {
    return (
      <div className="grid gap-4 md:grid-cols-2 lg:grid-cols-3 xl:grid-cols-4">
        {Array.from({ length: limit }).map((_, i) => (
          <Card key={i} className="p-3 animate-pulse">
            <HStack gap={3}>
              <div className="h-12 w-12 rounded-full bg-muted" />
              <Stack gap={2} className="flex-1">
                <div className="h-4 w-32 bg-muted rounded" />
                <div className="h-3 w-24 bg-muted rounded" />
              </Stack>
            </HStack>
          </Card>
        ))}
      </div>
    )
  }

  if (staff.length === 0) {
    return (
      <Card className="p-8">
        <Stack gap={4} className="items-center justify-center text-center">
          <Text muted>No staff members found</Text>
          <Button onClick={onCreateStaff}>Add Staff Member</Button>
        </Stack>
      </Card>
    )
  }

  return (
    <div className="grid gap-4 md:grid-cols-2 lg:grid-cols-3 xl:grid-cols-4">
      {staff.map((staffMember) => (
        <Card key={staffMember.id} className="p-3">
          <HStack align="start" justify="between" gap={3}>
            <HStack align="start" gap={3}>
              <Avatar className="h-12 w-12">
                <AvatarImage
                  src={staffMember.photo_url ?? undefined}
                  alt={staffMember.name}
                />
                <AvatarFallback>
                  {staffMember.name
                    .split(' ')
                    .map((n) => n[0])
                    .join('')
                    .toUpperCase()}
                </AvatarFallback>
              </Avatar>
              <Stack gap={1}>
                <HStack gap={2} align="center">
                  <Text size="sm" className="font-medium">
                    {staffMember.name}
                  </Text>
                </HStack>
                <Text size="xs" muted>
                  {staffMember.email ?? 'No email'}
                </Text>
                <HStack gap={2}>
                  <Badge
                    variant="secondary"
                    className={cn(
                      'text-[10px] px-1.5 py-0',
                      staffMember.staff_type === 'Teaching' &&
                        'bg-blue-500/10 text-blue-500',
                      staffMember.staff_type === 'NonTeaching' &&
                        'bg-purple-500/10 text-purple-500',
                      staffMember.staff_type === 'Administrative' &&
                        'bg-orange-500/10 text-orange-500',
                    )}
                  >
                    {staffMember.staff_type}
                  </Badge>
                  {staffMember.employment_status && (
                    <Badge
                      variant="outline"
                      className={cn(
                        'text-[10px] px-1.5 py-0',
                        staffMember.employment_status === 'Permanent'
                          ? 'bg-green-500/10 text-green-500'
                          : staffMember.employment_status === 'Contract'
                            ? 'bg-blue-500/10 text-blue-500'
                            : 'bg-gray-500/10 text-gray-500',
                      )}
                    >
                      {staffMember.employment_status}
                    </Badge>
                  )}
                </HStack>
              </Stack>
            </HStack>
            <DropdownMenu>
              <DropdownMenuTrigger
                render={
                  <Button variant="ghost" size="icon" className="h-8 w-8">
                    <HugeiconsIcon icon={GridViewIcon} className="h-4 w-4" />
                  </Button>
                }
              ></DropdownMenuTrigger>
              <DropdownMenuContent align="end">
                <DropdownMenuItem onClick={() => onEdit(staffMember)}>
                  Edit
                </DropdownMenuItem>
                {onViewProfile && (
                  <DropdownMenuItem onClick={() => onViewProfile(staffMember)}>
                    View Profile
                  </DropdownMenuItem>
                )}
                <DropdownMenuItem
                  onClick={() => onDelete(staffMember.id)}
                  className="text-destructive"
                >
                  Delete
                </DropdownMenuItem>
              </DropdownMenuContent>
            </DropdownMenu>
          </HStack>
        </Card>
      ))}
    </div>
  )
}
