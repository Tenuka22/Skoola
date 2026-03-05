import {
  Delete02Icon,
  Layers02Icon,
  MoreHorizontalIcon,
  PencilEdit01Icon,
} from '@hugeicons/core-free-icons'
import { HugeiconsIcon } from '@hugeicons/react'
import type { GradeLevelResponse } from '@/lib/api/types.gen'
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

interface GradeLevelsGridViewProps {
  data: Array<GradeLevelResponse>
  isLoading: boolean
  onEdit: (grade: GradeLevelResponse) => void
  onDelete: (id: string) => void
}

export function GradeLevelsGridView({
  data,
  isLoading,
  onEdit,
  onDelete,
}: GradeLevelsGridViewProps) {
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
              </Stack>
            </Stack>
          </Card>
        ))}
      </Grid>
    )
  }

  return (
    <Grid cols={4} gap={4}>
      {data.map((grade) => (
        <Card key={grade.id} className="p-3">
          <Stack gap={3}>
            <HStack justify="between" align="start">
              <HStack gap={3} align="start">
                <Stack gap={1}>
                  <HStack gap={2} align="center">
                    <Heading size="h4">{grade.grade_name}</Heading>
                    <Badge
                      variant="secondary"
                      className="text-[10px] px-1.5 py-0"
                    >
                      {grade.education_level}
                    </Badge>
                  </HStack>
                  <Text size="xs" muted>
                    ID: {grade.id}
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
                  <DropdownMenuItem onClick={() => onEdit(grade)}>
                    <HugeiconsIcon
                      icon={PencilEdit01Icon}
                      className="size-4 mr-2"
                    />
                    Edit Details
                  </DropdownMenuItem>
                  <DropdownMenuSeparator />
                  <DropdownMenuItem
                    onClick={() => onDelete(grade.id)}
                    variant="destructive"
                  >
                    <HugeiconsIcon
                      icon={Delete02Icon}
                      className="size-4 mr-2"
                    />
                    Delete Grade
                  </DropdownMenuItem>
                </DropdownMenuContent>
              </DropdownMenu>
            </HStack>

            <Stack gap={2}>
              <HStack gap={2} align="center">
                <HugeiconsIcon
                  icon={Layers02Icon}
                  className="size-4 text-muted-foreground"
                />
                <Text size="sm">Grade Number: {grade.grade_number}</Text>
              </HStack>
            </Stack>

            <HStack className="mt-2 pt-2 border-t border-border/50">
              <Button
                variant="outline"
                size="sm"
                className="flex-1 h-8 text-xs"
                onClick={() => onEdit(grade)}
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
