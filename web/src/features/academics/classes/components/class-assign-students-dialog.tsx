import * as React from 'react'
import {
  ArrowLeft01Icon,
  ArrowRight01Icon,
  Calendar01Icon,
  Search01Icon,
  Tick01Icon,
} from '@hugeicons/core-free-icons'
import { HugeiconsIcon } from '@hugeicons/react'
import { useMutation, useQuery, useQueryClient } from '@tanstack/react-query'
import { toast } from 'sonner'
import { format } from 'date-fns'
import type { ClassResponse } from '@/lib/api/types.gen'
import {
  bulkAssignStudentsToClassesMutation,
  getAllStudentsOptions,
} from '@/lib/api/@tanstack/react-query.gen'
import { HStack, Stack, Text } from '@/components/primitives'
import { Avatar, AvatarFallback, AvatarImage } from '@/components/ui/avatar'
import { Button } from '@/components/ui/button'
import { Checkbox } from '@/components/ui/checkbox'
import {
  Dialog,
  DialogContent,
  DialogDescription,
  DialogFooter,
  DialogHeader,
  DialogTitle,
} from '@/components/ui/dialog'
import { Input } from '@/components/ui/input'
import { ScrollArea } from '@/components/ui/scroll-area'
import { Spinner } from '@/components/ui/spinner'
import { authClient } from '@/lib/clients'
import {
  Popover,
  PopoverContent,
  PopoverTrigger,
} from '@/components/ui/popover'
import { Calendar } from '@/components/ui/calendar'
import { cn } from '@/lib/utils'

interface ClassAssignStudentsDialogProps {
  classItem: ClassResponse | null
  open: boolean
  onOpenChange: (open: boolean) => void
}

