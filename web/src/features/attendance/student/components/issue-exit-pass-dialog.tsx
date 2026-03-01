import { useState } from 'react'
import { format } from 'date-fns'
import { useIssueExitPass } from '../../api'
import type { ExitReason } from '@/lib/api/types.gen' // Changed from ExitReasonType
import { isExitReason } from '@/features/attendance/types'
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
import { Input } from '@/components/ui/input'
import { Textarea } from '@/components/ui/textarea'

export function IssueExitPassDialog({
  open,
  onOpenChange,
  studentId,
}: {
  open: boolean
  onOpenChange: (open: boolean) => void
  studentId: string
}) {
  const [reason, setReason] = useState<ExitReason>('Medical')
  const [remarks, setRemarks] = useState('')
  const [exitTime, setExitTime] = useState(format(new Date(), 'HH:mm'))

  const { mutate: issue, isPending } = useIssueExitPass()

  const handleSubmit = () => {
    issue(
      {
        body: {
          student_id: studentId,
          reason, // Changed from reason_type
          exit_time: exitTime,
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
          <DialogTitle>Issue Exit Pass</DialogTitle>
          <DialogDescription>
            Issue a digital exit pass for a student to leave the school
            premises.
          </DialogDescription>
        </DialogHeader>
        <div className="grid gap-4 py-4">
          <Input
            type="time"
            value={exitTime}
            onChange={(e) => setExitTime(e.target.value)}
          />
          <Select
            value={reason} // Changed from reasonType
            onValueChange={(v) => {
              if (v && isExitReason(v)) setReason(v)
            }}
          >
            <SelectTrigger>
              <SelectValue placeholder="Reason" />
            </SelectTrigger>
            <SelectContent>
              <SelectItem value="Medical">Medical</SelectItem>
              <SelectItem value="FamilyEvent">Family Event</SelectItem>
              <SelectItem value="Other">Other</SelectItem>
            </SelectContent>
          </Select>
          <Textarea
            placeholder="Remarks..."
            value={remarks}
            onChange={(e) => setRemarks(e.target.value)}
          />
        </div>
        <DialogFooter>
          <Button variant="outline" onClick={() => onOpenChange(false)}>
            Cancel
          </Button>
          <Button onClick={handleSubmit} disabled={isPending}>
            {isPending ? 'Issuing...' : 'Issue Pass'}
          </Button>
        </DialogFooter>
      </DialogContent>
    </Dialog>
  )
}
