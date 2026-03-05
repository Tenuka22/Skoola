import * as React from 'react'
import { useQuery } from '@tanstack/react-query'
import { useForm, useWatch, Controller } from 'react-hook-form'
import { zodResolver } from '@hookform/resolvers/zod'
import { DAYS_OF_WEEK } from '../constants'
import { timetableEntryFormSchema } from '../schemas'
import type {
  AcademicYearResponse,
  ClassResponse,
  StaffResponse,
} from '@/lib/api/types.gen'
import type { TimetableEntryFormValues } from '../schemas'
import {
  Dialog,
  DialogContent,
  DialogHeader,
  DialogTitle,
  DialogFooter,
} from '@/components/ui/dialog'
import { authClient } from '@/lib/clients'
import { getAllSubjectsOptions } from '@/lib/api/@tanstack/react-query.gen'
import { getGradePeriodsByGradeQueryOptions } from '@/features/academics/grade-periods/api'
import {
  Select,
  SelectContent,
  SelectItem,
  SelectTrigger,
  SelectValue,
} from '@/components/ui/select'
import { Input } from '@/components/ui/input'
import { Button } from '@/components/ui/button'
import { Checkbox } from '@/components/ui/checkbox'
import { HStack, Stack } from '@/components/primitives'
import { Field, FieldLabel, FieldError } from '@/components/ui/field'

interface TimetableAddDialogProps {
  open: boolean
  onOpenChange: (open: boolean) => void
  onConfirm: (data: TimetableEntryFormValues) => void
  isSubmitting?: boolean
  academicYears: Array<AcademicYearResponse>
  classes: Array<ClassResponse>
  staff: Array<StaffResponse>
}

