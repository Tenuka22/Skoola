import { HugeiconsIcon } from '@hugeicons/react'
import {
  MoreHorizontalIcon,
  PencilEdit01Icon,
  Delete02Icon,
  Clock01Icon,
} from '@hugeicons/core-free-icons'
import type { GradePeriodResponse } from '@/lib/api/types.gen'
import { Card } from '@/components/ui/card'
import { Badge } from '@/components/ui/badge'
import { Button } from '@/components/ui/button'
import {
  DropdownMenu,
  DropdownMenuContent,
  DropdownMenuItem,
  DropdownMenuLabel,
  DropdownMenuSeparator,
  DropdownMenuTrigger,
} from '@/components/ui/dropdown-menu'
import { Grid, HStack, Stack, Text } from '@/components/primitives'

interface GradePeriodsGridViewProps {
  periods: GradePeriodResponse[]
  onEdit: (period: GradePeriodResponse) => void
  onDelete: (period: GradePeriodResponse) => void
  isLoading?: boolean
}

export function GradePeriodsGridView({
  periods,
  onEdit,
  onDelete,
  isLoading,
}: GradePeriodsGridViewProps) {
  if (isLoading) {
    return (
      <Grid cols={1} gap={4} className="md:grid-cols-2 lg:grid-cols-3">
        {[...Array(6)].map((_, i) => (
          <Card key={i} className="p-3 h-24 animate-pulse bg-muted/50" />
        ))}
      </Grid>
    )
  }

  return (
    <Grid cols={1} gap={4} className="md:grid-cols-2 lg:grid-cols-3">
      {periods.map((period) => (
        <Card key={period.id} className="p-3">
          <HStack align="start" justify="between" gap={3}>
            <HStack align="start" gap={3}>
              <Badge
                variant="outline"
                className="h-8 w-8 flex items-center justify-center p-0 rounded-full text-sm font-bold shrink-0"
              >
                {period.period_number}
              </Badge>
              <Stack gap={1}>
                <HStack gap={2} align="center">
                  <Text size="sm" className="font-semibold">
                    {period.is_break ? 'Break / Interval' : 'Lesson Period'}
                  </Text>
                  <Badge
                    variant={period.is_break ? 'secondary' : 'default'}
                    className="text-[10px] px-1.5 py-0"
                  >
                    {period.is_break ? 'Break' : 'Active'}
                  </Badge>
                </HStack>
                <HStack gap={3} className="text-muted-foreground">
                  <HStack gap={1} align="center">
                    <HugeiconsIcon icon={Clock01Icon} className="size-3.5" />
                    <Text size="xs">{period.start_time}</Text>
                  </HStack>
                  <Text size="xs" muted>
                    →
                  </Text>
                  <HStack gap={1} align="center">
                    <HugeiconsIcon icon={Clock01Icon} className="size-3.5" />
                    <Text size="xs">{period.end_time}</Text>
                  </HStack>
                </HStack>
              </Stack>
            </HStack>

            <DropdownMenu>
              <DropdownMenuTrigger
                render={
                  <Button variant="ghost" size="sm" className="h-8 w-8 p-0">
                    <HugeiconsIcon icon={MoreHorizontalIcon} className="size-4" />
                  </Button>
                }
              />
              <DropdownMenuContent align="end">
                <DropdownMenuLabel>Actions</DropdownMenuLabel>
                <DropdownMenuItem onClick={() => onEdit(period)}>
                  <HugeiconsIcon icon={PencilEdit01Icon} className="mr-2 size-4" />
                  Edit
                </DropdownMenuItem>
                <DropdownMenuSeparator />
                <DropdownMenuItem
                  onClick={() => onDelete(period)}
                  className="text-destructive focus:text-destructive"
                >
                  <HugeiconsIcon icon={Delete02Icon} className="mr-2 size-4" />
                  Delete
                </DropdownMenuItem>
              </DropdownMenuContent>
            </DropdownMenu>
          </HStack>
        </Card>
      ))}
    </Grid>
  )
}
