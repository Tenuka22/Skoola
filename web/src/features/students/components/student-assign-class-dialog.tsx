import { HugeiconsIcon } from '@hugeicons/react'
import { Tick01Icon } from '@hugeicons/core-free-icons'
import * as React from 'react'
import { useQueries } from '@tanstack/react-query'
import type { z } from 'zod'
import type {
  AcademicYearResponse,
  ClassResponse,
  GradeLevelResponse,
  StudentResponse,
} from '@/lib/api/types.gen'
import type { UseFormReturn } from 'react-hook-form'
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
import { Input } from '@/components/ui/input'
import { authClient } from '@/lib/clients'
import {
  getAllAcademicYearsOptions,
  getAllClassesOptions,
  getAllGradeLevelsOptions,
} from '@/lib/api/@tanstack/react-query.gen'
import { zCreateStudentClassAssignmentRequest } from '@/lib/api/zod.gen'
import { FormBuilder, defineFormConfig } from '@/components/form-builder'

const formSchema = zCreateStudentClassAssignmentRequest.omit({
  student_id: true,
  to_date: true,
})

type FormValues = z.infer<typeof formSchema>

interface StudentAssignClassDialogProps {
  student: StudentResponse | null
  open: boolean
  onOpenChange: (open: boolean) => void
  onConfirm: (studentId: string, data: FormValues) => void
  isSubmitting?: boolean
}

export function StudentAssignClassDialog({
  student,
  open,
  onOpenChange,
  onConfirm,
  isSubmitting,
}: StudentAssignClassDialogProps) {
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

  const preload = React.useCallback(
    (form: UseFormReturn<FormValues, unknown, FormValues>) => {
      if (!open) {
        form.reset()
      }
    },
    [open],
  )

  const config = defineFormConfig(formSchema, {
    structure: [],
    extras: {
      top: (form) => (
        <>
          <div className="grid grid-cols-4 items-center gap-4">
            <Label htmlFor="grade_id" className="text-right">
              Grade
            </Label>
            <Select
              onValueChange={(value) => form.setValue('grade_id', value || '')}
              value={form.watch('grade_id')}
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
              onValueChange={(value) => form.setValue('class_id', value || '')}
              value={form.watch('class_id')}
            >
              <SelectTrigger id="class_id" className="col-span-3">
                <SelectValue placeholder="Select a class" />
              </SelectTrigger>
              <SelectContent>
                {classes
                  .filter(
                    (c: ClassResponse) =>
                      !form.watch('grade_id') ||
                      c.grade_id === form.watch('grade_id'),
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
            <Label htmlFor="academic_year_id" className="text-right text-xs">
              Academic Year
            </Label>
            <Select
              onValueChange={(value) =>
                form.setValue('academic_year_id', value || '')
              }
              value={form.watch('academic_year_id')}
            >
              <SelectTrigger id="academic_year_id" className="col-span-3">
                <SelectValue placeholder="Select an academic year" />
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
            <Label htmlFor="from_date" className="text-right">
              From Date
            </Label>
            <Input
              id="from_date"
              type="date"
              {...form.register('from_date')}
              className="col-span-3"
            />
          </div>
        </>
      ),
      bottom: (
        <DialogFooter className="mt-4">
          <Button
            type="button"
            variant="ghost"
            onClick={() => onOpenChange(false)}
          >
            Cancel
          </Button>
          <Button type="submit" disabled={isSubmitting}>
            {isSubmitting ? (
              <Spinner className="mr-2" />
            ) : (
              <HugeiconsIcon icon={Tick01Icon} className="size-4 mr-2" />
            )}
            Assign Class
          </Button>
        </DialogFooter>
      ),
    },
  })

  return (
    <Dialog open={open} onOpenChange={(val) => onOpenChange(val)}>
      <DialogContent>
        <DialogHeader>
          <DialogTitle>Assign Class to {student?.name_english}</DialogTitle>
        </DialogHeader>
        <FormBuilder
          schema={formSchema}
          config={config}
          defaultValues={{
            class_id: '',
            academic_year_id: '',
            grade_id: '',
            from_date: new Date().toISOString().split('T')[0],
          }}
          onSubmit={(values) => {
            if (student) onConfirm(student.id, values)
          }}
          preload={preload}
          isLoading={isSubmitting}
          showErrorSummary={false}
          toastErrors={false}
          showSuccessAlert={false}
          actions={[]}
          className="grid gap-4 py-4"
        />
      </DialogContent>
    </Dialog>
  )
}
