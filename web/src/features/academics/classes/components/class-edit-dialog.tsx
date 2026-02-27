import { HugeiconsIcon } from '@hugeicons/react'
import { Tick01Icon } from '@hugeicons/core-free-icons'
import * as React from 'react'
import { useQuery } from '@tanstack/react-query'
import { classFormSchema } from '../schemas'
import type { ClassResponse } from '@/lib/api/types.gen'
import type { ClassFormValues } from '../schemas'
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
import { authClient } from '@/lib/clients'
import {
  getAllAcademicYearsOptions,
  getAllGradeLevelsOptions,
} from '@/lib/api/@tanstack/react-query.gen'
import { zMedium } from '@/lib/api/zod.gen'
import { FormBuilder, defineFormConfig } from '@/components/form-builder'

interface ClassEditDialogProps {
  classItem: ClassResponse | null
  open: boolean
  onOpenChange: (open: boolean) => void
  onConfirm: (data: ClassFormValues) => void
  isSubmitting?: boolean
}

const mediums = zMedium.options

export function ClassEditDialog({
  classItem,
  open,
  onOpenChange,
  onConfirm,
  isSubmitting,
}: ClassEditDialogProps) {
  const { data: academicYearsData } = useQuery(
    getAllAcademicYearsOptions({ client: authClient }),
  )
  const academicYears = academicYearsData?.data || []

  const { data: gradeLevelsData } = useQuery(
    getAllGradeLevelsOptions({ client: authClient }),
  )
  const gradeLevels = gradeLevelsData?.data || []

  const preload = React.useCallback(
    (form: UseFormReturn<ClassFormValues, unknown, ClassFormValues>) => {
      if (classItem) {
        form.reset({
          id: classItem.id,
          section_name: classItem.section_name,
          grade_id: classItem.grade_id,
          academic_year_id: classItem.academic_year_id,
          max_capacity: classItem.max_capacity,
          medium: classItem.medium,
        })
      } else if (!open) {
        form.reset()
      }
    },
    [classItem, open],
  )

  const config = defineFormConfig(classFormSchema, {
    structure: [
      [
        {
          field: 'id',
          type: 'input',
          label: 'ID',
          disabled: true,
        },
        {
          field: 'section_name',
          type: 'input',
          label: 'Section Name',
          placeholder: 'e.g. A',
        },
      ],
      [
        {
          field: 'grade_id',
          type: 'select',
          label: 'Grade',
          placeholder: 'Select a grade level',
          items: gradeLevels.map((gl) => ({
            label: gl.grade_name,
            value: gl.id,
          })),
          parse: (value) => value,
        },
        {
          field: 'academic_year_id',
          type: 'select',
          label: 'Academic Year',
          placeholder: 'Select an academic year',
          items: academicYears.map((ay) => ({
            label: ay.name,
            value: ay.id,
          })),
          parse: (value) => value,
        },
      ],
      [
        {
          field: 'medium',
          type: 'select',
          label: 'Medium',
          placeholder: 'Select medium',
          items: mediums.map((m) => ({
            label: m,
            value: m,
          })),
          parse: (value) => zMedium.parse(value),
        },
        {
          field: 'max_capacity',
          type: 'input',
          label: 'Max Capacity',
          inputType: 'number',
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
            Save Changes
          </Button>
        </DialogFooter>
      ),
    },
  })

  return (
    <Dialog open={open} onOpenChange={(val) => onOpenChange(val)}>
      <DialogContent className="max-w-md">
        <DialogHeader>
          <DialogTitle>Edit Class</DialogTitle>
        </DialogHeader>
        <FormBuilder
          schema={classFormSchema}
          config={config}
          defaultValues={{
            id: '',
            section_name: '',
            grade_id: '',
            academic_year_id: '',
            max_capacity: 40,
            medium: 'English',
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
