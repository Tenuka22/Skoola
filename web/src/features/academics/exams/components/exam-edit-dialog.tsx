import { HugeiconsIcon } from '@hugeicons/react'
import { Tick01Icon } from '@hugeicons/core-free-icons'
import * as React from 'react'
import { useQuery } from '@tanstack/react-query'
import { updateExamSchema } from '../schemas'
import type { UpdateExamFormValues } from '../schemas'
import type { UseFormReturn } from 'react-hook-form'
import type { ExamResponse } from '@/lib/api/types.gen'
import { Button } from '@/components/ui/button'
import {
  Dialog,
  DialogContent,
  DialogFooter,
  DialogHeader,
  DialogTitle,
} from '@/components/ui/dialog'
import { Spinner } from '@/components/ui/spinner'
import { authClient } from '@/lib/clients'
import {
  getAllAcademicYearsOptions,
  getAllExamTypesOptions,
} from '@/lib/api/@tanstack/react-query.gen'
import { FormBuilder, defineFormConfig } from '@/components/form-builder'

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

  const config = defineFormConfig(updateExamSchema, {
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
          placeholder: 'Select academic year',
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
          placeholder: 'Select exam type',
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
          type: 'input',
          label: 'Start Date',
          inputType: 'date',
        },
        {
          field: 'end_date',
          type: 'input',
          label: 'End Date',
          inputType: 'date',
        },
      ],
    ],
    extras: {
      bottom: (
        <DialogFooter className="mt-4">
          <Button
            type="button"
            variant="ghost"
            onClick={() => onOpenChange(false)}
          >
            Cancel
          </Button>
          <Button type="submit" disabled={isSubmitting}>
            {isSubmitting ? (
              <Spinner className="mr-2" />
            ) : (
              <HugeiconsIcon icon={Tick01Icon} className="size-4 mr-2" />
            )}
            Update Exam
          </Button>
        </DialogFooter>
      ),
    },
  })

  const preload = React.useCallback(
    (
      form: UseFormReturn<UpdateExamFormValues, unknown, UpdateExamFormValues>,
    ) => {
      if (!open) {
        form.reset()
      } else if (exam) {
        form.reset({
          name: exam.name,
          academic_year_id: exam.academic_year_id,
          exam_type_id: exam.exam_type_id,
          start_date: exam.start_date,
          end_date: exam.end_date,
          term_id: exam.term_id, // Assuming term_id is also part of the schema but not in form
        })
      }
    },
    [open, exam],
  )

  return (
    <Dialog open={open} onOpenChange={onOpenChange}>
      <DialogContent className="max-w-xl">
        <DialogHeader>
          <DialogTitle>Edit Exam</DialogTitle>
        </DialogHeader>
        <FormBuilder
          schema={updateExamSchema}
          config={config}
          defaultValues={{
            name: '',
            academic_year_id: '',
            exam_type_id: '',
            start_date: '',
            end_date: '',
            term_id: '',
          }}
          onSubmit={onConfirm}
          preload={preload}
          isLoading={isSubmitting}
          showErrorSummary={false}
          toastErrors={false}
          showSuccessAlert={false}
          actions={[]}
          className="grid gap-4 py-4"
        />
      </DialogContent>
    </Dialog>
  )
}
