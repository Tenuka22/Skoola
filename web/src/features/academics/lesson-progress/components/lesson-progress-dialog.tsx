import * as React from 'react'
import { useQuery } from '@tanstack/react-query'
import { format } from 'date-fns'
import {
  Dialog,
  DialogContent,
  DialogHeader,
  DialogTitle,
} from '@/components/ui/dialog'
import { FormBuilder } from '@/components/form-builder'
import { lessonProgressSchema, type LessonProgressFormValues } from '../schemas'
import { getAllAcademicYearsQueryOptions } from '@/features/academics/years/api'
import { getTimetableByClassAndDayQueryOptions } from '@/features/academics/timetables/api'

interface LessonProgressDialogProps {
  open: boolean
  onOpenChange: (open: boolean) => void
  classId: string
  subjectId: string
  onConfirm: (data: LessonProgressFormValues) => void
  isSubmitting?: boolean
}

export function LessonProgressDialog({
  open,
  onOpenChange,
  classId,
  subjectId,
  onConfirm,
  isSubmitting,
}: LessonProgressDialogProps) {
  const [selectedDate, setSelectedDate] = React.useState<Date>(new Date())

  const academicYearsQuery = useQuery(getAllAcademicYearsQueryOptions())
  const currentYear = academicYearsQuery.data?.data.find((y) => y.current)

  const dayOfWeek = format(selectedDate, 'EEEE')

  const timetableQuery = useQuery({
    ...getTimetableByClassAndDayQueryOptions({
      path: {
        class_id: classId,
        day_of_week: dayOfWeek,
        academic_year_id: currentYear?.id || '',
      },
    }),
    enabled: !!classId && !!currentYear?.id && open,
  })

  // Filter timetable for the specific subject
  const relevantTimetableEntries = React.useMemo(() => {
    return (timetableQuery.data || []).filter((t) => t.subject_id === subjectId)
  }, [timetableQuery.data, subjectId])

  return (
    <Dialog open={open} onOpenChange={onOpenChange}>
      <DialogContent className="sm:max-w-[600px]">
        <DialogHeader>
          <DialogTitle>Record Lesson Progress</DialogTitle>
        </DialogHeader>
        <FormBuilder
          schema={lessonProgressSchema as any}
          defaultValues={{
            class_id: classId,
            subject_id: subjectId,
            date: format(selectedDate, 'yyyy-MM-dd'),
            is_skipped: false,
            is_substitution: false,
            priority_level: 1,
            progress_percentage: 0,
            topic_covered: '',
            sub_topic: '',
            homework_assigned: '',
            resources_used: '',
            timetable_id: relevantTimetableEntries[0]?.id || '',
          }}
          onSubmit={(values) => onConfirm(values as LessonProgressFormValues)}
          config={{
            structure: [
              [
                {
                  type: 'date-picker',
                  field: 'date',
                  label: 'Date',
                  onValueChange: (val) => {
                    if (val) setSelectedDate(new Date(val))
                  },
                },
              ],
              [
                {
                  type: 'select',
                  field: 'timetable_id',
                  label: 'Period/Time Slot',
                  items: relevantTimetableEntries.map((t) => ({
                    label: `${t.start_time} - ${t.end_time} (${t.period_number})`,
                    value: t.id,
                  })),
                  parse: (v) => v,
                },
              ],
              ['topic_covered', 'sub_topic'],
              ['progress_percentage', 'priority_level'],
              ['is_skipped', 'is_substitution'],
              ['homework_assigned'],
              ['resources_used'],
            ],
          }}
          actions={[
            {
              label: 'Cancel',
              onClick: () => onOpenChange(false),
              variant: 'outline',
            },
            {
              label: 'Record Progress',
              type: 'submit',
              variant: 'default',
              loading: isSubmitting,
            },
          ]}
          className="py-4"
        />
      </DialogContent>
    </Dialog>
  )
}
