'use client'

import { HugeiconsIcon } from '@hugeicons/react'
import { Delete02Icon, PencilEdit01Icon, UserAdd01Icon } from '@hugeicons/core-free-icons'
import { StaffForm } from './staff-form'
import type { StaffResponse } from '@/lib/api/types.gen'
import type { CreateStaffValues } from '../schemas'
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
  DialogHeader,
  DialogTitle,
} from '@/components/ui/dialog'

interface StaffModalsProps {
  staffToDelete: StaffResponse | null
  setStaffToDelete: (staff: StaffResponse | null) => void
  onDeleteConfirm: (id: string) => void
  isAddOpen: boolean
  setIsAddOpen: (open: boolean) => void
  onAddConfirm: (values: CreateStaffValues) => void
  isAdding?: boolean
  staffToEdit: StaffResponse | null
  setStaffToEdit: (staff: StaffResponse | null) => void
  onEditConfirm: (values: CreateStaffValues) => void
  isEditing?: boolean
}

export function StaffModals({
  staffToDelete,
  setStaffToDelete,
  onDeleteConfirm,
  isAddOpen,
  setIsAddOpen,
  onAddConfirm,
  isAdding,
  staffToEdit,
  setStaffToEdit,
  onEditConfirm,
  isEditing,
}: StaffModalsProps) {
  return (
    <>
      {/* Delete Confirmation */}
      <AlertDialog
        open={!!staffToDelete}
        onOpenChange={(open) => !open && setStaffToDelete(null)}
      >
        <AlertDialogContent className="rounded-[2.5rem] border-none p-10 shadow-2xl backdrop-blur-3xl ring-1 ring-white/20">
          <AlertDialogHeader>
            <div className="mx-auto mb-4 flex size-20 items-center justify-center rounded-3xl bg-destructive/10 text-destructive">
              <HugeiconsIcon icon={Delete02Icon} className="size-10" />
            </div>
            <AlertDialogTitle className="text-center text-3xl font-black tracking-tight">
              Remove Staff?
            </AlertDialogTitle>
            <AlertDialogDescription className="text-center text-base font-medium leading-relaxed opacity-70">
              This will permanently remove <strong>{staffToDelete?.name}</strong> from the institution records. 
              This action cannot be undone.
            </AlertDialogDescription>
          </AlertDialogHeader>
          <AlertDialogFooter className="mt-10 sm:justify-center gap-3">
            <AlertDialogCancel className="h-14 min-w-[120px] rounded-2xl border-none bg-muted/50 font-black uppercase tracking-widest transition-colors hover:bg-muted">
              Abort
            </AlertDialogCancel>
            <AlertDialogAction
              onClick={() => staffToDelete && onDeleteConfirm(staffToDelete.id)}
              className="h-14 min-w-[160px] rounded-2xl bg-destructive font-black uppercase tracking-widest text-destructive-foreground shadow-2xl shadow-destructive/20 transition-all hover:bg-destructive/90 active:scale-95"
            >
              Confirm Removal
            </AlertDialogAction>
          </AlertDialogFooter>
        </AlertDialogContent>
      </AlertDialog>

      {/* Add Staff Dialog */}
      <Dialog open={isAddOpen} onOpenChange={setIsAddOpen}>
        <DialogContent className="max-w-2xl rounded-[2.5rem] border-none p-8 shadow-2xl backdrop-blur-3xl ring-1 ring-white/20">
          <DialogHeader className="mb-6">
            <div className="flex items-center gap-4">
              <div className="flex size-14 items-center justify-center rounded-2xl bg-primary/10 text-primary">
                <HugeiconsIcon icon={UserAdd01Icon} className="size-7" />
              </div>
              <div>
                <DialogTitle className="text-2xl font-black tracking-tight">Add New Employee</DialogTitle>
                <DialogDescription className="font-medium">Register a new member of your staff.</DialogDescription>
              </div>
            </div>
          </DialogHeader>
          <StaffForm onSubmit={onAddConfirm} isSubmitting={isAdding} submitLabel="Register Employee" />
        </DialogContent>
      </Dialog>

      {/* Edit Staff Dialog */}
      <Dialog open={!!staffToEdit} onOpenChange={(open) => !open && setStaffToEdit(null)}>
        <DialogContent className="max-w-2xl rounded-[2.5rem] border-none p-8 shadow-2xl backdrop-blur-3xl ring-1 ring-white/20">
          <DialogHeader className="mb-6">
            <div className="flex items-center gap-4">
              <div className="flex size-14 items-center justify-center rounded-2xl bg-primary/10 text-primary">
                <HugeiconsIcon icon={PencilEdit01Icon} className="size-7" />
              </div>
              <div>
                <DialogTitle className="text-2xl font-black tracking-tight">Edit Profile</DialogTitle>
                <DialogDescription className="font-medium">Update institutional records for {staffToEdit?.name}.</DialogDescription>
              </div>
            </div>
          </DialogHeader>
          {staffToEdit && (
            <StaffForm 
              initialValues={staffToEdit as any} 
              onSubmit={onEditConfirm} 
              isSubmitting={isEditing} 
              submitLabel="Update Records" 
            />
          )}
        </DialogContent>
      </Dialog>
    </>
  )
}
