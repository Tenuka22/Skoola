import { HugeiconsIcon } from '@hugeicons/react'
import { Tick01Icon } from '@hugeicons/core-free-icons'
import { useForm } from 'react-hook-form'
import { zodResolver } from '@hookform/resolvers/zod'
import { useQueries } from '@tanstack/react-query'
import type { StaffResponse } from '@/lib/api/types.gen'
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
  getAllSubjectsOptions,
} from '@/lib/api/@tanstack/react-query.gen'
import { zAssignSubjectToTeacherRequest } from '@/lib/api/zod.gen'

type FormValues = z.infer<typeof zAssignSubjectToTeacherRequest>

interface StaffAssignSubjectDialogProps {
  staff: StaffResponse | null
  open: boolean
  onOpenChange: (open: boolean) => void
  onConfirm: (staffId: string, data: FormValues) => void
  isSubmitting?: boolean
}

export function StaffAssignSubjectDialog({
  staff,
  open,
  onOpenChange,
  onConfirm,
  isSubmitting,
}: StaffAssignSubjectDialogProps) {
  const form = useForm<FormValues>({
    resolver: zodResolver(zAssignSubjectToTeacherRequest),
    defaultValues: {
      subject_id: '',
      academic_year_id: '',
    },
  })

  const [academicYearsQuery, subjectsQuery] = useQueries({
    queries: [
      {
        ...getAllAcademicYearsOptions({ client: authClient }),
        staleTime: Infinity,
      },
      { ...getAllSubjectsOptions({ client: authClient }), staleTime: Infinity },
    ],
  })

  const academicYears = academicYearsQuery.data?.data || []
  const subjects = subjectsQuery.data?.data || []

  const handleSubmit = (data: FormValues) => {
    if (staff) {
      onConfirm(staff.id, data)
    }
  }

  return (
    <Dialog
      open={open}
      onOpenChange={(val) => {
        if (!val) form.reset()
        onOpenChange(val)
      }}
    >
      <DialogContent>
        <DialogHeader>
          <DialogTitle>Assign Subject to {staff?.name}</DialogTitle>
        </DialogHeader>
        <form
          onSubmit={form.handleSubmit(handleSubmit)}
          className="grid gap-4 py-4"
        >
          <p className="text-sm text-muted-foreground">
            Assign{' '}
            <span className="font-medium text-foreground">{staff?.name}</span>{' '}
            to a specific subject for an academic year.
          </p>
          <div className="grid grid-cols-4 items-center gap-4">
            <Label htmlFor="subject_id" className="text-right">
              Subject
            </Label>
            <Select
              onValueChange={(value) =>
                form.setValue('subject_id', value || '')
              }
              value={form.watch('subject_id')}
            >
              <SelectTrigger id="subject_id" className="col-span-3">
                <SelectValue placeholder="Select a subject" />
              </SelectTrigger>
              <SelectContent>
                {subjects.map((subject) => (
                  <SelectItem key={subject.id} value={subject.id}>
                    {subject.subject_name_en}
                  </SelectItem>
                ))}
              </SelectContent>
            </Select>
            {form.formState.errors.subject_id && (
              <p className="col-span-4 col-start-2 text-sm font-medium text-red-500">
                {form.formState.errors.subject_id.message}
              </p>
            )}
          </div>
          <div className="grid grid-cols-4 items-center gap-4">
            <Label htmlFor="academic_year_id" className="text-right">
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
              Assign Subject
            </Button>
          </DialogFooter>
        </form>
      </DialogContent>
    </Dialog>
  )
}
