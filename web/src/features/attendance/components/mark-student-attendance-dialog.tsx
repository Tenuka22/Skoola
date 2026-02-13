import * as React from 'react'
import { useForm } from 'react-hook-form'
import { zodResolver } from '@hookform/resolvers/zod'
import * as z from 'zod'
import {
  useMarkStudentAttendanceBulk,
  useUpdateStudentAttendance,
} from '../api'
import type { StudentAttendanceWithMember } from '../types'
import { zAttendanceStatus } from '@/lib/api/zod.gen'
import {
  Dialog,
  DialogContent,
  DialogDescription,
  DialogHeader,
  DialogTitle,
} from '@/components/ui/dialog'
import { Button } from '@/components/ui/button'
import {
  Form,
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
import { Textarea } from '@/components/ui/textarea'
import { useAuth } from '@/hooks/use-auth'

const attendanceSchema = z.object({
  status: zAttendanceStatus,
  remarks: z.string().optional().nullable(),
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

  const form = useForm<AttendanceFormValues>({
    resolver: zodResolver(attendanceSchema),
    defaultValues: {
      status: 'Present',
      remarks: '',
    },
  })

  React.useEffect(() => {
    if (attendance) {
      form.reset({
        status: attendance.status,
        remarks: attendance.remarks ?? '',
      })
    }
  }, [attendance, form])

  const onSubmit = (values: AttendanceFormValues) => {
    if (!attendance || !user) return

    if (attendance.id && !attendance.id.startsWith('temp-')) {
      // Update existing record
      updateMutation.mutate(
        {
          path: { attendance_id: attendance.id },
          body: values as any,
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
        <Form {...form}>
          <form
            onSubmit={form.handleSubmit(onSubmit)}
            className="space-y-4 pt-4"
          >
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
                      <SelectItem value="Present">Present</SelectItem>
                      <SelectItem value="Absent">Absent</SelectItem>
                      <SelectItem value="Late">Late</SelectItem>
                      <SelectItem value="Excused">Excused</SelectItem>
                      <SelectItem value="HalfDay">Half Day</SelectItem>
                    </SelectContent>
                  </Select>
                  <FormMessage />
                </FormItem>
              )}
            />
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
            <div className="flex justify-end pt-4">
              <Button
                type="submit"
                className="rounded-xl px-8 font-bold h-10"
                disabled={
                  markBulkMutation.isPending || updateMutation.isPending
                }
              >
                Save Attendance
              </Button>
            </div>
          </form>
        </Form>
      </DialogContent>
    </Dialog>
  )
}
