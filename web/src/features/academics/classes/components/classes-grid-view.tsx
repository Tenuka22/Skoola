import {
  Delete02Icon,
  MoreHorizontalIcon,
  PencilEdit01Icon,
  SchoolIcon,
  UserAccountIcon,
  UserGroupIcon,
} from '@hugeicons/core-free-icons'
import { HugeiconsIcon } from '@hugeicons/react'
import { useQuery } from '@tanstack/react-query'
import type { ClassResponse } from '@/lib/api/types.gen'
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
import { authClient } from '@/lib/clients'
import {
  getAllAcademicYearsOptions,
  getAllGradeLevelsOptions,
} from '@/lib/api/@tanstack/react-query.gen'

interface ClassesGridViewProps {
  data: Array<ClassResponse>
  isLoading: boolean
  onEdit: (classItem: ClassResponse) => void
  onDelete: (id: string) => void
  onAssignStudents: (classItem: ClassResponse) => void
}

export function ClassesGridView({
  data,
  isLoading,
  onEdit,
  onDelete,
  onAssignStudents,
}: ClassesGridViewProps) {
  const { data: academicYearsData } = useQuery(
    getAllAcademicYearsOptions({ client: authClient }),
  )
  const academicYears = academicYearsData?.data || []

  const { data: gradeLevelsData } = useQuery(
    getAllGradeLevelsOptions({ client: authClient }),
  )
  const gradeLevels = gradeLevelsData?.data || []

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
      {data.map((classItem) => {
        const gradeLevel = gradeLevels.find(
          (gl) => gl.id === classItem.grade_id,
        )
        const academicYear = academicYears.find(
          (ay) => ay.id === classItem.academic_year_id,
        )

        return (
          <Card key={classItem.id} className="p-3">
            <Stack gap={3}>
              <HStack justify="between" align="start">
                <Stack gap={1}>
                  <HStack gap={2} align="center">
                    <Heading size="h4">{classItem.section_name}</Heading>
                    <Badge
                      variant="secondary"
                      className="text-[10px] px-1.5 py-0 uppercase"
                    >
                      {classItem.medium}
                    </Badge>
                  </HStack>
                  <Text size="xs" muted>
                    {gradeLevel?.grade_name || classItem.grade_id}
                  </Text>
                </Stack>

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
                    <DropdownMenuItem onClick={() => onEdit(classItem)}>
                      <HugeiconsIcon
                        icon={PencilEdit01Icon}
                        className="size-4 mr-2"
                      />
                      Edit Details
                    </DropdownMenuItem>
                    <DropdownMenuItem
                      onClick={() => onAssignStudents(classItem)}
                    >
                      <HugeiconsIcon
                        icon={UserGroupIcon}
                        className="size-4 mr-2"
                      />
                      Assign Students
                    </DropdownMenuItem>
                    <DropdownMenuSeparator />
                    <DropdownMenuItem
                      onClick={() => onDelete(classItem.id)}
                      variant="destructive"
                    >
                      <HugeiconsIcon
                        icon={Delete02Icon}
                        className="size-4 mr-2"
                      />
                      Delete Class
                    </DropdownMenuItem>
                  </DropdownMenuContent>
                </DropdownMenu>
              </HStack>

              <Stack gap={2}>
                <HStack gap={2} align="center">
                  <HugeiconsIcon
                    icon={SchoolIcon}
                    className="size-4 text-muted-foreground"
                  />
                  <Text size="sm" muted>
                    {academicYear?.name || classItem.academic_year_id}
                  </Text>
                </HStack>
                <HStack gap={2} align="center">
                  <HugeiconsIcon
                    icon={UserAccountIcon}
                    className="size-4 text-muted-foreground"
                  />
                  <Text size="sm">Capacity: {classItem.max_capacity}</Text>
                </HStack>
              </Stack>

              <HStack className="mt-2 pt-2 border-t border-border/50">
                <Button
                  variant="outline"
                  size="sm"
                  className="flex-1 h-8 text-xs"
                  onClick={() => onEdit(classItem)}
                >
                  Details
                </Button>
              </HStack>
            </Stack>
          </Card>
        )
      })}
    </Grid>
  )
}
