'use client'

import { HugeiconsIcon } from '@hugeicons/react'
import { PencilEdit01Icon } from '@hugeicons/core-free-icons'
import { StaffForm } from './staff-form'
import type { StaffResponse } from '../../../lib/api/types.gen'
import type { UpdateStaffRequest } from '../../../lib/api/types.gen'

import { mapStaffResponseToCreateStaffValues } from '../utils/staff-mappers'
import {
  Dialog,
  DialogContent,
  DialogDescription,
  DialogHeader,
  DialogTitle,
} from '../../../components/ui/dialog'

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
      <DialogContent className="max-w-2xl rounded-[2.5rem] border-none p-8 shadow-2xl backdrop-blur-3xl ring-1 ring-white/20">
        <DialogHeader className="mb-6">
          <div className="flex items-center gap-4">
            <div className="flex size-14 items-center justify-center rounded-2xl bg-primary/10 text-primary">
              <HugeiconsIcon icon={PencilEdit01Icon} className="size-7" />
            </div>
            <div>
              <DialogTitle className="text-2xl font-black tracking-tight">
                Edit Profile
              </DialogTitle>
              <DialogDescription className="font-medium">
                Update institutional records for {staffToEdit?.name}.
              </DialogDescription>
            </div>
          </div>
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