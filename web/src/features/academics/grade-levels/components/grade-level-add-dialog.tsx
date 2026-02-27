import { HugeiconsIcon } from '@hugeicons/react'
import { FloppyDiskIcon } from '@hugeicons/core-free-icons'
import * as React from 'react'
import { gradeLevelFormSchema } from '../schemas'
import type { GradeLevelFormValues } from '../schemas'
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
import { zEducationLevel } from '@/lib/api/zod.gen'
import { FormBuilder, defineFormConfig } from '@/components/form-builder'

interface GradeLevelAddDialogProps {
  open: boolean
  onOpenChange: (open: boolean) => void
  onConfirm: (data: GradeLevelFormValues) => void
  isSubmitting?: boolean
}

const educationLevels = zEducationLevel.options

export function GradeLevelAddDialog({
  open,
  onOpenChange,
  onConfirm,
  isSubmitting,
}: GradeLevelAddDialogProps) {
  const preload = React.useCallback(
    (
      form: UseFormReturn<GradeLevelFormValues, unknown, GradeLevelFormValues>,
    ) => {
      if (!open) {
        form.reset()
      }
    },
    [open],
  )

  const config = defineFormConfig(gradeLevelFormSchema, {
    structure: [
      [
        {
          field: 'id',
          type: 'input',
          label: 'ID',
          placeholder: 'e.g. GRADE-1',
        },
        {
          field: 'grade_name',
          type: 'input',
          label: 'Grade Name',
          placeholder: 'e.g. Grade 1',
        },
      ],
      [
        {
          field: 'grade_number',
          type: 'input',
          label: 'Grade Number',
          inputType: 'number',
        },
        {
          field: 'education_level',
          type: 'select',
          label: 'Education Level',
          placeholder: 'Select level',
          items: educationLevels.map((level) => ({
            label: level,
            value: level,
          })),
          parse: (val) => zEducationLevel.parse(val),
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
            Add Grade Level
          </Button>
        </DialogFooter>
      ),
    },
  })

  return (
    <Dialog open={open} onOpenChange={(val) => onOpenChange(val)}>
      <DialogContent>
        <DialogHeader>
          <DialogTitle>Add New Grade Level</DialogTitle>
        </DialogHeader>
        <FormBuilder
          schema={gradeLevelFormSchema}
          config={config}
          defaultValues={{
            id: '',
            grade_name: '',
            grade_number: 1,
            education_level: 'Primary',
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
