import { HugeiconsIcon } from '@hugeicons/react'
import { FloppyDiskIcon } from '@hugeicons/core-free-icons'
import { useForm } from 'react-hook-form'
import { zodResolver } from '@hookform/resolvers/zod'
import { useEffect } from 'react'
import { useQuery } from '@tanstack/react-query'
import { timetableEntryFormSchema } from '../schemas'
import type {
  AcademicYearResponse,
  ClassResponse,
  StaffResponse,
  TimetableResponse,
} from '@/lib/api/types.gen'
import type { TimetableEntryFormValues } from '../schemas'
import { Button } from '@/components/ui/button'
import {
  Dialog,
  DialogContent,
  DialogFooter,
  DialogHeader,
  DialogTitle,
} from '@/components/ui/dialog'
import { Input } from '@/components/ui/input'
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
import { getAllSubjectsOptions } from '@/lib/api/@tanstack/react-query.gen'

interface TimetableEditDialogProps {
  timetableEntry: TimetableResponse | null
  open: boolean
  onOpenChange: (open: boolean) => void
  onConfirm: (data: TimetableEntryFormValues) => void
  isSubmitting?: boolean
  academicYears: Array<AcademicYearResponse>
  classes: Array<ClassResponse>
  staff: Array<StaffResponse>
}

export function TimetableEditDialog({
  timetableEntry,
  open,
  onOpenChange,
  onConfirm,
  isSubmitting,
  academicYears,
  classes,
  staff,
}: TimetableEditDialogProps) {
  const form = useForm<TimetableEntryFormValues>({
    resolver: zodResolver(timetableEntryFormSchema),
    defaultValues: {
      class_id: '',
      subject_id: '',
      teacher_id: '',
      academic_year_id: '',
      day_of_week: '',
      start_time: '',
      end_time: '',
      room: '',
      period_number: 1,
    },
  })

  const { data: subjectsData } = useQuery({
    ...getAllSubjectsOptions({ client: authClient }),
    staleTime: Infinity,
  })
  const subjects = subjectsData?.data || []

  const daysOfWeek = [
    'Monday',
    'Tuesday',
    'Wednesday',
    'Thursday',
    'Friday',
    'Saturday',
    'Sunday',
  ]

  useEffect(() => {
    if (timetableEntry) {
      form.reset({
        class_id: timetableEntry.class_id,
        subject_id: timetableEntry.subject_id,
        teacher_id: timetableEntry.teacher_id,
        academic_year_id: timetableEntry.academic_year_id,
        day_of_week: timetableEntry.day_of_week,
        start_time: timetableEntry.start_time,
        end_time: timetableEntry.end_time,
        room: timetableEntry.room || '',
        period_number: timetableEntry.period_number,
      })
    }
  }, [timetableEntry, form])

  const handleSubmit = (data: TimetableEntryFormValues) => {
    onConfirm(data)
  }

  return (
    <Dialog
      open={open}
      onOpenChange={(val) => {
        if (!val) form.reset()
        onOpenChange(val)
      }}
    >
      <DialogContent className="max-w-md">
        <DialogHeader>
          <DialogTitle>Edit Timetable Entry</DialogTitle>
        </DialogHeader>
        <form
          onSubmit={form.handleSubmit(handleSubmit)}
          className="grid gap-4 py-4"
        >
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
                {classes.map((cls) => (
                  <SelectItem key={cls.id} value={cls.id}>
                    {cls.section_name}
                  </SelectItem>
                ))}
              </SelectContent>
            </Select>
            {form.formState.errors.class_id && (
              <p className="col-span-4 col-start-2 text-sm font-medium text-red-500">
                {form.formState.errors.class_id.message}
              </p>
            )}
          </div>
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
            <Label htmlFor="teacher_id" className="text-right">
              Teacher
            </Label>
            <Select
              onValueChange={(value) =>
                form.setValue('teacher_id', value || '')
              }
              value={form.watch('teacher_id')}
            >
              <SelectTrigger id="teacher_id" className="col-span-3">
                <SelectValue placeholder="Select a teacher" />
              </SelectTrigger>
              <SelectContent>
                {staff.map((teacher) => (
                  <SelectItem key={teacher.id} value={teacher.id}>
                    {teacher.name}
                  </SelectItem>
                ))}
              </SelectContent>
            </Select>
            {form.formState.errors.teacher_id && (
              <p className="col-span-4 col-start-2 text-sm font-medium text-red-500">
                {form.formState.errors.teacher_id.message}
              </p>
            )}
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
          <div className="grid grid-cols-4 items-center gap-4">
            <Label htmlFor="day_of_week" className="text-right text-xs">
              Day of Week
            </Label>
            <Select
              onValueChange={(value) =>
                form.setValue('day_of_week', value || '')
              }
              value={form.watch('day_of_week')}
            >
              <SelectTrigger id="day_of_week" className="col-span-3">
                <SelectValue placeholder="Select a day" />
              </SelectTrigger>
              <SelectContent>
                {daysOfWeek.map((day) => (
                  <SelectItem key={day} value={day}>
                    {day}
                  </SelectItem>
                ))}
              </SelectContent>
            </Select>
            {form.formState.errors.day_of_week && (
              <p className="col-span-4 col-start-2 text-sm font-medium text-red-500">
                {form.formState.errors.day_of_week.message}
              </p>
            )}
          </div>
          <div className="grid grid-cols-4 items-center gap-4">
            <Label htmlFor="start_time" className="text-right">
              Start Time
            </Label>
            <Input
              id="start_time"
              type="time"
              {...form.register('start_time')}
              className="col-span-3"
            />
            {form.formState.errors.start_time && (
              <p className="col-span-4 col-start-2 text-sm font-medium text-red-500">
                {form.formState.errors.start_time.message}
              </p>
            )}
          </div>
          <div className="grid grid-cols-4 items-center gap-4">
            <Label htmlFor="end_time" className="text-right">
              End Time
            </Label>
            <Input
              id="end_time"
              type="time"
              {...form.register('end_time')}
              className="col-span-3"
            />
            {form.formState.errors.end_time && (
              <p className="col-span-4 col-start-2 text-sm font-medium text-red-500">
                {form.formState.errors.end_time.message}
              </p>
            )}
          </div>
          <div className="grid grid-cols-4 items-center gap-4">
            <Label htmlFor="period_number" className="text-right">
              Period
            </Label>
            <Input
              id="period_number"
              type="number"
              {...form.register('period_number', { valueAsNumber: true })}
              className="col-span-3"
            />
            {form.formState.errors.period_number && (
              <p className="col-span-4 col-start-2 text-sm font-medium text-red-500">
                {form.formState.errors.period_number.message}
              </p>
            )}
          </div>
          <div className="grid grid-cols-4 items-center gap-4">
            <Label htmlFor="room" className="text-right">
              Room
            </Label>
            <Input
              id="room"
              {...form.register('room')}
              className="col-span-3"
            />
            {form.formState.errors.room && (
              <p className="col-span-4 col-start-2 text-sm font-medium text-red-500">
                {form.formState.errors.room.message}
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
                <HugeiconsIcon icon={FloppyDiskIcon} className="size-4 mr-2" />
              )}
              Save Changes
            </Button>
          </DialogFooter>
        </form>
      </DialogContent>
    </Dialog>
  )
}
