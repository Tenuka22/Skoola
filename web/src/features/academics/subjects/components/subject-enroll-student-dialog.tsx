import { HugeiconsIcon } from '@hugeicons/react'
import { UserAdd01Icon } from '@hugeicons/core-free-icons'
import { useForm } from 'react-hook-form'
import { zodResolver } from '@hookform/resolvers/zod'
import { useQuery } from '@tanstack/react-query'
import { enrollStudentInSubjectSchema } from '../schemas'
import type { SubjectResponse } from '@/lib/api/types.gen'
import type { z } from 'zod'
import { Button } from '@/components/ui/button'
import {
  Dialog,
  DialogContent,
  DialogFooter,
  DialogHeader,
  DialogTitle,
} from '@/components/ui/dialog'
import { Label } from '@/components/ui/label'
import { Spinner } from '@/components/ui/spinner'
import {
  Select,
  SelectContent,
  SelectItem,
  SelectTrigger,
  SelectValue,
} from '@/components/ui/select'
import { authClient } from '@/lib/clients'
import {
  getAllAcademicYearsOptions,
  getAllStudentsOptions,
} from '@/lib/api/@tanstack/react-query.gen'

type FormValues = z.infer<typeof enrollStudentInSubjectSchema>

interface SubjectEnrollStudentDialogProps {
  subject: SubjectResponse | null
  open: boolean
  onOpenChange: (open: boolean) => void
  onConfirm: (studentId: string, academicYearId: string) => void
  isSubmitting?: boolean
}

export function SubjectEnrollStudentDialog({
  subject,
  open,
  onOpenChange,
  onConfirm,
  isSubmitting,
}: SubjectEnrollStudentDialogProps) {
  const form = useForm<FormValues>({
    resolver: zodResolver(enrollStudentInSubjectSchema),
    defaultValues: {
      student_id: '',
      academic_year_id: '',
      subject_id: subject?.id || '',
    },
  })

  const { data: studentsData } = useQuery(
    getAllStudentsOptions({ client: authClient }),
  )
  const students = studentsData?.data || []

  const { data: academicYearsData } = useQuery(
    getAllAcademicYearsOptions({ client: authClient }),
  )
  const academicYears = academicYearsData?.data || []

  const handleSubmit = (data: FormValues) => {
    onConfirm(data.student_id, data.academic_year_id)
  }

  return (
    <Dialog
      open={open}
      onOpenChange={(val) => {
        if (!val) form.reset()
        if (val && subject) form.setValue('subject_id', subject.id)
        onOpenChange(val)
      }}
    >
      <DialogContent>
        <DialogHeader>
          <DialogTitle>Enroll Student in {subject?.subject_name_en}</DialogTitle>
        </DialogHeader>
        <form onSubmit={form.handleSubmit(handleSubmit)} className="grid gap-4 py-4">
          <p className="text-sm text-muted-foreground">
            Enroll a student into the subject <span className="font-medium text-foreground">{subject?.subject_name_en}</span>.
          </p>
          <div className="grid grid-cols-4 items-center gap-4">
            <Label htmlFor="student_id" className="text-right">
              Student
            </Label>
            <Select
              onValueChange={(value) => form.setValue('student_id', value || '')}
              value={form.watch('student_id')}
            >
              <SelectTrigger className="col-span-3">
                <SelectValue placeholder="Select a student" />
              </SelectTrigger>
              <SelectContent>
                {students.map((student) => (
                  <SelectItem key={student.id} value={student.id}>
                    {student.name_english}
                  </SelectItem>
                ))}
              </SelectContent>
            </Select>
            {form.formState.errors.student_id && (
              <p className="col-span-4 col-start-2 text-sm font-medium text-red-500">
                {form.formState.errors.student_id.message}
              </p>
            )}
          </div>
          <div className="grid grid-cols-4 items-center gap-4">
            <Label htmlFor="academic_year_id" className="text-right">
              Academic Year
            </Label>
            <Select
              onValueChange={(value) => form.setValue('academic_year_id', value || '')}
              value={form.watch('academic_year_id')}
            >
              <SelectTrigger className="col-span-3">
                <SelectValue placeholder="Select an academic year" />
              </SelectTrigger>
              <SelectContent>
                {academicYears.map((ay) => (
                  <SelectItem key={ay.id} value={ay.id}>
                    {ay.name}
                  </SelectItem>
                ))}
              </SelectContent>
            </Select>
            {form.formState.errors.academic_year_id && (
              <p className="col-span-4 col-start-2 text-sm font-medium text-red-500">
                {form.formState.errors.academic_year_id.message}
              </p>
            )}
          </div>
          <DialogFooter className="mt-4">
            <Button type="button" variant="ghost" onClick={() => onOpenChange(false)}>
              Cancel
            </Button>
            <Button type="submit" disabled={isSubmitting}>
              {isSubmitting ? (
                <Spinner className="mr-2" />
              ) : (
                <HugeiconsIcon icon={UserAdd01Icon} className="size-4 mr-2" />
              )}
              Enroll Student
            </Button>
          </DialogFooter>
        </form>
      </DialogContent>
    </Dialog>
  )
}
