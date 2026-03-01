import * as React from 'react'
import { Tick01Icon } from '@hugeicons/core-free-icons'
import { HugeiconsIcon } from '@hugeicons/react'
import { useMutation, useQueries, useQueryClient } from '@tanstack/react-query'
import { toast } from 'sonner'
import type {
  AcademicYearResponse,
  ClassResponse,
  GradeLevelResponse,
} from '@/lib/api/types.gen'
import {
  bulkAssignStudentsToClassesMutation,
  getAllAcademicYearsOptions,
  getAllClassesOptions,
  getAllGradeLevelsOptions,
} from '@/lib/api/@tanstack/react-query.gen'
import { Stack } from '@/components/primitives'
import { Button } from '@/components/ui/button'
import {
  Dialog,
  DialogContent,
  DialogDescription,
  DialogFooter,
  DialogHeader,
  DialogTitle,
} from '@/components/ui/dialog'
import { Input } from '@/components/ui/input'
import { Label } from '@/components/ui/label'
import {
  Select,
  SelectContent,
  SelectItem,
  SelectTrigger,
  SelectValue,
} from '@/components/ui/select'
import { Spinner } from '@/components/ui/spinner'
import { authClient } from '@/lib/clients'

interface StudentBulkAssignClassDialogProps {
  selectedStudentIds: Set<string>
  open: boolean
  onOpenChange: (open: boolean) => void
}

export function StudentBulkAssignClassDialog({
  selectedStudentIds,
  open,
  onOpenChange,
}: StudentBulkAssignClassDialogProps) {
  const [gradeId, setGradeId] = React.useState('')
  const [classId, setClassId] = React.useState('')
  const [academicYearId, setAcademicYearId] = React.useState('')
  const [fromDate, setFromDate] = React.useState(
    new Date().toISOString().split('T')[0],
  )

  const queryClient = useQueryClient()

  const [academicYearsQuery, classesQuery, gradeLevelsQuery] = useQueries({
    queries: [
      {
        ...getAllAcademicYearsOptions({ client: authClient }),
        staleTime: Infinity,
      },
      { ...getAllClassesOptions({ client: authClient }), staleTime: Infinity },
      {
        ...getAllGradeLevelsOptions({ client: authClient }),
        staleTime: Infinity,
      },
    ],
  })

  const academicYears = academicYearsQuery.data?.data || []
  const classes = classesQuery.data?.data || []
  const gradeLevels = gradeLevelsQuery.data?.data || []

  const assignMutation = useMutation({
    ...bulkAssignStudentsToClassesMutation({ client: authClient }),
    onSuccess: () => {
      toast.success('Students assigned to class successfully.')
      queryClient.invalidateQueries({ queryKey: ['getAllStudents'] })
      onOpenChange(false)
    },
    onError: (error) => {
      toast.error(
        `Failed to assign students: ${error.message || 'Unknown error'}`,
      )
    },
  })

  const handleConfirm = () => {
    if (
      !classId ||
      !gradeId ||
      !academicYearId ||
      selectedStudentIds.size === 0
    ) {
      toast.error('Please fill in all required fields.')
      return
    }

    assignMutation.mutate({
      body: {
        assignments: Array.from(selectedStudentIds).map((studentId) => ({
          student_id: studentId,
          class_id: classId,
          grade_id: gradeId,
          academic_year_id: academicYearId,
          from_date: fromDate,
        })),
      },
    })
  }

  return (
    <Dialog open={open} onOpenChange={onOpenChange}>
      <DialogContent className="sm:max-w-[425px]">
        <DialogHeader>
          <DialogTitle>Bulk Assign Students to Class</DialogTitle>
          <DialogDescription>
            Assign {selectedStudentIds.size} selected students to a class.
          </DialogDescription>
        </DialogHeader>
        <Stack gap={4} className="py-4">
          <div className="grid grid-cols-4 items-center gap-4">
            <Label htmlFor="grade_id" className="text-right">
              Grade
            </Label>
            <Select
              onValueChange={(val) => setGradeId(val ?? '')}
              value={gradeId}
            >
              <SelectTrigger id="grade_id" className="col-span-3">
                <SelectValue placeholder="Select a grade level" />
              </SelectTrigger>
              <SelectContent>
                {gradeLevels.map((gl: GradeLevelResponse) => (
                  <SelectItem key={gl.id} value={gl.id}>
                    {gl.grade_name}
                  </SelectItem>
                ))}
              </SelectContent>
            </Select>
          </div>
          <div className="grid grid-cols-4 items-center gap-4">
            <Label htmlFor="class_id" className="text-right">
              Class
            </Label>
            <Select
              onValueChange={(val) => setClassId(val ?? '')}
              value={classId}
            >
              <SelectTrigger id="class_id" className="col-span-3">
                <SelectValue placeholder="Select a class" />
              </SelectTrigger>
              <SelectContent>
                {classes
                  .filter(
                    (c: ClassResponse) => !gradeId || c.grade_id === gradeId,
                  )
                  .map((cls: ClassResponse) => (
                    <SelectItem key={cls.id} value={cls.id}>
                      {cls.section_name}
                    </SelectItem>
                  ))}
              </SelectContent>
            </Select>
          </div>
          <div className="grid grid-cols-4 items-center gap-4">
            <Label htmlFor="academic_year_id" className="text-right">
              Year
            </Label>
            <Select
              onValueChange={(val) => setAcademicYearId(val ?? '')}
              value={academicYearId}
            >
              <SelectTrigger id="academic_year_id" className="col-span-3">
                <SelectValue placeholder="Select academic year" />
              </SelectTrigger>
              <SelectContent>
                {academicYears.map((ay: AcademicYearResponse) => (
                  <SelectItem key={ay.id} value={ay.id}>
                    {ay.name}
                  </SelectItem>
                ))}
              </SelectContent>
            </Select>
          </div>
          <div className="grid grid-cols-4 items-center gap-4">
            <Label htmlFor="from_date" className="text-right text-xs">
              From Date
            </Label>
            <Input
              id="from_date"
              type="date"
              value={fromDate}
              onChange={(e) => setFromDate(e.target.value)}
              className="col-span-3"
            />
          </div>
        </Stack>
        <DialogFooter>
          <Button variant="ghost" onClick={() => onOpenChange(false)}>
            Cancel
          </Button>
          <Button
            onClick={handleConfirm}
            disabled={
              !classId ||
              !gradeId ||
              !academicYearId ||
              assignMutation.isPending
            }
          >
            {assignMutation.isPending ? (
              <Spinner className="mr-2" />
            ) : (
              <HugeiconsIcon icon={Tick01Icon} className="size-4 mr-2" />
            )}
            Assign Students
          </Button>
        </DialogFooter>
      </DialogContent>
    </Dialog>
  )
}
