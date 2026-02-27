import { HugeiconsIcon } from '@hugeicons/react'
import { Tick01Icon } from '@hugeicons/core-free-icons'
import * as React from 'react'
import { useQueries } from '@tanstack/react-query'
import type { StaffResponse } from '@/lib/api/types.gen'
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
import {
  getAllAcademicYearsOptions,
  getAllClassesOptions,
} from '@/lib/api/@tanstack/react-query.gen'
import { zAssignClassToTeacherRequest } from '@/lib/api/zod.gen'
import { FormBuilder, defineFormConfig } from '@/components/form-builder'

type FormValues = z.infer<typeof zAssignClassToTeacherRequest>

interface StaffAssignClassDialogProps {
  staff: StaffResponse | null
  open: boolean
  onOpenChange: (open: boolean) => void
  onConfirm: (staffId: string, data: FormValues) => void
  isSubmitting?: boolean
}

export function StaffAssignClassDialog({
  staff,
  open,
  onOpenChange,
  onConfirm,
  isSubmitting,
}: StaffAssignClassDialogProps) {
  const [academicYearsQuery, classesQuery] = useQueries({
    queries: [
      {
        ...getAllAcademicYearsOptions({ client: authClient }),
        staleTime: Infinity,
      },
      { ...getAllClassesOptions({ client: authClient }), staleTime: Infinity },
    ],
  })

  const academicYears = academicYearsQuery.data?.data || []
  const classes = classesQuery.data?.data || []

  const preload = React.useCallback(
    (form: UseFormReturn<FormValues, unknown, FormValues>) => {
      if (!open) {
        form.reset()
      }
    },
    [open],
  )

  const config = defineFormConfig(zAssignClassToTeacherRequest, {
    structure: [],
    extras: {
      top: (form) => (
        <>
          <p className="text-sm text-muted-foreground">
            Assign{' '}
            <span className="font-medium text-foreground">{staff?.name}</span>{' '}
            to a specific class for an academic year.
          </p>
          <div className="grid grid-cols-4 items-center gap-4">
            <Label htmlFor="class_id" className="text-right">
              Class
            </Label>
            <Select
              onValueChange={(value) => form.setValue('class_id', value || '')}
              value={form.watch('class_id')}
            >
              <SelectTrigger id="class_id" className="col-span-3">
                <SelectValue placeholder="Select a class" />
              </SelectTrigger>
              <SelectContent>
                {classes.map((cls) => (
                  <SelectItem key={cls.id} value={cls.id}>
                    {cls.section_name}
                  </SelectItem>
                ))}
              </SelectContent>
            </Select>
            {form.formState.errors.class_id && (
              <p className="col-span-4 col-start-2 text-sm font-medium text-red-500">
                {form.formState.errors.class_id.message}
              </p>
            )}
          </div>
          <div className="grid grid-cols-4 items-center gap-4">
            <Label htmlFor="academic_year_id" className="text-right">
              Academic Year
            </Label>
            <Select
              onValueChange={(value) =>
                form.setValue('academic_year_id', value || '')
              }
              value={form.watch('academic_year_id')}
            >
              <SelectTrigger id="academic_year_id" className="col-span-3">
                <SelectValue placeholder="Select an academic year" />
              </SelectTrigger>
              <SelectContent>
                {academicYears.map((ay) => (
                  <SelectItem key={ay.id} value={ay.id}>
                    {ay.name}
                  </SelectItem>
                ))}
              </SelectContent>
            </Select>
            {form.formState.errors.academic_year_id && (
              <p className="col-span-4 col-start-2 text-sm font-medium text-red-500">
                {form.formState.errors.academic_year_id.message}
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
            Assign Class
          </Button>
        </DialogFooter>
      ),
    },
  })

  return (
    <Dialog open={open} onOpenChange={(val) => onOpenChange(val)}>
      <DialogContent>
        <DialogHeader>
          <DialogTitle>Assign Class to {staff?.name}</DialogTitle>
        </DialogHeader>
        <FormBuilder
          schema={zAssignClassToTeacherRequest}
          config={config}
          defaultValues={{ class_id: '', academic_year_id: '' }}
          onSubmit={(values) => {
            if (staff) onConfirm(staff.id, values)
          }}
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
