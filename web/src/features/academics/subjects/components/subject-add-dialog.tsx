import { HugeiconsIcon } from '@hugeicons/react'
import { FloppyDiskIcon } from '@hugeicons/core-free-icons'
import * as React from 'react'
import { subjectFormSchema } from '../schemas'
import type { SubjectFormValues } from '../schemas'
import type { UseFormReturn } from 'react-hook-form'
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

interface SubjectAddDialogProps {
  open: boolean
  onOpenChange: (open: boolean) => void
  onConfirm: (data: SubjectFormValues) => void
  isSubmitting?: boolean
}

export function SubjectAddDialog({
  open,
  onOpenChange,
  onConfirm,
  isSubmitting,
}: SubjectAddDialogProps) {
  const preload = React.useCallback(
    (form: UseFormReturn<SubjectFormValues, unknown, SubjectFormValues>) => {
      if (!open) {
        form.reset()
      }
    },
    [open],
  )

  const config = defineFormConfig(subjectFormSchema, {
    structure: [
      [
        {
          field: 'id',
          type: 'input',
          label: 'ID',
          placeholder: 'e.g. SUBJ-MATH',
        },
        {
          field: 'subject_code',
          type: 'input',
          label: 'Code',
          placeholder: 'e.g. MAT101',
        },
      ],
      [
        {
          field: 'subject_name_en',
          type: 'input',
          label: 'Name (EN)',
        },
      ],
      [
        {
          field: 'subject_name_si',
          type: 'input',
          label: 'Name (SI)',
        },
        {
          field: 'subject_name_ta',
          type: 'input',
          label: 'Name (TA)',
        },
      ],
      [
        {
          field: 'is_core',
          type: 'checkbox',
          label: 'Is Core Subject',
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
              <HugeiconsIcon icon={FloppyDiskIcon} className="size-4 mr-2" />
            )}
            Add Subject
          </Button>
        </DialogFooter>
      ),
    },
  })

  return (
    <Dialog open={open} onOpenChange={(val) => onOpenChange(val)}>
      <DialogContent className="max-w-md">
        <DialogHeader>
          <DialogTitle>Add New Subject</DialogTitle>
        </DialogHeader>
        <FormBuilder
          schema={subjectFormSchema}
          config={config}
          defaultValues={{
            id: '',
            subject_code: '',
            subject_name_en: '',
            is_core: true,
            subject_name_si: '',
            subject_name_ta: '',
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
