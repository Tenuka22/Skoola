import * as React from 'react'
import { useQuery } from '@tanstack/react-query'
import { updateExamSchema } from '../schemas'
import type { UpdateExamFormValues } from '../schemas'
import type { UseFormReturn } from 'react-hook-form'
import type { ExamResponse } from '@/lib/api/types.gen'
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

interface ExamEditDialogProps {
  open: boolean
  onOpenChange: (open: boolean) => void
  onConfirm: (data: UpdateExamFormValues) => void
  isSubmitting?: boolean
  exam: ExamResponse | null
}

export function ExamEditDialog({
  open,
  onOpenChange,
  onConfirm,
  isSubmitting,
  exam,
}: ExamEditDialogProps) {
  const { data: academicYearsData } = useQuery(
    getAllAcademicYearsOptions({ client: authClient }),
  )
  const academicYears = academicYearsData?.data || []

  const { data: examTypesData } = useQuery(
    getAllExamTypesOptions({ client: authClient }),
  )
  const examTypes = examTypesData?.data || []

  const preload = React.useCallback(
    (form: UseFormReturn<UpdateExamFormValues, unknown, UpdateExamFormValues>) => {
      if (exam && open) {
        form.reset({
          name: exam.name,
          academic_year_id: exam.academic_year_id,
          exam_type_id: exam.exam_type_id,
          start_date: exam.start_date,
          end_date: exam.end_date,
          term_id: exam.term_id,
        })
      }
    },
    [exam, open],
  )

  return (
    <Dialog open={open} onOpenChange={onOpenChange}>
      <DialogContent className="max-w-xl">
        <DialogHeader>
          <DialogTitle>Edit Exam</DialogTitle>
        </DialogHeader>
        <FormBuilder
          schema={updateExamSchema}
          defaultValues={{
            name: '',
            academic_year_id: '',
            exam_type_id: '',
            start_date: '',
            end_date: '',
            term_id: '',
          }}
          preload={preload}
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
              label: 'Update Exam',
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
