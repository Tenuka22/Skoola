'use client'

import { mapStaffResponseToCreateStaffValues } from '../utils/staff-mappers'
import {
  Dialog,
  DialogContent,
  DialogDescription,
  DialogHeader,
  DialogTitle,
} from '../../../components/ui/dialog'
import { StaffForm } from './staff-form'
import type {
  StaffResponse,
  UpdateStaffRequest,
} from '../../../lib/api/types.gen'

interface StaffEditDialogProps {
  staffToEdit: StaffResponse | null
  setStaffToEdit: (staff: StaffResponse | null) => void
  onEditConfirm: (values: UpdateStaffRequest) => void
  isEditing?: boolean
}

export function StaffEditDialog({
  staffToEdit,
  setStaffToEdit,
  onEditConfirm,
  isEditing,
}: StaffEditDialogProps) {
  return (
    <Dialog
      open={!!staffToEdit}
      onOpenChange={(open) => !open && setStaffToEdit(null)}
    >
      <DialogContent>
        <DialogHeader>
          <DialogTitle>Edit Profile</DialogTitle>
          <DialogDescription>
            Update institutional records for {staffToEdit?.name}.
          </DialogDescription>
        </DialogHeader>
        {staffToEdit && (
          <StaffForm
            initialValues={mapStaffResponseToCreateStaffValues(staffToEdit)}
            onSubmit={onEditConfirm}
            isSubmitting={isEditing}
            submitLabel="Update Records"
          />
        )}
      </DialogContent>
    </Dialog>
  )
}
