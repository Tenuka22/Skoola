import {
  Calendar01Icon,
  Delete02Icon,
  MoreHorizontalIcon,
  PencilEdit01Icon,
  StarIcon,
} from '@hugeicons/core-free-icons'
import { HugeiconsIcon } from '@hugeicons/react'
import type { AcademicYearResponse } from '@/lib/api/types.gen'
import { Badge } from '@/components/ui/badge'
import { Button } from '@/components/ui/button'
import { Card } from '@/components/ui/card'
import {
  DropdownMenu,
  DropdownMenuContent,
  DropdownMenuItem,
  DropdownMenuSeparator,
  DropdownMenuTrigger,
} from '@/components/ui/dropdown-menu'
import { Grid, HStack, Heading, Stack, Text } from '@/components/primitives'
import { Skeleton } from '@/components/ui/skeleton'

interface AcademicYearsGridViewProps {
  data: Array<AcademicYearResponse>
  isLoading: boolean
  onEdit: (year: AcademicYearResponse) => void
  onDelete: (id: string) => void
  onSetCurrent: (id: string) => void
}

export function AcademicYearsGridView({
  data,
  isLoading,
  onEdit,
  onDelete,
  onSetCurrent,
}: AcademicYearsGridViewProps) {
  if (isLoading) {
    return (
      <Grid cols={4} gap={4}>
        {Array.from({ length: 8 }).map((_, i) => (
          <Card key={i} className="p-3">
            <Stack gap={3}>
              <HStack justify="between" align="start">
                <Skeleton className="h-6 w-32 rounded-md" />
                <Skeleton className="h-8 w-8 rounded-md" />
              </HStack>
              <Stack gap={2}>
                <Skeleton className="h-4 w-full rounded-md" />
                <Skeleton className="h-4 w-3/4 rounded-md" />
              </Stack>
            </Stack>
          </Card>
        ))}
      </Grid>
    )
  }

  return (
    <Grid cols={4} gap={4}>
      {data.map((year) => (
        <Card key={year.id} className="p-3">
          <Stack gap={3}>
            <HStack justify="between" align="start">
              <HStack gap={3} align="start">
                <Stack gap={1}>
                  <HStack gap={2} align="center">
                    <Heading size="h4">{year.name}</Heading>
                    {year.current && (
                      <Badge
                        variant="secondary"
                        className="text-[10px] px-1.5 py-0"
                      >
                        Active
                      </Badge>
                    )}
                  </HStack>
                  <Text size="xs" muted>
                    ID: {year.id}
                  </Text>
                </Stack>
              </HStack>

              <DropdownMenu>
                <DropdownMenuTrigger
                  render={
                    <Button variant="ghost" size="icon" className="size-8">
                      <HugeiconsIcon
                        icon={MoreHorizontalIcon}
                        className="size-4"
                      />
                    </Button>
                  }
                />
                <DropdownMenuContent align="end">
                  <DropdownMenuItem onClick={() => onEdit(year)}>
                    <HugeiconsIcon
                      icon={PencilEdit01Icon}
                      className="size-4 mr-2"
                    />
                    Edit Details
                  </DropdownMenuItem>
                  {!year.current && (
                    <DropdownMenuItem onClick={() => onSetCurrent(year.id)}>
                      <HugeiconsIcon icon={StarIcon} className="size-4 mr-2" />
                      Set as Current
                    </DropdownMenuItem>
                  )}
                  <DropdownMenuSeparator />
                  <DropdownMenuItem
                    onClick={() => onDelete(year.id)}
                    variant="destructive"
                  >
                    <HugeiconsIcon
                      icon={Delete02Icon}
                      className="size-4 mr-2"
                    />
                    Delete Year
                  </DropdownMenuItem>
                </DropdownMenuContent>
              </DropdownMenu>
            </HStack>

            <Stack gap={2}>
              <HStack gap={2} align="center">
                <HugeiconsIcon
                  icon={Calendar01Icon}
                  className="size-4 text-muted-foreground"
                />
                <Text size="sm">
                  {year.year_start} - {year.year_end}
                </Text>
              </HStack>
            </Stack>

            <HStack className="mt-2 pt-2 border-t border-border/50">
              <Button
                variant="outline"
                size="sm"
                className="flex-1 h-8 text-xs"
                onClick={() => onEdit(year)}
              >
                Details
              </Button>
            </HStack>
          </Stack>
        </Card>
      ))}
    </Grid>
  )
}
