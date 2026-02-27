import { HugeiconsIcon } from '@hugeicons/react'
import { Tick01Icon } from '@hugeicons/core-free-icons'
import * as React from 'react'
import { useQuery } from '@tanstack/react-query'
import { assignSubjectToGradeSchema } from '../schemas'
import type { SubjectResponse } from '@/lib/api/types.gen'
import type { z } from 'zod'
import type { UseFormReturn } from 'react-hook-form'
import { Button } from '@/components/ui/button'
import {
  Dialog,
  DialogContent,
  DialogFooter,
  DialogHeader,
  DialogTitle,
} from '@/components/ui/dialog'
import { Label } from '@/components/ui/label'
import { Spinner } from '@/components/ui/spinner'
import {
  Select,
  SelectContent,
  SelectItem,
  SelectTrigger,
  SelectValue,
} from '@/components/ui/select'
import { authClient } from '@/lib/clients'
import { getAllGradeLevelsOptions } from '@/lib/api/@tanstack/react-query.gen'
import { FormBuilder, defineFormConfig } from '@/components/form-builder'

type FormValues = z.infer<typeof assignSubjectToGradeSchema>

interface SubjectAssignToGradeDialogProps {
  subject: SubjectResponse | null
  open: boolean
  onOpenChange: (open: boolean) => void
  onConfirm: (gradeId: string) => void
  isSubmitting?: boolean
}

export function SubjectAssignToGradeDialog({
  subject,
  open,
  onOpenChange,
  onConfirm,
  isSubmitting,
}: SubjectAssignToGradeDialogProps) {
  const { data: gradeLevelsData } = useQuery(
    getAllGradeLevelsOptions({ client: authClient }),
  )
  const gradeLevels = gradeLevelsData?.data || []

  const preload = React.useCallback(
    (form: UseFormReturn<FormValues, unknown, FormValues>) => {
      if (!open) {
        form.reset()
        return
      }
      if (subject) {
        form.setValue('subject_id', subject.id)
      }
    },
    [open, subject],
  )

  const config = defineFormConfig(assignSubjectToGradeSchema, {
    structure: [],
    extras: {
      top: (form) => (
        <>
          <p className="text-sm text-muted-foreground">
            Assign{' '}
            <span className="font-medium text-foreground">
              {subject?.subject_name_en}
            </span>{' '}
            to a specific grade level.
          </p>
          <div className="grid grid-cols-4 items-center gap-4">
            <Label htmlFor="grade_id" className="text-right">
              Grade Level
            </Label>
            <Select
              onValueChange={(value) => form.setValue('grade_id', value || '')}
              value={form.watch('grade_id')}
            >
              <SelectTrigger className="col-span-3">
                <SelectValue placeholder="Select a grade level" />
              </SelectTrigger>
              <SelectContent>
                {gradeLevels.map((gl) => (
                  <SelectItem key={gl.id} value={gl.id}>
                    {gl.grade_name}
                  </SelectItem>
                ))}
              </SelectContent>
            </Select>
            {form.formState.errors.grade_id && (
              <p className="col-span-4 col-start-2 text-sm font-medium text-red-500">
                {form.formState.errors.grade_id.message}
              </p>
            )}
          </div>
        </>
      ),
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
            Assign
          </Button>
        </DialogFooter>
      ),
    },
  })

  return (
    <Dialog open={open} onOpenChange={(val) => onOpenChange(val)}>
      <DialogContent>
        <DialogHeader>
          <DialogTitle>Assign Subject to Grade Level</DialogTitle>
        </DialogHeader>
        <FormBuilder
          schema={assignSubjectToGradeSchema}
          config={config}
          defaultValues={{
            grade_id: '',
            subject_id: subject?.id || '',
          }}
          onSubmit={(values) => onConfirm(values.grade_id)}
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
