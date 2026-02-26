'use client'

import {
  Dialog,
  DialogContent,
  DialogDescription,
  DialogHeader,
  DialogTitle,
} from '../../../components/ui/dialog'
import { StaffForm } from './staff-form'
import type { StaffFormValues } from '../schemas'

interface StaffAddDialogProps {
  isAddOpen: boolean
  setIsAddOpen: (open: boolean) => void
  onAddConfirm: (values: StaffFormValues) => void
  isAdding?: boolean
}

export function StaffAddDialog({
  isAddOpen,
  setIsAddOpen,
  onAddConfirm,
  isAdding,
}: StaffAddDialogProps) {
  return (
    <Dialog open={isAddOpen} onOpenChange={setIsAddOpen}>
      <DialogContent>
        <DialogHeader>
          <DialogTitle>Add New Employee</DialogTitle>
          <DialogDescription>
            Register a new member of your staff.
          </DialogDescription>
        </DialogHeader>
        <StaffForm
          onSubmit={onAddConfirm}
          isSubmitting={isAdding}
          submitLabel="Register Employee"
        />
      </DialogContent>
    </Dialog>
  )
}
