'use client'

import * as React from 'react'
import { format } from 'date-fns'
import { academicYearFormSchema } from '../schemas'
import type { UseFormReturn } from 'react-hook-form'
import type { AcademicYearFormValues } from '../schemas'
import type { AcademicYearResponse } from '@/lib/api'
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

interface AcademicYearEditDialogProps {
  year: AcademicYearResponse | null
  open: boolean
  onOpenChange: (open: boolean) => void
  onConfirm: (data: AcademicYearFormValues) => void
  isSubmitting?: boolean
}

export function AcademicYearEditDialog({
  year,
  open,
  onOpenChange,
  onConfirm,
  isSubmitting,
}: AcademicYearEditDialogProps) {
  const preload = React.useCallback(
    (
      form: UseFormReturn<
        AcademicYearFormValues,
        unknown,
        AcademicYearFormValues
      >,
    ) => {
      if (year) {
        form.reset({
          name: year.name,
          start_date: format(new Date(String(year.year_start)), 'yyyy-MM-dd'),
          end_date: format(new Date(String(year.year_end)), 'yyyy-MM-dd'),
        })
      } else if (!open) {
        form.reset()
      }
    },
    [year, open],
  )

  const config = defineFormConfig(academicYearFormSchema, {
    structure: [
      [
        {
          field: 'id',
          type: 'input',
          label: 'ID',
          disabled: true,
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
          <DialogTitle>Edit Academic Year</DialogTitle>
          <DialogDescription>
            Update the details of the academic year.
          </DialogDescription>
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
