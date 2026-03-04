import { HugeiconsIcon } from '@hugeicons/react'
import { Tick01Icon } from '@hugeicons/core-free-icons'
import * as React from 'react'
import { syllabusTopicSchema } from '../schemas'
import type { SyllabusTopicFormValues } from '../schemas'
import type { UseFormReturn } from 'react-hook-form'
import type { SyllabusResponse } from '@/lib/api/types.gen'
import { Button } from '@/components/ui/button'
import {
  Dialog,
  DialogContent,
  DialogFooter,
  DialogHeader,
  DialogTitle,
} from '@/components/ui/dialog'
import { Spinner } from '@/components/ui/spinner'
import { FormBuilder, defineFormConfig } from '@/components/form-builder'

interface SyllabusDialogProps {
  open: boolean
  onOpenChange: (open: boolean) => void
  onConfirm: (data: SyllabusTopicFormValues) => void
  isSubmitting?: boolean
  topic?: SyllabusResponse | null
  parentTopic?: SyllabusResponse | null
  standardId: string
  title: string
}

export function SyllabusDialog({
  open,
  onOpenChange,
  onConfirm,
  isSubmitting,
  topic,
  parentTopic,
  standardId,
  title,
}: SyllabusDialogProps) {
  const preload = React.useCallback(
    (
      form: UseFormReturn<
        SyllabusTopicFormValues,
        unknown,
        SyllabusTopicFormValues
      >,
    ) => {
      if (!open) {
        form.reset()
      } else if (topic) {
        form.reset({
          curriculum_standard_id: topic.curriculum_standard_id,
          parent_id: topic.parent_id || null,
          topic_name: topic.topic_name,
          description: topic.description || '',
          required_periods: topic.required_periods,
          buffer_periods: topic.buffer_periods,
          is_practical: topic.is_practical,
          suggested_duration_hours: topic.suggested_duration_hours || null,
        })
      } else {
        form.reset({
          curriculum_standard_id: standardId,
          parent_id: parentTopic?.id || null,
          topic_name: '',
          description: '',
          required_periods: 1,
          buffer_periods: 0,
          is_practical: false,
          suggested_duration_hours: null,
        })
      }
    },
    [open, topic, parentTopic, standardId],
  )

  const config = defineFormConfig(syllabusTopicSchema, {
    structure: [
      [
        {
          field: 'topic_name',
          type: 'input',
          label: 'Topic Name',
          placeholder: 'e.g. Introduction to Algebra',
        },
      ],
      [
        {
          field: 'required_periods',
          type: 'input',
          label: 'Required Periods',
          inputType: 'number',
        },
        {
          field: 'buffer_periods',
          type: 'input',
          label: 'Buffer Periods',
          inputType: 'number',
        },
      ],
      [
        {
          field: 'suggested_duration_hours',
          type: 'input',
          label: 'Duration (Hours)',
          inputType: 'number',
        },
        {
          field: 'is_practical',
          type: 'switch',
          label: 'Is Practical',
        },
      ],
      [
        {
          field: 'description',
          type: 'textarea',
          label: 'Description',
          placeholder: 'Topic details and objectives...',
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
            {topic ? 'Update Topic' : 'Add Topic'}
          </Button>
        </DialogFooter>
      ),
    },
  })

  return (
    <Dialog open={open} onOpenChange={(val) => onOpenChange(val)}>
      <DialogContent className="max-w-xl">
        <DialogHeader>
          <DialogTitle>{title}</DialogTitle>
        </DialogHeader>
        <FormBuilder
          schema={syllabusTopicSchema}
          config={config}
          defaultValues={{
            curriculum_standard_id: standardId,
            parent_id: parentTopic?.id || null,
            topic_name: '',
            description: '',
            required_periods: 1,
            buffer_periods: 0,
            is_practical: false,
            suggested_duration_hours: null,
          }}
          onSubmit={(values) => onConfirm(values)}
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