export function TimetableAddDialog({
  open,
  onOpenChange,
  onConfirm,
  isSubmitting,
  academicYears,
  classes,
  staff,
}: TimetableAddDialogProps) {
  const form = useForm<TimetableEntryFormValues>({
    resolver: zodResolver(timetableEntryFormSchema),
    defaultValues: {
      class_id: '',
      subject_id: '',
      teacher_id: '',
      academic_year_id: '',
      day_of_week: '',
      start_time: '08:00:00',
      end_time: '08:40:00',
      room: '',
      period_number: 1,
      grade_period_id: undefined,
    },
  })

  const selectedClassId = useWatch({
    control: form.control,
    name: 'class_id',
  })

  const selectedClass = React.useMemo(
    () => classes.find((c) => c.id === selectedClassId),
    [classes, selectedClassId]
  )

  const { data: subjectsData } = useQuery({
    ...getAllSubjectsOptions({ client: authClient }),
    staleTime: Infinity,
  })
  const subjects = subjectsData?.data || []

  const { data: gradePeriods = [] } = useQuery({
    ...getGradePeriodsByGradeQueryOptions(selectedClass?.grade_id ?? ''),
    enabled: !!selectedClass?.grade_id,
  })

  const [usePredefined, setUsePredefined] = React.useState(false)

  React.useEffect(() => {
    if (!open) {
      form.reset()
      setUsePredefined(false)
    }
  }, [open, form])

  const handlePeriodChange = (periodId: string | null) => {
    if (periodId && Array.isArray(gradePeriods)) {
      const period = gradePeriods.find((p) => p.id === periodId)
      if (period) {
        form.setValue('period_number', period.period_number)
        form.setValue('start_time', period.start_time)
        form.setValue('end_time', period.end_time)
        form.setValue('grade_period_id', periodId)
      }
    }
  }

  return (
    <Dialog open={open} onOpenChange={onOpenChange}>
      <DialogContent className="max-w-2xl">
        <DialogHeader>
          <DialogTitle>Add New Timetable Entry</DialogTitle>
        </DialogHeader>

        <form onSubmit={form.handleSubmit(onConfirm)}>
          <Stack gap={4} className="py-4">
            <HStack gap={4}>
              <Controller
                control={form.control}
                name="class_id"
                render={({ field, fieldState }) => (
                  <Field className="flex-1">
                    <FieldLabel>Class</FieldLabel>
                    <Select
                      onValueChange={field.onChange}
                      value={field.value}
                    >
                      <SelectTrigger>
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
                    {fieldState.error && <FieldError errors={[fieldState.error]} />}
                  </Field>
                )}
              />
              <Controller
                control={form.control}
                name="subject_id"
                render={({ field, fieldState }) => (
                  <Field className="flex-1">
                    <FieldLabel>Subject</FieldLabel>
                    <Select
                      onValueChange={field.onChange}
                      value={field.value}
                    >
                      <SelectTrigger>
                        <SelectValue placeholder="Select a subject" />
                      </SelectTrigger>
                      <SelectContent>
                        {subjects.map((s) => (
                          <SelectItem key={s.id} value={s.id}>
                            {s.subject_name_en}
                          </SelectItem>
                        ))}
                      </SelectContent>
                    </Select>
                    {fieldState.error && <FieldError errors={[fieldState.error]} />}
                  </Field>
                )}
              />
            </HStack>

            <HStack gap={4}>
              <Controller
                control={form.control}
                name="teacher_id"
                render={({ field, fieldState }) => (
                  <Field className="flex-1">
                    <FieldLabel>Teacher</FieldLabel>
                    <Select
                      onValueChange={field.onChange}
                      value={field.value}
                    >
                      <SelectTrigger>
                        <SelectValue placeholder="Select a teacher" />
                      </SelectTrigger>
                      <SelectContent>
                        {staff.map((s) => (
                          <SelectItem key={s.id} value={s.id}>
                            {s.name}
                          </SelectItem>
                        ))}
                      </SelectContent>
                    </Select>
                    {fieldState.error && <FieldError errors={[fieldState.error]} />}
                  </Field>
                )}
              />
              <Controller
                control={form.control}
                name="academic_year_id"
                render={({ field, fieldState }) => (
                  <Field className="flex-1">
                    <FieldLabel>Academic Year</FieldLabel>
                    <Select
                      onValueChange={field.onChange}
                      value={field.value}
                    >
                      <SelectTrigger>
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
                    {fieldState.error && <FieldError errors={[fieldState.error]} />}
                  </Field>
                )}
              />
            </HStack>

            <HStack gap={4}>
              <Controller
                control={form.control}
                name="day_of_week"
                render={({ field, fieldState }) => (
                  <Field className="flex-1">
                    <FieldLabel>Day of Week</FieldLabel>
                    <Select
                      onValueChange={field.onChange}
                      value={field.value}
                    >
                      <SelectTrigger>
                        <SelectValue placeholder="Select a day" />
                      </SelectTrigger>
                      <SelectContent>
                        {DAYS_OF_WEEK.map((day) => (
                          <SelectItem key={day} value={day}>
                            {day}
                          </SelectItem>
                        ))}
                      </SelectContent>
                    </Select>
                    {fieldState.error && <FieldError errors={[fieldState.error]} />}
                  </Field>
                )}
              />
              <div className="flex-1 flex items-end pb-2">
                <HStack gap={2} align="center">
                  <Checkbox
                    id="use-predefined"
                    checked={usePredefined}
                    onCheckedChange={(val) => {
                      setUsePredefined(!!val)
                      if (!val) form.setValue('grade_period_id', undefined)
                    }}
                    disabled={!selectedClassId || !Array.isArray(gradePeriods) || gradePeriods.length === 0}
                  />
                  <label
                    htmlFor="use-predefined"
                    className="text-sm font-medium leading-none peer-disabled:cursor-not-allowed peer-disabled:opacity-70"
                  >
                    Use Predefined Period
                  </label>
                </HStack>
              </div>
            </HStack>

            {usePredefined && (
              <Controller
                control={form.control}
                name="grade_period_id"
                render={({ field, fieldState }) => (
                  <Field>
                    <FieldLabel>Grade Period</FieldLabel>
                    <Select
                      onValueChange={(val) => {
                        field.onChange(val)
                        handlePeriodChange(val)
                      }}
                      value={field.value || ''}
                    >
                      <SelectTrigger>
                        <SelectValue placeholder="Select a predefined period" />
                      </SelectTrigger>
                      <SelectContent>
                        {Array.isArray(gradePeriods) && gradePeriods.map((p) => (
                          <SelectItem key={p.id} value={p.id}>
                            Period {p.period_number} ({p.start_time} - {p.end_time}) {p.is_break ? '[Break]' : ''}
                          </SelectItem>
                        ))}
                      </SelectContent>
                    </Select>
                    {fieldState.error && <FieldError errors={[fieldState.error]} />}
                  </Field>
                )}
              />
            )}

            <HStack gap={4}>
              <Controller
                control={form.control}
                name="period_number"
                render={({ field, fieldState }) => (
                  <Field className="flex-1">
                    <FieldLabel>Period Number</FieldLabel>
                    <Input
                      type="number"
                      {...field}
                      disabled={usePredefined}
                      onChange={(e) => field.onChange(parseInt(e.target.value))}
                    />
                    {fieldState.error && <FieldError errors={[fieldState.error]} />}
                  </Field>
                )}
              />
              <Controller
                control={form.control}
                name="room"
                render={({ field, fieldState }) => (
                  <Field className="flex-1">
                    <FieldLabel>Room</FieldLabel>
                    <Input placeholder="Enter room name/number" {...field} />
                    {fieldState.error && <FieldError errors={[fieldState.error]} />}
                  </Field>
                )}
              />
            </HStack>

            <HStack gap={4}>
              <Controller
                control={form.control}
                name="start_time"
                render={({ field, fieldState }) => (
                  <Field className="flex-1">
                    <FieldLabel>Start Time</FieldLabel>
                    <Input type="time" {...field} disabled={usePredefined} />
                    {fieldState.error && <FieldError errors={[fieldState.error]} />}
                  </Field>
                )}
              />
              <Controller
                control={form.control}
                name="end_time"
                render={({ field, fieldState }) => (
                  <Field className="flex-1">
                    <FieldLabel>End Time</FieldLabel>
                    <Input type="time" {...field} disabled={usePredefined} />
                    {fieldState.error && <FieldError errors={[fieldState.error]} />}
                  </Field>
                )}
              />
            </HStack>
          </Stack>

          <DialogFooter>
            <Button
              type="button"
              variant="ghost"
              onClick={() => onOpenChange(false)}
            >
              Cancel
            </Button>
            <Button type="submit" disabled={isSubmitting}>
              {isSubmitting ? 'Adding...' : 'Add Entry'}
            </Button>
          </DialogFooter>
        </form>
      </DialogContent>
    </Dialog>
  )
}
