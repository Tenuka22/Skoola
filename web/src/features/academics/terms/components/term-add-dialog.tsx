import { HugeiconsIcon } from '@hugeicons/react'
import { FloppyDiskIcon } from '@hugeicons/core-free-icons'
import * as React from 'react'
import { useQuery } from '@tanstack/react-query'

import { termFormSchema } from '../schemas'
import type { TermFormValues } from '../schemas'
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
import { getAllAcademicYearsOptions } from '@/lib/api/@tanstack/react-query.gen'
import { authClient } from '@/lib/clients'
import { FormBuilder, defineFormConfig } from '@/components/form-builder'

interface TermAddDialogProps {
  open: boolean
  onOpenChange: (open: boolean) => void
  onConfirm: (data: TermFormValues) => void
  isSubmitting?: boolean
}

export function TermAddDialog({
  open,
  onOpenChange,
  onConfirm,
  isSubmitting,
}: TermAddDialogProps) {
  const { data: academicYearsData } = useQuery(
    getAllAcademicYearsOptions({ client: authClient }),
  )
  const academicYears = academicYearsData?.data || []

  const preload = React.useCallback(
    (form: UseFormReturn<TermFormValues, unknown, TermFormValues>) => {
      if (!open) {
        form.reset()
      }
    },
    [open],
  )

  const config = defineFormConfig(termFormSchema, {
    structure: [],
    extras: {
      top: (form) => (
        <>
          <div className="grid grid-cols-4 items-center gap-4">
            <Label htmlFor="name" className="text-right">
              Name
            </Label>
            <Input
              id="name"
              {...form.register('name')}
              className="col-span-3"
            />
            {form.formState.errors.name && (
              <p className="col-span-4 col-start-2 text-sm font-medium text-red-500">
                {form.formState.errors.name.message}
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
          <div className="grid grid-cols-4 items-center gap-4">
            <Label htmlFor="term_number" className="text-right">
              Term Number
            </Label>
            <Input
              id="term_number"
              type="number"
              {...form.register('term_number', { valueAsNumber: true })}
              className="col-span-3"
            />
            {form.formState.errors.term_number && (
              <p className="col-span-4 col-start-2 text-sm font-medium text-red-500">
                {form.formState.errors.term_number.message}
              </p>
            )}
          </div>
          <div className="grid grid-cols-4 items-center gap-4">
            <Label htmlFor="start_date" className="text-right">
              Start Date
            </Label>
            <Input
              id="start_date"
              type="date"
              {...form.register('start_date')}
              className="col-span-3"
            />
            {form.formState.errors.start_date && (
              <p className="col-span-4 col-start-2 text-sm font-medium text-red-500">
                {form.formState.errors.start_date.message}
              </p>
            )}
          </div>
          <div className="grid grid-cols-4 items-center gap-4">
            <Label htmlFor="end_date" className="text-right">
              End Date
            </Label>
            <Input
              id="end_date"
              type="date"
              {...form.register('end_date')}
              className="col-span-3"
            />
            {form.formState.errors.end_date && (
              <p className="col-span-4 col-start-2 text-sm font-medium text-red-500">
                {form.formState.errors.end_date.message}
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
              <HugeiconsIcon icon={FloppyDiskIcon} className="size-4 mr-2" />
            )}
            Add Term
          </Button>
        </DialogFooter>
      ),
    },
  })

  return (
    <Dialog open={open} onOpenChange={(val) => onOpenChange(val)}>
      <DialogContent>
        <DialogHeader>
          <DialogTitle>Add New Term</DialogTitle>
        </DialogHeader>
        <FormBuilder
          schema={termFormSchema}
          config={config}
          defaultValues={{
            name: '',
            start_date: '',
            end_date: '',
            academic_year_id: '',
            term_number: 1,
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
