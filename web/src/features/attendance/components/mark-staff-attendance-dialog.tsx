import { Controller } from 'react-hook-form'
import { useCallback } from 'react'
import { toast } from 'sonner'
import { isFuture, isToday } from 'date-fns'
import { useMarkStaffAttendanceBulk, useUpdateStaffAttendance } from '../api'
import { ALL_ATTENDANCE_STATUSES } from '../types'
import type {
  ControllerFieldState,
  ControllerRenderProps,
  Path,
  UseFormReturn,
} from 'react-hook-form'
import type { z } from 'zod'
import type { StaffAttendanceWithMember } from '../types'
import { zMarkStaffAttendanceRequest } from '@/lib/api/zod.gen'
import {
  Dialog,
  DialogContent,
  DialogDescription,
  DialogHeader,
  DialogTitle,
} from '@/components/ui/dialog'
import { Button } from '@/components/ui/button'
import { Field, FieldError, FieldLabel } from '@/components/ui/field'
import {
  Select,
  SelectContent,
  SelectItem,
  SelectTrigger,
  SelectValue,
} from '@/components/ui/select'
import { Input } from '@/components/ui/input'
import { Textarea } from '@/components/ui/textarea'
import { FormBuilder, defineFormConfig } from '@/components/form-builder'

const attendanceSchema = zMarkStaffAttendanceRequest.omit({ date: true })

type AttendanceFormValues = z.infer<typeof attendanceSchema>

interface MarkStaffAttendanceDialogProps {
  attendance: StaffAttendanceWithMember | null
  open: boolean
  onOpenChange: (open: boolean) => void
  date: string
}

