import * as React from 'react'
import type { GradePeriodResponse } from '@/lib/api/types.gen'
import {
  GradePeriodFormValues,
  gradePeriodFormSchema,
} from '../schemas'
import {
  Dialog,
  DialogContent,
  DialogHeader,
  DialogTitle,
} from '@/components/ui/dialog'
import { FormBuilder, defineFormConfig } from '@/components/form-builder'
import type { UseFormReturn } from 'react-hook-form'

interface GradePeriodDialogProps {
  open: boolean
  onOpenChange: (open: boolean) => void
  onConfirm: (data: GradePeriodFormValues) => void
  isSubmitting?: boolean
  period?: GradePeriodResponse | null
  gradeId: string
  title: string
}

export function GradePeriodDialog({
  open,
  onOpenChange,
  onConfirm,
  isSubmitting,
  period,
  gradeId,
  title,
}: GradePeriodDialogProps) {
  const preload = React.useCallback(
    (form: UseFormReturn<any>) => {
      if (period) {
        form.reset({
          grade_id: period.grade_id,
          period_number: period.period_number,
          start_time: period.start_time,
          end_time: period.end_time,
          is_break: period.is_break,
        })
      } else {
        form.reset({
          grade_id: gradeId,
          period_number: 1,
          start_time: '08:00:00',
          end_time: '08:40:00',
          is_break: false,
        })
      }
    },
    [period, gradeId],
  )

  const config = defineFormConfig(gradePeriodFormSchema, {
    structure: [
      [
        {
          type: 'input',
          field: 'period_number',
          label: 'Period Number',
          inputType: 'number',
          parse: (v) => Number(v),
        },
      ],
      [
        {
          type: 'input',
          field: 'start_time',
          label: 'Start Time (HH:MM:SS)',
          placeholder: '08:00:00',
        },
        {
          type: 'input',
          field: 'end_time',
          label: 'End Time (HH:MM:SS)',
          placeholder: '08:40:00',
        },
      ],
      [
        {
          type: 'checkbox',
          field: 'is_break',
          label: 'Is Break?',
          description: 'Check this if this period is a break or interval.',
        },
      ],
    ],
  })

  return (
    <Dialog open={open} onOpenChange={onOpenChange}>
      <DialogContent>
        <DialogHeader>
          <DialogTitle>{title}</DialogTitle>
        </DialogHeader>

        <FormBuilder
          schema={gradePeriodFormSchema}
          config={config}
          defaultValues={{
            grade_id: gradeId,
            period_number: 1,
            start_time: '08:00:00',
            end_time: '08:40:00',
            is_break: false,
          }}
          onSubmit={async (values) => {
            onConfirm({
              ...values,
              is_break: !!values.is_break
            })
          }}
          preload={preload}
          isLoading={isSubmitting}
          showErrorSummary={false}
          toastErrors={true}
          showSuccessAlert={false}
          actions={[
            {
              label: 'Cancel',
              variant: 'ghost',
              onClick: () => onOpenChange(false),
            },
            {
              label: 'Save Period',
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
