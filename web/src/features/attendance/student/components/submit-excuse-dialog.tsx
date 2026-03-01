import { useState } from 'react'
import { useSubmitExcuse } from '../../api'
import type { AttendanceStatus } from '@/lib/api/types.gen'
import { isAttendanceStatus } from '@/features/attendance/types'
import {
  Dialog,
  DialogContent,
  DialogDescription,
  DialogFooter,
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

export function SubmitExcuseDialog({
  open,
  onOpenChange,
  attendanceRecordId,
}: {
  open: boolean
  onOpenChange: (open: boolean) => void
  attendanceRecordId: string
}) {
  const [excuseType, setExcuseType] = useState<AttendanceStatus>('Excused')
  const [remarks, setRemarks] = useState('')
  // In a real app, you'd handle file uploads here
  // const [document, setDocument] = useState<File | null>(null)

  const { mutate: submit, isPending } = useSubmitExcuse()

  const handleSubmit = () => {
    submit(
      {
        body: {
          attendance_record_id: attendanceRecordId,
          excuse_type: excuseType,
          // Removed remarks as it's not in SubmitExcuseRequest
          document_url: null, // Placeholder for file upload
        },
      },
      {
        onSuccess: () => onOpenChange(false),
      },
    )
  }

  return (
    <Dialog open={open} onOpenChange={onOpenChange}>
      <DialogContent>
        <DialogHeader>
          <DialogTitle>Submit Attendance Excuse</DialogTitle>
          <DialogDescription>
            Submit an excuse for an absence. Please provide a reason and any
            supporting documents.
          </DialogDescription>
        </DialogHeader>
        <div className="grid gap-4 py-4">
          <Select
            value={excuseType}
            onValueChange={(v) => {
              if (v && isAttendanceStatus(v)) setExcuseType(v)
            }}
          >
            <SelectTrigger>
              <SelectValue placeholder="Excuse Type" />
            </SelectTrigger>
            <SelectContent>
              <SelectItem value="Excused">Excused</SelectItem>
              <SelectItem value="SchoolBusiness">School Business</SelectItem>
            </SelectContent>
          </Select>
          <Textarea
            placeholder="Remarks..."
            value={remarks}
            onChange={(e) => setRemarks(e.target.value)}
          />
          {/* File upload input would go here */}
        </div>
        <DialogFooter>
          <Button variant="outline" onClick={() => onOpenChange(false)}>
            Cancel
          </Button>
          <Button onClick={handleSubmit} disabled={isPending}>
            {isPending ? 'Submitting...' : 'Submit'}
          </Button>
        </DialogFooter>
      </DialogContent>
    </Dialog>
  )
}
