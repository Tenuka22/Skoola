import { HugeiconsIcon } from '@hugeicons/react'
import { Tick01Icon } from '@hugeicons/core-free-icons'
import * as React from 'react'
import { useQuery } from '@tanstack/react-query'
import { classFormSchema } from '../schemas'
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
import { Input } from '@/components/ui/input'
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
  getAllGradeLevelsOptions,
} from '@/lib/api/@tanstack/react-query.gen'
import { zMedium } from '@/lib/api/zod.gen'
import { FormBuilder, defineFormConfig } from '@/components/form-builder'

interface ClassAddDialogProps {
  open: boolean
  onOpenChange: (open: boolean) => void
  onConfirm: (data: ClassFormValues) => void
  isSubmitting?: boolean
}

const mediums = zMedium.options

export function ClassAddDialog({
  open,
  onOpenChange,
  onConfirm,
  isSubmitting,
}: ClassAddDialogProps) {
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
      if (!open) {
        form.reset()
      }
    },
    [open],
  )

  const config = defineFormConfig(classFormSchema, {
    structure: [],
    extras: {
      top: (form) => (
        <>
          <div className="grid grid-cols-4 items-center gap-4">
            <Label htmlFor="id" className="text-right">
              ID
            </Label>
            <Input
              id="id"
              {...form.register('id')}
              placeholder="e.g. CLASS-1A"
              className="col-span-3"
            />
            {form.formState.errors.id && (
              <p className="col-span-4 col-start-2 text-sm font-medium text-red-500">
                {form.formState.errors.id.message}
              </p>
            )}
          </div>
          <div className="grid grid-cols-4 items-center gap-4">
            <Label htmlFor="section_name" className="text-right text-xs">
              Section Name
            </Label>
            <Input
              id="section_name"
              {...form.register('section_name')}
              placeholder="e.g. A"
              className="col-span-3"
            />
            {form.formState.errors.section_name && (
              <p className="col-span-4 col-start-2 text-sm font-medium text-red-500">
                {form.formState.errors.section_name.message}
              </p>
            )}
          </div>
          <div className="grid grid-cols-4 items-center gap-4">
            <Label htmlFor="grade_id" className="text-right">
              Grade
            </Label>
            <Select
              onValueChange={(value) => form.setValue('grade_id', value || '')}
              value={form.watch('grade_id')}
            >
              <SelectTrigger id="grade_id" className="col-span-3">
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
          <div className="grid grid-cols-4 items-center gap-4">
            <Label htmlFor="academic_year_id" className="text-right text-xs">
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
          <div className="grid grid-cols-4 items-center gap-4">
            <Label htmlFor="medium" className="text-right">
              Medium
            </Label>
            <Select
              onValueChange={(value) =>
                form.setValue('medium', value ?? 'English')
              }
              value={form.watch('medium')}
            >
              <SelectTrigger id="medium" className="col-span-3">
                <SelectValue placeholder="Select medium" />
              </SelectTrigger>
              <SelectContent>
                {mediums.map((m) => (
                  <SelectItem key={m} value={m}>
                    {m}
                  </SelectItem>
                ))}
              </SelectContent>
            </Select>
          </div>
          <div className="grid grid-cols-4 items-center gap-4">
            <Label htmlFor="max_capacity" className="text-right text-xs">
              Max Capacity
            </Label>
            <Input
              id="max_capacity"
              type="number"
              {...form.register('max_capacity', { valueAsNumber: true })}
              className="col-span-3"
            />
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
            Add Class
          </Button>
        </DialogFooter>
      ),
    },
  })

  return (
    <Dialog open={open} onOpenChange={(val) => onOpenChange(val)}>
      <DialogContent className="max-w-md">
        <DialogHeader>
          <DialogTitle>Add New Class</DialogTitle>
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
