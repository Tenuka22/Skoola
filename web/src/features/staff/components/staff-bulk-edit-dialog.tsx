'use client'

import { Controller } from 'react-hook-form'
import { bulkEditStaffFormSchema } from '../schemas'
import type {
  ControllerFieldState,
  ControllerRenderProps,
  Path,
} from 'react-hook-form'
import type { BulkEditStaffFormValues } from '../schemas'
import {
  Dialog,
  DialogContent,
  DialogDescription,
  DialogFooter,
  DialogHeader,
  DialogTitle,
} from '@/components/ui/dialog'
import { Button } from '@/components/ui/button'
import {
  Select,
  SelectContent,
  SelectItem,
  SelectTrigger,
  SelectValue,
} from '@/components/ui/select'
import { Spinner } from '@/components/ui/spinner'
import { zEmploymentStatus, zStaffType } from '@/lib/api/zod.gen'
import { FormBuilder, defineFormConfig } from '@/components/form-builder'
import { Field, FieldError, FieldLabel } from '@/components/ui/field'

interface StaffBulkEditDialogProps {
  open: boolean
  onOpenChange: (open: boolean) => void
  onConfirm: (data: BulkEditStaffFormValues) => void
  selectedCount: number
  isSubmitting?: boolean
}

export function StaffBulkEditDialog({
  open,
  onOpenChange,
  onConfirm,
  selectedCount,
  isSubmitting,
}: StaffBulkEditDialogProps) {
  const staffTypes = zStaffType.options
  const employmentStatuses = zEmploymentStatus.options

  const config = defineFormConfig(bulkEditStaffFormSchema, {
    structure: [],
    extras: {
      top: (form) => (
        <>
          <Controller
            name="staff_type"
            control={form.control}
            render={({
              field,
              fieldState,
            }: {
              field: ControllerRenderProps<
                BulkEditStaffFormValues,
                Path<BulkEditStaffFormValues>
              >
              fieldState: ControllerFieldState
            }) => (
              <Field data-invalid={fieldState.invalid}>
                <FieldLabel htmlFor="staff_type">Staff Type</FieldLabel>
                <Select
                  {...field}
                  value={typeof field.value === 'string' ? field.value : ''}
                  onValueChange={field.onChange}
                >
                  <SelectTrigger
                    id="staff_type"
                    aria-invalid={fieldState.invalid}
                  >
                    <SelectValue placeholder="Select a type" />
                  </SelectTrigger>
                  <SelectContent>
                    {staffTypes.map((type) => (
                      <SelectItem key={type} value={type}>
                        {type}
                      </SelectItem>
                    ))}
                  </SelectContent>
                </Select>
                {fieldState.invalid && (
                  <FieldError errors={[fieldState.error]} />
                )}
              </Field>
            )}
          />
          <Controller
            name="employment_status"
            control={form.control}
            render={({
              field,
              fieldState,
            }: {
              field: ControllerRenderProps<
                BulkEditStaffFormValues,
                Path<BulkEditStaffFormValues>
              >
              fieldState: ControllerFieldState
            }) => (
              <Field data-invalid={fieldState.invalid}>
                <FieldLabel htmlFor="employment_status">
                  Employment Status
                </FieldLabel>
                <Select
                  {...field}
                  value={typeof field.value === 'string' ? field.value : ''}
                  onValueChange={field.onChange}
                >
                  <SelectTrigger
                    id="employment_status"
                    aria-invalid={fieldState.invalid}
                  >
                    <SelectValue placeholder="Select a status" />
                  </SelectTrigger>
                  <SelectContent>
                    {employmentStatuses.map((status) => (
                      <SelectItem key={status} value={status}>
                        {status}
                      </SelectItem>
                    ))}
                  </SelectContent>
                </Select>
                {fieldState.invalid && (
                  <FieldError errors={[fieldState.error]} />
                )}
              </Field>
            )}
          />
        </>
      ),
      bottom: (
        <DialogFooter>
          <Button variant="ghost" onClick={() => onOpenChange(false)}>
            Cancel
          </Button>
          <Button type="submit" disabled={isSubmitting}>
            {isSubmitting && <Spinner className="mr-2" />}
            Save changes
          </Button>
        </DialogFooter>
      ),
    },
  })

  return (
    <Dialog open={open} onOpenChange={onOpenChange}>
      <DialogContent>
        <DialogHeader>
          <DialogTitle>Bulk Edit Staff</DialogTitle>
          <DialogDescription>
            This will update {selectedCount} staff members.
          </DialogDescription>
        </DialogHeader>
        <FormBuilder
          schema={bulkEditStaffFormSchema}
          config={config}
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
