import { HugeiconsIcon } from '@hugeicons/react'
import { AlertCircleIcon } from '@hugeicons/core-free-icons'
import { useQuery } from '@tanstack/react-query'
import type { StudentMarkResponse, StudentResponse } from '@/lib/api/types.gen'
import { Button } from '@/components/ui/button'
import {
  Dialog,
  DialogContent,
  DialogDescription,
  DialogHeader,
  DialogTitle,
} from '@/components/ui/dialog'
import { ScrollArea } from '@/components/ui/scroll-area'
import { Spinner } from '@/components/ui/spinner'
import { authClient } from '@/lib/clients'
import { getStudentMarksByStudentIdOptions } from '@/lib/api/@tanstack/react-query.gen'
import { Badge } from '@/components/ui/badge'

interface StudentMarksDialogProps {
  student: StudentResponse | null
  open: boolean
  onOpenChange: (open: boolean) => void
}

export function StudentMarksDialog({
  student,
  open,
  onOpenChange,
}: StudentMarksDialogProps) {
  const {
    data: marksData,
    isLoading,
    isError,
    error,
  } = useQuery({
    ...getStudentMarksByStudentIdOptions({
      client: authClient,
      path: { student_id: student?.id ?? '' },
    }),
    enabled: !!student,
  })

  const marks = marksData || []

  return (
    <Dialog open={open} onOpenChange={onOpenChange}>
      <DialogContent className="max-w-3xl flex flex-col h-[80vh]">
        <DialogHeader>
          <DialogTitle>Academic Marks: {student?.name_english}</DialogTitle>
          <DialogDescription>
            View student performance across different exams and subjects.
          </DialogDescription>
        </DialogHeader>

        <div className="flex flex-col gap-4 flex-1 overflow-hidden">
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
                Error: {error?.message}
              </p>
            </div>
          ) : marks.length === 0 ? (
            <p className="text-xs text-muted-foreground italic text-center p-8">
              No academic marks found for this student.
            </p>
          ) : (
            <table className="w-full text-sm">
              <thead className="bg-muted/50 sticky top-0">
                <tr>
                  <th className="p-3 text-left">Exam</th>
                  <th className="p-3 text-left">Subject</th>
                  <th className="p-3 text-right">Marks</th>
                  <th className="p-3 text-center">Status</th>
                </tr>
              </thead>
              <tbody className="divide-y">
                {marks.map((mark: StudentMarkResponse, index: number) => (
                  <tr key={index}>
                    <td className="p-3">{mark.exam_id}</td>
                    <td className="p-3">{mark.subject_id}</td>
                    <td className="p-3 text-right font-medium">
                      {mark.marks_obtained}
                    </td>
                    <td className="p-3 text-center">
                      <Badge
                        variant={mark.is_absent ? 'destructive' : 'secondary'}
                      >
                        {mark.is_absent ? 'Absent' : 'Present'}
                      </Badge>
                    </td>
                  </tr>
                ))}
              </tbody>
            </table>
          )}
        </div>
        <Button onClick={() => onOpenChange(false)} className="mt-4">
          Close
        </Button>
      </DialogContent>
    </Dialog>
  )
}
