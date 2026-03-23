import * as React from 'react'
import { UserAdd01Icon } from '@hugeicons/core-free-icons'
import { HugeiconsIcon } from '@hugeicons/react'

import { EMPLOYMENT_STATUSES, GENDERS, STAFF_TYPES } from '../constants'
import { createStaffSchema } from '../schemas'
import type { CreateStaffValues } from '../schemas'
import type { Staff, StaffFormData } from '../types'
import type { UseFormReturn } from 'react-hook-form'
import {
  AlertDialog,
  AlertDialogAction,
  AlertDialogCancel,
  AlertDialogContent,
  AlertDialogDescription,
  AlertDialogFooter,
  AlertDialogHeader,
  AlertDialogTitle,
} from '@/components/ui/alert-dialog'
import {
  Dialog,
  DialogContent,
  DialogDescription,
  DialogTitle,
} from '@/components/ui/dialog'
import { Button } from '@/components/ui/button'
import { Spinner } from '@/components/ui/spinner'
import { FormBuilder, defineFormConfig } from '@/components/form-builder'
import { Stack } from '@/components/primitives'

interface StaffModalsProps {
  staffToDelete: string | null
  setStaffToDelete: (id: string | null) => void
  onDeleteConfirm: (id: string) => void
  isBulkDeleteOpen: boolean
  setIsBulkDeleteOpen: (open: boolean) => void
  onBulkDeleteConfirm: () => void
  staffToEdit: Staff | null
  setStaffToEdit: (staff: Staff | null) => void
  onEditConfirm: (data: StaffFormData) => void
  isCreateStaffOpen: boolean
  setIsCreateStaffOpen: (open: boolean) => void
  onCreateConfirm: (data: StaffFormData) => void
  isSubmitting: boolean
}

export function StaffModals({
  staffToDelete,
  setStaffToDelete,
  onDeleteConfirm,
  isBulkDeleteOpen,
  setIsBulkDeleteOpen,
  onBulkDeleteConfirm,
  staffToEdit,
  setStaffToEdit,
  onEditConfirm,
  isCreateStaffOpen,
  setIsCreateStaffOpen,
  onCreateConfirm,
  isSubmitting,
}: StaffModalsProps) {
  return (
    <>
      {/* Delete Confirmation Dialog */}
      <AlertDialog
        open={!!staffToDelete}
        onOpenChange={() => setStaffToDelete(null)}
      >
        <AlertDialogContent>
          <AlertDialogHeader>
            <AlertDialogTitle>Delete Staff Member</AlertDialogTitle>
            <AlertDialogDescription>
              Are you sure you want to delete this staff member? This action
              cannot be undone.
            </AlertDialogDescription>
          </AlertDialogHeader>
          <AlertDialogFooter>
            <AlertDialogCancel>Cancel</AlertDialogCancel>
            <AlertDialogAction
              onClick={() => staffToDelete && onDeleteConfirm(staffToDelete)}
              className="bg-destructive text-destructive-foreground hover:bg-destructive/90"
            >
              Delete
            </AlertDialogAction>
          </AlertDialogFooter>
        </AlertDialogContent>
      </AlertDialog>

      {/* Bulk Delete Confirmation Dialog */}
      <AlertDialog
        open={isBulkDeleteOpen}
        onOpenChange={setIsBulkDeleteOpen}
      >
        <AlertDialogContent>
          <AlertDialogHeader>
            <AlertDialogTitle>Bulk Delete Staff</AlertDialogTitle>
            <AlertDialogDescription>
              Are you sure you want to delete the selected staff members? This
              action cannot be undone.
            </AlertDialogDescription>
          </AlertDialogHeader>
          <AlertDialogFooter>
            <AlertDialogCancel>Cancel</AlertDialogCancel>
            <AlertDialogAction
              onClick={onBulkDeleteConfirm}
              className="bg-destructive text-destructive-foreground hover:bg-destructive/90"
            >
              Delete All
            </AlertDialogAction>
          </AlertDialogFooter>
        </AlertDialogContent>
      </AlertDialog>

      {/* Edit Staff Dialog */}
      <StaffFormDialog
        open={!!staffToEdit}
        onOpenChange={() => setStaffToEdit(null)}
        staff={staffToEdit}
        onSubmit={onEditConfirm}
        isSubmitting={isSubmitting}
        mode="edit"
      />

      {/* Create Staff Dialog */}
      <StaffFormDialog
        open={isCreateStaffOpen}
        onOpenChange={setIsCreateStaffOpen}
        staff={null}
        onSubmit={onCreateConfirm}
        isSubmitting={isSubmitting}
        mode="create"
      />
    </>
  )
}

