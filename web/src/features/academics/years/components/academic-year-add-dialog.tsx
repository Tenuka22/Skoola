'use client'

import { academicYearFormSchema } from '../schemas'
import type { AcademicYearFormValues } from '../schemas'
import {
  Dialog,
  DialogContent,
  DialogDescription,
  DialogFooter,
  DialogHeader,
  DialogTitle,
} from '@/components/ui/dialog'
import { Button } from '@/components/ui/button'
import { Spinner } from '@/components/ui/spinner'
import { FormBuilder, defineFormConfig } from '@/components/form-builder'

interface AcademicYearAddDialogProps {
  open: boolean
  onOpenChange: (open: boolean) => void
  onConfirm: (data: AcademicYearFormValues) => void
  isSubmitting?: boolean
}

export function AcademicYearAddDialog({
  open,
  onOpenChange,
  onConfirm,
  isSubmitting,
}: AcademicYearAddDialogProps) {
  const config = defineFormConfig(academicYearFormSchema, {
    structure: [
      [
        {
          field: 'id',
          type: 'input',
          label: 'ID',
          placeholder: 'e.g., AY-2024',
        },
        {
          field: 'name',
          type: 'input',
          label: 'Name',
          placeholder: 'e.g., 2024-2025',
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
        <DialogFooter>
          <Button variant="ghost" onClick={() => onOpenChange(false)}>
            Cancel
          </Button>
          <Button type="submit" disabled={isSubmitting}>
            {isSubmitting && <Spinner className="mr-2" />}
            Create
          </Button>
        </DialogFooter>
      ),
    },
  })

  return (
    <Dialog open={open} onOpenChange={onOpenChange}>
      <DialogContent>
        <DialogHeader>
          <DialogTitle>Add Academic Year</DialogTitle>
          <DialogDescription>Create a new academic year.</DialogDescription>
        </DialogHeader>
        <FormBuilder
          schema={academicYearFormSchema}
          config={config}
          defaultValues={{
            id: '',
            name: '',
            start_date: '',
            end_date: '',
          }}
          onSubmit={(values) => onConfirm(values)}
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
