import * as React from 'react'
import { gradeLevelFormSchema } from '../schemas'
import type { GradeLevelResponse } from '@/lib/api/types.gen'
import type { GradeLevelFormValues } from '../schemas'
import type { UseFormReturn } from 'react-hook-form'
import { Button } from '@/components/ui/button'
import {
  Dialog,
  DialogContent,
  DialogDescription,
  DialogFooter,
  DialogHeader,
  DialogTitle,
} from '@/components/ui/dialog'
import { Spinner } from '@/components/ui/spinner'
import { zEducationLevel } from '@/lib/api/zod.gen'
import { FormBuilder, defineFormConfig } from '@/components/form-builder'

interface GradeLevelEditDialogProps {
  gradeLevel: GradeLevelResponse | null
  open: boolean
  onOpenChange: (open: boolean) => void
  onConfirm: (data: GradeLevelFormValues) => void
  isSubmitting?: boolean
}

const educationLevels = zEducationLevel.options

export function GradeLevelEditDialog({
  gradeLevel,
  open,
  onOpenChange,
  onConfirm,
  isSubmitting,
}: GradeLevelEditDialogProps) {
  const preload = React.useCallback(
    (
      form: UseFormReturn<GradeLevelFormValues, unknown, GradeLevelFormValues>,
    ) => {
      if (gradeLevel) {
        form.reset({
          id: gradeLevel.id,
          grade_name: gradeLevel.grade_name,
          grade_number: gradeLevel.grade_number,
          education_level: gradeLevel.education_level,
        })
      } else if (!open) {
        form.reset()
      }
    },
    [gradeLevel, open],
  )

  const config = defineFormConfig(gradeLevelFormSchema, {
    structure: [
      [
        {
          field: 'id',
          type: 'input',
          label: 'ID',
          disabled: true,
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
        <DialogFooter>
          <Button
            type="button"
            variant="ghost"
            onClick={() => onOpenChange(false)}
          >
            Cancel
          </Button>
          <Button type="submit" disabled={isSubmitting}>
            {isSubmitting && <Spinner className="mr-2" />}
            Save Changes
          </Button>
        </DialogFooter>
      ),
    },
  })

  return (
    <Dialog open={open} onOpenChange={onOpenChange}>
      <DialogContent>
        <DialogHeader>
          <DialogTitle>Edit Grade Level</DialogTitle>
          <DialogDescription>
            Update the details of the academic grade level.
          </DialogDescription>
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
          className="space-y-4"
        />
      </DialogContent>
    </Dialog>
  )
}
