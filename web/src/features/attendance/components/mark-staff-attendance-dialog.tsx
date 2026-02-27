import * as React from 'react'
import { useMarkStaffAttendanceBulk, useUpdateStaffAttendance } from '../api'
import { ALL_ATTENDANCE_STATUSES } from '../types'
import type { z } from 'zod'
import type { StaffAttendanceWithMember } from '../types'
import type { UseFormReturn } from 'react-hook-form'
import { zMarkStaffAttendanceRequest } from '@/lib/api/zod.gen'
import {
  Dialog,
  DialogContent,
  DialogDescription,
  DialogHeader,
  DialogTitle,
} from '@/components/ui/dialog'
import { Button } from '@/components/ui/button'
import {
  FormControl,
  FormField,
  FormItem,
  FormLabel,
  FormMessage,
} from '@/components/ui/form'
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

  const preload = React.useCallback(
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
          <FormField
            control={form.control}
            name="status"
            render={({ field }) => (
              <FormItem>
                <FormLabel className="font-bold uppercase text-[10px] tracking-widest text-muted-foreground">
                  Status
                </FormLabel>
                <Select
                  onValueChange={field.onChange}
                  defaultValue={field.value}
                >
                  <FormControl>
                    <SelectTrigger className="rounded-xl border-2 h-10 font-bold">
                      <SelectValue placeholder="Select status" />
                    </SelectTrigger>
                  </FormControl>
                  <SelectContent>
                    {ALL_ATTENDANCE_STATUSES.map((status) => (
                      <SelectItem key={status} value={status}>
                        {status === 'HalfDay' ? 'Half Day' : status}
                      </SelectItem>
                    ))}
                  </SelectContent>
                </Select>
                <FormMessage />
              </FormItem>
            )}
          />
          <div className="grid grid-cols-2 gap-4">
            <FormField
              control={form.control}
              name="time_in"
              render={({ field }) => (
                <FormItem>
                  <FormLabel className="font-bold uppercase text-[10px] tracking-widest text-muted-foreground">
                    Time In
                  </FormLabel>
                  <FormControl>
                    <Input
                      type="time"
                      {...field}
                      value={field.value ?? ''}
                      className="rounded-xl border-2 h-10 font-bold"
                    />
                  </FormControl>
                  <FormMessage />
                </FormItem>
              )}
            />
            <FormField
              control={form.control}
              name="time_out"
              render={({ field }) => (
                <FormItem>
                  <FormLabel className="font-bold uppercase text-[10px] tracking-widest text-muted-foreground">
                    Time Out
                  </FormLabel>
                  <FormControl>
                    <Input
                      type="time"
                      {...field}
                      value={field.value ?? ''}
                      className="rounded-xl border-2 h-10 font-bold"
                    />
                  </FormControl>
                  <FormMessage />
                </FormItem>
              )}
            />
          </div>
          <FormField
            control={form.control}
            name="remarks"
            render={({ field }) => (
              <FormItem>
                <FormLabel className="font-bold uppercase text-[10px] tracking-widest text-muted-foreground">
                  Remarks
                </FormLabel>
                <FormControl>
                  <Textarea
                    {...field}
                    value={field.value ?? ''}
                    className="rounded-xl border-2 font-bold min-h-[100px]"
                    placeholder="Add any notes here..."
                  />
                </FormControl>
                <FormMessage />
              </FormItem>
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