export const MarkStaffAttendanceDialog = ({
  attendance,
  open,
  onOpenChange,
  date,
}: MarkStaffAttendanceDialogProps) => {
  const markBulkMutation = useMarkStaffAttendanceBulk()
  const updateMutation = useUpdateStaffAttendance()

  const preload = useCallback(
    (
      form: UseFormReturn<AttendanceFormValues, unknown, AttendanceFormValues>,
    ) => {
      if (attendance) {
        form.reset({
          status: attendance.status,
          time_in: attendance.time_in ?? '',
          time_out: attendance.time_out ?? '',
          remarks: attendance.remarks ?? '',
        })
      } else if (!open) {
        form.reset()
      }
    },
    [attendance, open],
  )

  const config = defineFormConfig(attendanceSchema, {
    structure: [],
    extras: {
      top: (form) => (
        <>
          <Controller
            name="status"
            control={form.control}
            render={({
              field,
              fieldState,
            }: {
              field: ControllerRenderProps<
                AttendanceFormValues,
                Path<AttendanceFormValues>
              >
              fieldState: ControllerFieldState
            }) => (
              <Field data-invalid={fieldState.invalid}>
                <FieldLabel
                  htmlFor="status"
                  className="font-bold uppercase text-[10px] tracking-widest text-muted-foreground"
                >
                  Status
                </FieldLabel>
                <Select
                  {...field}
                  value={typeof field.value === 'string' ? field.value : ''}
                  onValueChange={field.onChange}
                >
                  <SelectTrigger
                    id="status"
                    className="rounded-xl border-2 h-10 font-bold"
                    aria-invalid={fieldState.invalid}
                  >
                    <SelectValue placeholder="Select status" />
                  </SelectTrigger>
                  <SelectContent>
                    {ALL_ATTENDANCE_STATUSES.map((status) => (
                      <SelectItem key={status} value={status}>
                        {status === 'HalfDay' ? 'Half Day' : status}
                      </SelectItem>
                    ))}
                  </SelectContent>
                </Select>
                {fieldState.invalid && (
                  <FieldError errors={[fieldState.error]} />
                )}
              </Field>
            )}
          />
          <div className="grid grid-cols-2 gap-4">
            <Controller
              name="time_in"
              control={form.control}
              render={({
                field,
                fieldState,
              }: {
                field: ControllerRenderProps<
                  AttendanceFormValues,
                  Path<AttendanceFormValues>
                >
                fieldState: ControllerFieldState
              }) => (
                <Field data-invalid={fieldState.invalid}>
                  <FieldLabel
                    htmlFor="time_in"
                    className="font-bold uppercase text-[10px] tracking-widest text-muted-foreground"
                  >
                    Time In
                  </FieldLabel>
                  <Input
                    {...field}
                    id="time_in"
                    type="time"
                    className="rounded-xl border-2 h-10 font-bold"
                    aria-invalid={fieldState.invalid}
                    value={field.value ?? ''}
                  />
                  {fieldState.invalid && (
                    <FieldError errors={[fieldState.error]} />
                  )}
                </Field>
              )}
            />
            <Controller
              name="time_out"
              control={form.control}
              render={({
                field,
                fieldState,
              }: {
                field: ControllerRenderProps<
                  AttendanceFormValues,
                  Path<AttendanceFormValues>
                >
                fieldState: ControllerFieldState
              }) => (
                <Field data-invalid={fieldState.invalid}>
                  <FieldLabel
                    htmlFor="time_out"
                    className="font-bold uppercase text-[10px] tracking-widest text-muted-foreground"
                  >
                    Time Out
                  </FieldLabel>
                  <Input
                    {...field}
                    id="time_out"
                    type="time"
                    className="rounded-xl border-2 h-10 font-bold"
                    aria-invalid={fieldState.invalid}
                    value={field.value ?? ''}
                  />
                  {fieldState.invalid && (
                    <FieldError errors={[fieldState.error]} />
                  )}
                </Field>
              )}
            />
          </div>
          <Controller
            name="remarks"
            control={form.control}
            render={({
              field,
              fieldState,
            }: {
              field: ControllerRenderProps<
                AttendanceFormValues,
                Path<AttendanceFormValues>
              >
              fieldState: ControllerFieldState
            }) => (
              <Field data-invalid={fieldState.invalid}>
                <FieldLabel
                  htmlFor="remarks"
                  className="font-bold uppercase text-[10px] tracking-widest text-muted-foreground"
                >
                  Remarks
                </FieldLabel>
                <Textarea
                  {...field}
                  id="remarks"
                  className="rounded-xl border-2 font-bold min-h-[100px]"
                  placeholder="Add any notes here..."
                  aria-invalid={fieldState.invalid}
                  value={field.value ?? ''}
                />
                {fieldState.invalid && (
                  <FieldError errors={[fieldState.error]} />
                )}
              </Field>
            )}
          />
        </>
      ),
      bottom: (
        <div className="flex justify-end pt-4">
          <Button
            type="submit"
            className="rounded-xl px-8 font-bold h-10"
            disabled={markBulkMutation.isPending || updateMutation.isPending}
          >
            Save Attendance
          </Button>
        </div>
      ),
    },
  })

  const onSubmit = (values: AttendanceFormValues) => {
    if (!attendance) return

    const selectedDate = new Date(date)

    if (isFuture(selectedDate) && !isToday(selectedDate)) {
      toast.error('Attendance cannot be marked for a future date.')
      return
    }

    if (attendance.created_at) {
      // Update existing record
      updateMutation.mutate(
        {
          path: { attendance_id: attendance.id },
          body: values,
        },
        {
          onSuccess: () => onOpenChange(false),
        },
      )
    } else {
      // Create new record
      markBulkMutation.mutate(
        {
          body: {
            date,
            attendance_records: [
              {
                staff_id: attendance.staff_id,
                status: values.status,
                time_in: values.time_in,
                time_out: values.time_out,
                remarks: values.remarks,
              },
            ],
          },
        },
        {
          onSuccess: () => onOpenChange(false),
        },
      )
    }
  }

  return (
    <Dialog open={open} onOpenChange={onOpenChange}>
      <DialogContent className="sm:max-w-[425px]">
        <DialogHeader>
          <DialogTitle className="text-xl font-black">
            Mark Attendance
          </DialogTitle>
          <DialogDescription className="font-medium text-muted-foreground">
            Mark daily attendance for {attendance?.staff?.name}.
          </DialogDescription>
        </DialogHeader>
        <FormBuilder
          schema={attendanceSchema}
          config={config}
          defaultValues={{
            status: 'Present',
            time_in: '',
            time_out: '',
            remarks: '',
          }}
          onSubmit={(values) => onSubmit(values)}
          preload={preload}
          isLoading={markBulkMutation.isPending || updateMutation.isPending}
          showErrorSummary={false}
          toastErrors={false}
          showSuccessAlert={false}
          actions={[]}
          className="space-y-4 pt-4"
        />
      </DialogContent>
    </Dialog>
  )
}
