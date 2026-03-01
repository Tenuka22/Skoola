import * as React from 'react'
import { Search01Icon, Tick01Icon } from '@hugeicons/core-free-icons'
import { HugeiconsIcon } from '@hugeicons/react'
import { useMutation, useQuery, useQueryClient } from '@tanstack/react-query'
import { toast } from 'sonner'
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
  DialogFooter,
  DialogHeader,
  DialogTitle,
} from '@/components/ui/dialog'
import { Input } from '@/components/ui/input'
import { ScrollArea } from '@/components/ui/scroll-area'
import { Spinner } from '@/components/ui/spinner'
import { authClient } from '@/lib/clients'

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
  const [selectedStudentIds, setSelectedStudentIds] = React.useState<
    Set<string>
  >(new Set())
  const [fromDate, setFromDate] = React.useState(
    new Date().toISOString().split('T')[0],
  )

  const queryClient = useQueryClient()

  const studentsQuery = useQuery({
    ...getAllStudentsOptions({
      client: authClient,
      query: {
        limit: 100, // For simplicity, just fetch top 100
        search: search || undefined,
      },
    }),
    enabled: open,
  })

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
          from_date: fromDate,
        })),
      },
    })
  }

  const students = studentsQuery.data?.data || []

  return (
    <Dialog open={open} onOpenChange={onOpenChange}>
      <DialogContent className="sm:max-w-[500px]">
        <DialogHeader>
          <DialogTitle>
            Assign Students to {classItem?.section_name}
          </DialogTitle>
        </DialogHeader>
        <Stack gap={4} className="py-4">
          <div className="grid grid-cols-4 items-center gap-4">
            <Text className="text-right">From Date</Text>
            <Input
              type="date"
              value={fromDate}
              onChange={(e) => setFromDate(e.target.value)}
              className="col-span-3"
            />
          </div>
          <div className="relative">
            <HugeiconsIcon
              icon={Search01Icon}
              className="absolute left-3 top-1/2 size-4 -translate-y-1/2 text-muted-foreground"
            />
            <Input
              placeholder="Search students..."
              value={search}
              onChange={(e) => setSearch(e.target.value)}
              className="pl-9"
            />
          </div>
          <ScrollArea className="h-[300px] pr-4">
            {studentsQuery.isLoading ? (
              <div className="flex h-full items-center justify-center">
                <Spinner />
              </div>
            ) : (
              <Stack gap={2}>
                {students.map((student) => (
                  <HStack
                    key={student.id}
                    className="cursor-pointer rounded-lg border p-2 hover:bg-muted/50"
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
                  <Text size="sm" muted className="text-center py-4">
                    No students found.
                  </Text>
                )}
              </Stack>
            )}
          </ScrollArea>
        </Stack>
        <DialogFooter>
          <Button variant="ghost" onClick={() => onOpenChange(false)}>
            Cancel
          </Button>
          <Button
            onClick={handleConfirm}
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
