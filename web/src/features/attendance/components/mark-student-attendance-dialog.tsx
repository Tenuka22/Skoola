import { useCallback } from 'react'
import { isFuture, isToday } from 'date-fns'
import { toast } from 'sonner'
import { Controller } from 'react-hook-form'
import {
  useMarkStudentAttendanceBulk,
  useUpdateStudentAttendance,
} from '../api'
import { ALL_ATTENDANCE_STATUSES } from '../types'
import type {
  ControllerFieldState,
  ControllerRenderProps,
  Path,
  UseFormReturn,
} from 'react-hook-form'
import type { StudentAttendanceWithMember } from '../types'
import type { z } from 'zod'
import { zMarkStudentAttendanceRequest } from '@/lib/api/zod.gen'
import {
  Dialog,
  DialogContent,
  DialogDescription,
  DialogHeader,
  DialogTitle,
} from '@/components/ui/dialog'
import { Button } from '@/components/ui/button'
import {
  Select,
  SelectContent,
  SelectItem,
  SelectTrigger,
  SelectValue,
} from '@/components/ui/select'
import { Textarea } from '@/components/ui/textarea'
import { useAuth } from '@/hooks/use-auth'
import { FormBuilder, defineFormConfig } from '@/components/form-builder'
import { Field, FieldError, FieldLabel } from '@/components/ui/field'

const attendanceSchema = zMarkStudentAttendanceRequest.pick({
  status: true,
  remarks: true,
})

type AttendanceFormValues = z.infer<typeof attendanceSchema>

interface MarkStudentAttendanceDialogProps {
  attendance: StudentAttendanceWithMember | null
  open: boolean
  onOpenChange: (open: boolean) => void
  date: string
  classId: string
}

export const MarkStudentAttendanceDialog = ({
  attendance,
  open,
  onOpenChange,
  date,
  classId,
}: MarkStudentAttendanceDialogProps) => {
  const { user } = useAuth()
  const markBulkMutation = useMarkStudentAttendanceBulk()
  const updateMutation = useUpdateStudentAttendance()

  const preload = useCallback(
    (
      form: UseFormReturn<AttendanceFormValues, unknown, AttendanceFormValues>,
    ) => {
      if (attendance) {
        form.reset({
          status: attendance.status,
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
    if (!attendance || !user) return

    const selectedDate = new Date(date)

    if (isFuture(selectedDate) && !isToday(selectedDate)) {
      toast.error('Attendance cannot be marked for a future date.')
      return
    }

    if (attendance.id && !attendance.id.startsWith('temp-')) {
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
            attendance_records: [
              {
                student_id: attendance.student_id,
                class_id: classId,
                date,
                status: values.status,
                remarks: values.remarks,
                marked_by: user.id,
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
            Mark daily attendance for {attendance?.student?.name_english}.
          </DialogDescription>
        </DialogHeader>
        <FormBuilder
          schema={attendanceSchema}
          config={config}
          defaultValues={{
            status: 'Present',
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
