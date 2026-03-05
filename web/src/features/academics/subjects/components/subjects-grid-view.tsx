import {
  Book01Icon,
  Delete02Icon,
  MoreHorizontalIcon,
  PencilEdit01Icon,
  UserGroupIcon,
} from '@hugeicons/core-free-icons'
import { HugeiconsIcon } from '@hugeicons/react'
import type { SubjectResponse } from '@/lib/api/types.gen'
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

interface SubjectsGridViewProps {
  data: Array<SubjectResponse>
  isLoading: boolean
  onEdit: (subject: SubjectResponse) => void
  onDelete: (id: string) => void
  onAssignToGrade: (subject: SubjectResponse) => void
  onAssignToStream: (subject: SubjectResponse) => void
  onEnrollStudent: (subject: SubjectResponse) => void
  onViewEnrollments: (subject: SubjectResponse) => void
}

export function SubjectsGridView({
  data,
  isLoading,
  onEdit,
  onDelete,
  onAssignToGrade,
  onAssignToStream,
  onEnrollStudent,
  onViewEnrollments,
}: SubjectsGridViewProps) {
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
      {data.map((subject) => {
        return (
          <Card key={subject.id} className="p-3">
            <Stack gap={3}>
              <HStack justify="between" align="start">
                <Stack gap={1}>
                  <HStack gap={2} align="center">
                    <Heading size="h4">{subject.subject_name_en}</Heading>
                    <Badge
                      variant={subject.is_core ? 'default' : 'secondary'}
                      className="text-[10px] px-1.5 py-0 uppercase"
                    >
                      {subject.is_core ? 'Core' : 'Elective'}
                    </Badge>
                  </HStack>
                  <Text size="xs" muted>
                    {subject.subject_code}
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
                    <DropdownMenuItem onClick={() => onEdit(subject)}>
                      <HugeiconsIcon
                        icon={PencilEdit01Icon}
                        className="size-4 mr-2"
                      />
                      Edit Details
                    </DropdownMenuItem>
                    <DropdownMenuItem
                      onClick={() => onViewEnrollments(subject)}
                    >
                      <HugeiconsIcon
                        icon={UserGroupIcon}
                        className="size-4 mr-2"
                      />
                      View Enrollments
                    </DropdownMenuItem>
                    <DropdownMenuSeparator />
                    <DropdownMenuItem onClick={() => onAssignToGrade(subject)}>
                      <HugeiconsIcon
                        icon={Book01Icon}
                        className="size-4 mr-2"
                      />
                      Assign to Grade
                    </DropdownMenuItem>
                    <DropdownMenuItem onClick={() => onAssignToStream(subject)}>
                      <HugeiconsIcon
                        icon={Book01Icon}
                        className="size-4 mr-2"
                      />
                      Assign to Stream
                    </DropdownMenuItem>
                    <DropdownMenuItem onClick={() => onEnrollStudent(subject)}>
                      <HugeiconsIcon
                        icon={UserGroupIcon}
                        className="size-4 mr-2"
                      />
                      Enroll Student
                    </DropdownMenuItem>
                    <DropdownMenuSeparator />
                    <DropdownMenuItem
                      onClick={() => onDelete(subject.id)}
                      variant="destructive"
                    >
                      <HugeiconsIcon
                        icon={Delete02Icon}
                        className="size-4 mr-2"
                      />
                      Delete Subject
                    </DropdownMenuItem>
                  </DropdownMenuContent>
                </DropdownMenu>
              </HStack>

              <HStack className="mt-2 pt-2 border-t border-border/50">
                <Button
                  variant="outline"
                  size="sm"
                  className="flex-1 h-8 text-xs"
                  onClick={() => onEdit(subject)}
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
