import { useQuery } from '@tanstack/react-query'
import { examSchema } from '../schemas'
import type { ExamFormValues } from '../schemas'
import {
  Dialog,
  DialogContent,
  DialogHeader,
  DialogTitle,
} from '@/components/ui/dialog'
import { authClient } from '@/lib/clients'
import {
  getAllAcademicYearsOptions,
  getAllExamTypesOptions,
} from '@/lib/api/@tanstack/react-query.gen'
import { FormBuilder } from '@/components/form-builder'

interface ExamAddDialogProps {
  open: boolean
  onOpenChange: (open: boolean) => void
  onConfirm: (data: ExamFormValues) => void
  isSubmitting?: boolean
}

export function ExamAddDialog({
  open,
  onOpenChange,
  onConfirm,
  isSubmitting,
}: ExamAddDialogProps) {
  const { data: academicYearsData } = useQuery(
    getAllAcademicYearsOptions({ client: authClient }),
  )
  const academicYears = academicYearsData?.data || []

  const { data: examTypesData } = useQuery(
    getAllExamTypesOptions({ client: authClient }),
  )
  const examTypes = examTypesData?.data || []

  return (
    <Dialog open={open} onOpenChange={onOpenChange}>
      <DialogContent className="max-w-xl">
        <DialogHeader>
          <DialogTitle>Add New Exam</DialogTitle>
        </DialogHeader>
        <FormBuilder
          schema={examSchema}
          defaultValues={{
            name: '',
            academic_year_id: '',
            exam_type_id: '',
            start_date: '',
            end_date: '',
            term_id: '',
          }}
          onSubmit={onConfirm}
          config={{
            structure: [
              [
                {
                  field: 'name',
                  type: 'input',
                  label: 'Exam Name',
                  placeholder: 'e.g. Mid-Term Exam',
                },
              ],
              [
                {
                  field: 'academic_year_id',
                  type: 'select',
                  label: 'Academic Year',
                  placeholder: 'Select Year',
                  items: academicYears.map((ay) => ({
                    label: ay.name,
                    value: ay.id,
                  })),
                  parse: (v) => v,
                },
                {
                  field: 'exam_type_id',
                  type: 'select',
                  label: 'Exam Type',
                  placeholder: 'Select Type',
                  items: examTypes.map((et) => ({
                    label: et.name,
                    value: et.id,
                  })),
                  parse: (v) => v,
                },
              ],
              [
                {
                  field: 'start_date',
                  type: 'date-picker',
                  label: 'Start Date',
                },
                {
                  field: 'end_date',
                  type: 'date-picker',
                  label: 'End Date',
                },
              ],
            ],
          }}
          actions={[
            {
              label: 'Cancel',
              onClick: () => onOpenChange(false),
              variant: 'outline',
            },
            {
              label: 'Add Exam',
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