export function ClassAssignStudentsDialog({
  classItem,
  open,
  onOpenChange,
}: ClassAssignStudentsDialogProps) {
  const [search, setSearch] = React.useState('')
  const [page, setPage] = React.useState(1)
  const [selectedStudentIds, setSelectedStudentIds] = React.useState<
    Set<string>
  >(new Set())
  const [fromDate, setFromDate] = React.useState<Date>(new Date())

  const queryClient = useQueryClient()

  const studentsQuery = useQuery({
    ...getAllStudentsOptions({
      client: authClient,
      query: {
        limit: 50,
        page: page,
        search: search || undefined,
      },
    }),
    enabled: open,
  })

  // Reset page when search changes
  React.useEffect(() => {
    setPage(1)
  }, [search])

  const assignMutation = useMutation({
    ...bulkAssignStudentsToClassesMutation({ client: authClient }),
    onSuccess: () => {
      toast.success('Students assigned to class successfully.')
      queryClient.invalidateQueries({ queryKey: ['getAllClasses'] })
      onOpenChange(false)
      setSelectedStudentIds(new Set())
    },
    onError: (error) => {
      toast.error(
        `Failed to assign students: ${error.message || 'Unknown error'}`,
      )
    },
  })

  const handleToggleStudent = (studentId: string) => {
    const next = new Set(selectedStudentIds)
    if (next.has(studentId)) {
      next.delete(studentId)
    } else {
      next.add(studentId)
    }
    setSelectedStudentIds(next)
  }

  const handleConfirm = () => {
    if (!classItem || selectedStudentIds.size === 0) return

    assignMutation.mutate({
      body: {
        assignments: Array.from(selectedStudentIds).map((studentId) => ({
          student_id: studentId,
          class_id: classItem.id,
          grade_id: classItem.grade_id,
          academic_year_id: classItem.academic_year_id,
          from_date: format(fromDate, 'yyyy-MM-dd'),
        })),
      },
    })
  }

  const students = studentsQuery.data?.data || []
  const totalPages = studentsQuery.data?.total_pages || 1

  return (
    <Dialog open={open} onOpenChange={onOpenChange}>
      <DialogContent className="sm:max-w-[500px]">
        <DialogHeader>
          <DialogTitle>
            Assign Students to {classItem?.section_name}
          </DialogTitle>
          <DialogDescription>
            Select students to enroll in this class for the current academic
            year.
          </DialogDescription>
        </DialogHeader>
        <Stack gap={4} className="py-4">
          <Stack gap={1}>
            <Text size="sm" className="font-medium">
              Effective From
            </Text>
            <Popover>
              <PopoverTrigger
                render={
                  <Button
                    variant="outline"
                    className={cn(
                      'w-full justify-start text-left font-normal h-10',
                      !fromDate && 'text-muted-foreground',
                    )}
                  >
                    <HugeiconsIcon
                      icon={Calendar01Icon}
                      className="mr-2 h-4 w-4"
                    />
                    {fromDate ? (
                      format(fromDate, 'PPP')
                    ) : (
                      <span>Pick a date</span>
                    )}
                  </Button>
                }
              />
              <PopoverContent className="w-auto p-0" align="start">
                <Calendar
                  mode="single"
                  selected={fromDate}
                  onSelect={(date) => date && setFromDate(date)}
                  initialFocus
                />
              </PopoverContent>
            </Popover>
          </Stack>

          <Stack gap={1}>
            <div className="relative">
              <HugeiconsIcon
                icon={Search01Icon}
                className="absolute left-3 top-1/2 size-4 -translate-y-1/2 text-muted-foreground"
              />
              <Input
                placeholder="Search students by name or admission number..."
                value={search}
                onChange={(e) => setSearch(e.target.value)}
                className="pl-9"
              />
            </div>
            <Text size="xs" muted>
              Search to find more students. Showing up to 50 students per page.
            </Text>
          </Stack>

          <ScrollArea className="h-[300px] pr-4 border rounded-xl p-2 bg-muted/10">
            {studentsQuery.isLoading ? (
              <div className="flex h-full items-center justify-center">
                <Spinner />
              </div>
            ) : (
              <Stack gap={2}>
                {students.map((student) => (
                  <HStack
                    key={student.id}
                    className={cn(
                      'cursor-pointer rounded-lg border p-2 hover:bg-muted/50 transition-colors',
                      selectedStudentIds.has(student.id) &&
                        'bg-primary/5 border-primary/20',
                    )}
                    onClick={() => handleToggleStudent(student.id)}
                    justify="between"
                  >
                    <HStack gap={3}>
                      <Checkbox
                        checked={selectedStudentIds.has(student.id)}
                        onCheckedChange={() => handleToggleStudent(student.id)}
                      />
                      <Avatar className="size-8">
                        <AvatarImage src={student.profile_photo_url || ''} />
                        <AvatarFallback>
                          {student.name_english[0].toUpperCase()}
                        </AvatarFallback>
                      </Avatar>
                      <Stack gap={0}>
                        <Text size="sm" className="font-medium">
                          {student.name_english}
                        </Text>
                        <Text size="xs" muted>
                          {student.admission_number}
                        </Text>
                      </Stack>
                    </HStack>
                  </HStack>
                ))}
                {students.length === 0 && (
                  <Text size="sm" muted className="text-center py-8">
                    No students found.
                  </Text>
                )}
              </Stack>
            )}
          </ScrollArea>

          {totalPages > 1 && (
            <HStack justify="between" align="center" className="px-1">
              <Text size="xs" muted>
                Page {page} of {totalPages}
              </Text>
              <HStack gap={2}>
                <Button
                  variant="outline"
                  size="sm"
                  className="h-8 px-2"
                  disabled={page <= 1}
                  onClick={() => setPage((p) => p - 1)}
                >
                  <HugeiconsIcon
                    icon={ArrowLeft01Icon}
                    className="size-4 mr-1"
                  />
                  Prev
                </Button>
                <Button
                  variant="outline"
                  size="sm"
                  className="h-8 px-2"
                  disabled={page >= totalPages}
                  onClick={() => setPage((p) => p + 1)}
                >
                  Next
                  <HugeiconsIcon
                    icon={ArrowRight01Icon}
                    className="size-4 ml-1"
                  />
                </Button>
              </HStack>
            </HStack>
          )}
        </Stack>
        <DialogFooter className="gap-2">
          <Button variant="ghost" onClick={() => onOpenChange(false)}>
            Cancel
          </Button>
          <Button
            onClick={handleConfirm}
            className="shadow-md"
            disabled={selectedStudentIds.size === 0 || assignMutation.isPending}
          >
            {assignMutation.isPending ? (
              <Spinner className="mr-2" />
            ) : (
              <HugeiconsIcon icon={Tick01Icon} className="size-4 mr-2" />
            )}
            Assign {selectedStudentIds.size} Students
          </Button>
        </DialogFooter>
      </DialogContent>
    </Dialog>
  )
}
