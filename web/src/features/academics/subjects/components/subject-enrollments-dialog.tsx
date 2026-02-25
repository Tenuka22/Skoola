import { HugeiconsIcon } from '@hugeicons/react'
import { AlertCircleIcon, SchoolIcon, UserIcon } from '@hugeicons/core-free-icons'
import { useQuery } from '@tanstack/react-query'
import * as React from 'react'
import type { SubjectResponse } from '@/lib/api/types.gen'
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
import {
  getAllAcademicYearsOptions,
} from '@/lib/api/@tanstack/react-query.gen'
import { Badge } from '@/components/ui/badge'
import {
  Select,
  SelectContent,
  SelectItem,
  SelectTrigger,
  SelectValue,
} from '@/components/ui/select'
import { Label } from '@/components/ui/label'

interface SubjectEnrollmentsDialogProps {
  subject: SubjectResponse | null
  open: boolean
  onOpenChange: (open: boolean) => void
}

export function SubjectEnrollmentsDialog({
  subject,
  open,
  onOpenChange,
}: SubjectEnrollmentsDialogProps) {
  const [selectedAcademicYearId, setSelectedAcademicYearId] = React.useState<string | undefined>(undefined)

  const { data: academicYearsData } = useQuery(
    getAllAcademicYearsOptions({ client: authClient }),
  )
  const academicYears = React.useMemo(() => academicYearsData?.data || [], [academicYearsData?.data])

  React.useEffect(() => {
    if (academicYears.length > 0 && !selectedAcademicYearId) {
      const currentYear = academicYears.find((ay) => ay.current)
      setSelectedAcademicYearId(currentYear?.id || academicYears[0]?.id)
    }
  }, [academicYears, selectedAcademicYearId])

  // TODO: Implement getStudentsBySubject endpoint in backend.
  // Currently getStudentEnrollments is for a specific student.
  const enrollments: Array<unknown> = []
  const { isLoading, isError, error } = {
    isLoading: false,
    isError: false,
    error: null as Error | null,
  }

  return (
    <Dialog open={open} onOpenChange={onOpenChange}>
      <DialogContent className="max-w-xl flex flex-col h-[70vh]">
        <DialogHeader>
          <DialogTitle>Enrollments for {subject?.subject_name_en}</DialogTitle>
          <DialogDescription>
            View students enrolled in this subject.
          </DialogDescription>
        </DialogHeader>

        <div className="flex flex-col gap-4 flex-1 overflow-hidden">
          <div className="flex items-center gap-2">
            <Label htmlFor="academic_year_select" className="text-right whitespace-nowrap">
              Academic Year:
            </Label>
            <Select
              onValueChange={(value) => setSelectedAcademicYearId(value || undefined)}
              value={selectedAcademicYearId}
            >
              <SelectTrigger id="academic_year_select">
                <SelectValue placeholder="Select Academic Year" />
              </SelectTrigger>
              <SelectContent>
                {academicYears.map((ay) => (
                  <SelectItem key={ay.id} value={ay.id}>
                    {ay.name}
                  </SelectItem>
                ))}
              </SelectContent>
            </Select>
          </div>

          {isLoading ? (
            <div className="grid flex-1 place-items-center">
              <Spinner />
            </div>
          ) : isError ? (
            <div className="grid flex-1 place-items-center px-4 py-8 text-center">
              <HugeiconsIcon
                icon={AlertCircleIcon}
                className="size-12 text-destructive"
              />
              <p className="text-sm text-muted-foreground mt-2">
                Error loading enrollments: {error?.message}
              </p>
            </div>
          ) : enrollments.length === 0 ? (
            <div className="grid flex-1 place-items-center px-4 py-8 text-center">
              <HugeiconsIcon
                icon={SchoolIcon}
                className="size-12 text-muted-foreground opacity-20"
              />
              <p className="text-sm text-muted-foreground mt-2">
                No students enrolled in this subject for the selected academic year.
              </p>
            </div>
          ) : (
            <ScrollArea className="flex-1 pr-4">
              <div className="grid grid-cols-1 gap-3">
                {enrollments.map((enrollment: any) => (
                  <div
                    key={enrollment.student_id}
                    className="flex items-center gap-3 p-3 border rounded-lg shadow-sm"
                  >
                    <div className="size-8 rounded-full bg-muted flex items-center justify-center">
                      <HugeiconsIcon
                        icon={UserIcon}
                        className="size-4 text-muted-foreground"
                      />
                    </div>
                    <div className="flex-1">
                      <p className="font-medium text-sm">
                        {enrollment.student_name}
                      </p>
                      <p className="text-xs text-muted-foreground">
                        ID: {enrollment.student_id}
                      </p>
                    </div>
                    <Badge variant="secondary">Enrolled</Badge>
                  </div>
                ))}
              </div>
            </ScrollArea>
          )}
        </div>
        <Button onClick={() => onOpenChange(false)} className="mt-4">
          Close
        </Button>
      </DialogContent>
    </Dialog>
  )
}
