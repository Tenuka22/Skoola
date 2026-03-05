import { HugeiconsIcon } from '@hugeicons/react'
import { Tick01Icon } from '@hugeicons/core-free-icons'
import * as React from 'react'
import { useQuery } from '@tanstack/react-query'
import { curriculumStandardSchema } from '../schemas'
import type { CurriculumStandardFormValues } from '../schemas'
import type { UseFormReturn } from 'react-hook-form'
import type { CurriculumStandardResponse } from '@/lib/api/types.gen'
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
  getAllGradeLevelsOptions,
  getAllSubjectsOptions,
} from '@/lib/api/@tanstack/react-query.gen'
import { zMedium } from '@/lib/api/zod.gen'
import { FormBuilder, defineFormConfig } from '@/components/form-builder'

interface CurriculumDialogProps {
  open: boolean
  onOpenChange: (open: boolean) => void
  onConfirm: (data: CurriculumStandardFormValues) => void
  isSubmitting?: boolean
  standard?: CurriculumStandardResponse | null
  title: string
}

const mediums = zMedium.options

export function CurriculumDialog({
  open,
  onOpenChange,
  onConfirm,
  isSubmitting,
  standard,
  title,
}: CurriculumDialogProps) {
  const { data: subjectsData } = useQuery(
    getAllSubjectsOptions({ client: authClient }),
  )
  const subjects = subjectsData?.data || []

  const { data: gradeLevelsData } = useQuery(
    getAllGradeLevelsOptions({ client: authClient }),
  )
  const gradeLevels = gradeLevelsData?.data || []

  const preload = React.useCallback(
    (
      form: UseFormReturn<
        CurriculumStandardFormValues,
        unknown,
        CurriculumStandardFormValues
      >,
    ) => {
      if (!open) {
        form.reset()
      } else if (standard) {
        form.reset({
          subject_id: standard.subject_id,
          grade_level_id: standard.grade_level_id,
          standard_code: standard.standard_code,
          description: standard.description || '',
          medium: standard.medium,
          version_name: standard.version_name || '',
          start_date: standard.start_date || '',
          end_date: standard.end_date || '',
          is_active: standard.is_active,
        })
      }
    },
    [open, standard],
  )

  const config = defineFormConfig(curriculumStandardSchema, {
    structure: [
      [
        {
          field: 'standard_code',
          type: 'input',
          label: 'Standard Code',
          placeholder: 'e.g. CUR-2026-MATH-G10',
        },
        {
          field: 'version_name',
          type: 'input',
          label: 'Version Name',
          placeholder: 'e.g. 2026 Revision',
        },
      ],
      [
        {
          field: 'subject_id',
          type: 'select',
          label: 'Subject',
          placeholder: 'Select a subject',
          items: subjects.map((s) => ({
            label: s.subject_name_en,
            value: s.id,
          })),
          parse: (value) => value,
        },
        {
          field: 'grade_level_id',
          type: 'select',
          label: 'Grade Level',
          placeholder: 'Select a grade level',
          items: gradeLevels.map((gl) => ({
            label: gl.grade_name,
            value: gl.id,
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
          field: 'is_active',
          type: 'switch',
          label: 'Is Active',
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
      [
        {
          field: 'description',
          type: 'textarea',
          label: 'Description',
          placeholder: 'Describe the standard...',
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
            {isSubmitting ? (
              <Spinner className="mr-2" />
            ) : (
              <HugeiconsIcon icon={Tick01Icon} className="size-4 mr-2" />
            )}
            {standard ? 'Update Standard' : 'Add Standard'}
          </Button>
        </DialogFooter>
      ),
    },
  })

  return (
    <Dialog open={open} onOpenChange={(val) => onOpenChange(val)}>
      <DialogContent className="max-w-2xl">
        <DialogHeader>
          <DialogTitle>{title}</DialogTitle>
        </DialogHeader>
        <FormBuilder
          schema={curriculumStandardSchema}
          config={config}
          defaultValues={{
            subject_id: '',
            grade_level_id: '',
            standard_code: '',
            description: '',
            medium: 'English',
            version_name: '',
            start_date: '',
            end_date: '',
            is_active: true,
          }}
          onSubmit={(values) => onConfirm(values)}
          preload={preload}
          isLoading={isSubmitting}
          showErrorSummary={false}
          toastErrors={false}
          showSuccessAlert={false}
          actions={[]}
          className="grid gap-4"
        />
      </DialogContent>
    </Dialog>
  )
}