interface StaffFormDialogProps {
  open: boolean
  onOpenChange: (open: boolean) => void
  staff: Staff | null
  onSubmit: (data: StaffFormData) => void
  isSubmitting: boolean
  mode: 'create' | 'edit'
}

function StaffFormDialog({
  open,
  onOpenChange,
  staff,
  onSubmit,
  isSubmitting,
  mode,
}: StaffFormDialogProps) {
  const preload = React.useCallback(
    (form: UseFormReturn<CreateStaffValues, unknown, CreateStaffValues>) => {
      if (open && staff) {
        form.reset({
          name: staff.name,
          email: staff.email || '',
          phone: staff.phone || '',
          dob: staff.dob,
          gender: staff.gender,
          employee_id: staff.employee_id,
          staff_type: staff.staff_type,
          employment_status: staff.employment_status ?? '',
          address: staff.address || '',
          nic: staff.nic || '',
        })
      } else if (open) {
        form.reset({
          name: '',
          email: '',
          phone: '',
          dob: '',
          gender: 'Male',
          employee_id: '',
          staff_type: 'Teaching',
          employment_status: '',
          address: '',
          nic: '',
        })
      }
    },
    [staff, open],
  )

  const config = defineFormConfig(createStaffSchema, {
    structure: [
      [{ field: 'name', type: 'input', label: 'Full Name', placeholder: 'John Doe' }],
      [{ field: 'email', type: 'input', label: 'Email Address', placeholder: 'john@example.com', inputType: 'email' }],
      [{ field: 'phone', type: 'input', label: 'Phone', placeholder: '+1 234 567 8900' }],
      [{ field: 'dob', type: 'input', label: 'Date of Birth', inputType: 'date' }],
      [
        {
          field: 'gender',
          type: 'select',
          label: 'Gender',
          items: [...GENDERS],
          parse: (value) => value,
        },
      ],
      [{ field: 'employee_id', type: 'input', label: 'Employee ID', placeholder: 'EMP001' }],
      [
        {
          field: 'staff_type',
          type: 'select',
          label: 'Staff Type',
          items: [...STAFF_TYPES],
          parse: (value) => value,
        },
      ],
      [
        {
          field: 'employment_status',
          type: 'select',
          label: 'Employment Status',
          items: [{ label: 'Select status', value: '' }, ...EMPLOYMENT_STATUSES],
          parse: (value) => value,
        },
      ],
      [{ field: 'address', type: 'textarea', label: 'Address', rows: 3 }],
      [{ field: 'nic', type: 'input', label: 'NIC / Passport Number' }],
    ],
    extras: {
      bottom: (
        <div className="flex items-center justify-end gap-2 border-t border-border/40 bg-muted/20 px-6 py-4 mt-6 -mx-6 -mb-6">
          <Button
            type="button"
            variant="outline"
            onClick={() => onOpenChange(false)}
          >
            Cancel
          </Button>
          <Button type="submit" disabled={isSubmitting}>
            {isSubmitting && <Spinner className="mr-2 h-4 w-4" />}
            {mode === 'create' ? 'Create' : 'Save'}
          </Button>
        </div>
      ),
    },
  })

  return (
    <Dialog open={open} onOpenChange={onOpenChange}>
      <DialogContent className="max-w-lg p-0 overflow-hidden">
        <div className="flex flex-col border-b border-border/40 bg-muted/20 p-6 pb-6">
          <div className="flex gap-4 items-start">
            <div className="flex h-10 w-10 shrink-0 items-center justify-center rounded-full bg-primary/10 ring-1 ring-primary/20">
              <HugeiconsIcon
                icon={UserAdd01Icon}
                className="size-5 text-primary"
              />
            </div>
            <Stack gap={1} className="pt-1">
              <DialogTitle className="text-xl">
                {mode === 'create' ? 'Add Staff Member' : 'Edit Staff Member'}
              </DialogTitle>
              <DialogDescription className="text-sm">
                {mode === 'create'
                  ? 'Enter the staff member details below.'
                  : 'Update the staff member details.'}
              </DialogDescription>
            </Stack>
          </div>
        </div>

        <div className="px-6 pb-6 pt-4">
          <FormBuilder
            schema={createStaffSchema}
            config={config}
            onSubmit={(values) => onSubmit(values)}
            preload={preload}
            isLoading={isSubmitting}
            showErrorSummary={false}
            toastErrors={false}
            showSuccessAlert={false}
            actions={[]}
            className="space-y-4"
          />
        </div>
      </DialogContent>
    </Dialog>
  )
}
