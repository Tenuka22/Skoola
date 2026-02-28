import { HugeiconsIcon } from '@hugeicons/react'
import {
  AlertCircleIcon,
  CalendarCheckIn01Icon,
  FloppyDiskIcon,
} from '@hugeicons/core-free-icons'
import { useMutation, useQuery, useQueryClient } from '@tanstack/react-query'
import { format } from 'date-fns'
import * as React from 'react'
import { toast } from 'sonner'
import type {
  AttendanceStatus,
  StudentAttendanceResponse,
  StudentResponse,
} from '@/lib/api/types.gen'
import { Button } from '@/components/ui/button'
import {
  Dialog,
  DialogContent,
  DialogDescription,
  DialogHeader,
  DialogTitle,
} from '@/components/ui/dialog'
import { Spinner } from '@/components/ui/spinner'
import { authClient } from '@/lib/clients'
import {
  getAttendanceByStudentOptions,
  markIndividualStudentAttendanceMutation,
} from '@/lib/api/@tanstack/react-query.gen'
import { Badge } from '@/components/ui/badge'
import {
  Select,
  SelectContent,
  SelectItem,
  SelectTrigger,
  SelectValue,
} from '@/components/ui/select'
import { Input } from '@/components/ui/input'
import { Label } from '@/components/ui/label'
import {
  ALL_ATTENDANCE_STATUSES,
  isAttendanceStatus,
} from '@/features/attendance/types'

interface StudentAttendanceDialogProps {
  student: StudentResponse | null
  open: boolean
  onOpenChange: (open: boolean) => void
}

export function StudentAttendanceDialog({
  student,
  open,
  onOpenChange,
}: StudentAttendanceDialogProps) {
  const queryClient = useQueryClient()
  const [date, setDate] = React.useState(format(new Date(), 'yyyy-MM-dd'))
  const [status, setStatus] = React.useState<AttendanceStatus>('Present')
  const [remarks, setRemarks] = React.useState('')

  const {
    data: attendanceData,
    isLoading,
    isError,
    error,
  } = useQuery({
    ...getAttendanceByStudentOptions({
      client: authClient,
      path: { student_id: student?.id ?? '' },
    }),
    enabled: !!student,
  })

  const markAttendance = useMutation({
    ...markIndividualStudentAttendanceMutation({ client: authClient }),
    onSuccess: () => {
      toast.success('Attendance marked successfully.')
      queryClient.invalidateQueries({
        queryKey: ['getAttendanceByStudent', { student_id: student?.id }],
      })
    },
    onError: (error) => {
      toast.error(
        `Failed to mark attendance: ${error.message || 'Unknown error'}`,
      )
    },
  })

  const handleSubmit = (e: React.FormEvent) => {
    e.preventDefault()
    if (student) {
      markAttendance.mutate({
        body: {
          student_id: student.id,
          date,
          status,
          remarks: remarks || undefined,
          marked_by: 'Admin', // Assuming admin for now
          class_id: '', // Should be fetched from student current class
        },
      })
    }
  }

  const attendanceRecords = attendanceData || []

  return (
    <Dialog open={open} onOpenChange={onOpenChange}>
      <DialogContent className="max-w-2xl flex flex-col h-[85vh]">
        <DialogHeader>
          <DialogTitle>
            Attendance Management: {student?.name_english}
          </DialogTitle>
          <DialogDescription>
            Record student attendance and view history.
          </DialogDescription>
        </DialogHeader>

        <div className="flex flex-col gap-6 flex-1 overflow-hidden">
          {/* Mark Attendance Form */}
          <form
            onSubmit={handleSubmit}
            className="grid grid-cols-2 gap-4 p-4 border rounded-xl bg-muted/30"
          >
            <div className="space-y-2">
              <Label htmlFor="date">Date</Label>
              <Input
                id="date"
                type="date"
                value={date}
                onChange={(e) => setDate(e.target.value)}
              />
            </div>
            <div className="space-y-2">
              <Label htmlFor="status">Status</Label>
              <Select
                value={status}
                onValueChange={(val) => {
                  if (val && isAttendanceStatus(val)) {
                    setStatus(val)
                  }
                }}
              >
                <SelectTrigger id="status">
                  <SelectValue />
                </SelectTrigger>
                <SelectContent>
                  {ALL_ATTENDANCE_STATUSES.map((s) => (
                    <SelectItem key={s} value={s}>
                      {s}
                    </SelectItem>
                  ))}
                </SelectContent>
              </Select>
            </div>
            <div className="col-span-2 space-y-2">
              <Label htmlFor="remarks">Remarks</Label>
              <Input
                id="remarks"
                placeholder="Optional remarks"
                value={remarks}
                onChange={(e) => setRemarks(e.target.value)}
              />
            </div>
            <div className="col-span-2">
              <Button
                type="submit"
                className="w-full"
                disabled={markAttendance.isPending}
              >
                {markAttendance.isPending ? (
                  <Spinner className="mr-2" />
                ) : (
                  <HugeiconsIcon
                    icon={FloppyDiskIcon}
                    className="size-4 mr-2"
                  />
                )}
                Mark Attendance
              </Button>
            </div>
          </form>

          {/* Attendance History */}
          <div className="flex-1 flex flex-col min-h-0">
            <h3 className="text-sm font-semibold mb-2 flex items-center gap-2">
              <HugeiconsIcon icon={CalendarCheckIn01Icon} className="size-4" />
              Attendance History
            </h3>
            {isLoading ? (
              <div className="grid flex-1 place-items-center">
                <Spinner />
              </div>
            ) : isError ? (
              <div className="grid flex-1 place-items-center text-center">
                <HugeiconsIcon
                  icon={AlertCircleIcon}
                  className="size-8 text-destructive opacity-50"
                />
                <p className="text-sm text-muted-foreground mt-2">
                  Error:{' '}
                  {error instanceof Error ? error.message : 'Unknown error'}
                </p>
              </div>
            ) : attendanceRecords.length === 0 ? (
              <p className="text-xs text-muted-foreground italic text-center p-8">
                No attendance history found.
              </p>
            ) : (
              <div className="divide-y">
                {attendanceRecords.map(
                  (record: StudentAttendanceResponse, index: number) => (
                    <div
                      key={index}
                      className="p-3 flex items-center justify-between text-sm"
                    >
                      <div className="flex flex-col">
                        <span className="font-medium">
                          {format(new Date(record.date), 'PPP')}
                        </span>
                        <span className="text-xs text-muted-foreground">
                          {record.remarks || 'No remarks'}
                        </span>
                      </div>
                      <Badge
                        variant="secondary"
                        className={
                          record.status === 'Present'
                            ? 'bg-green-500/10 text-green-600 border-green-500/20'
                            : record.status === 'Absent'
                              ? 'bg-red-500/10 text-red-600 border-red-500/20'
                              : 'bg-blue-500/10 text-blue-600 border-blue-500/20'
                        }
                      >
                        {record.status}
                      </Badge>
                    </div>
                  ),
                )}
              </div>
            )}
          </div>
        </div>
      </DialogContent>
    </Dialog>
  )
}
