'use client'

import {
  Dialog,
  DialogContent,
  DialogDescription,
  DialogHeader,
  DialogTitle,
} from '../../../components/ui/dialog'
import { StudentForm } from './student-form'
import type { CreateStudentValues } from '../schemas'

interface StudentAddDialogProps {
  isAddOpen: boolean
  setIsAddOpen: (open: boolean) => void
  onAddConfirm: (values: CreateStudentValues) => void
  isAdding?: boolean
}

export function StudentAddDialog({
  isAddOpen,
  setIsAddOpen,
  onAddConfirm,
  isAdding,
}: StudentAddDialogProps) {
  return (
    <Dialog open={isAddOpen} onOpenChange={setIsAddOpen}>
      <DialogContent>
        <DialogHeader>
          <DialogTitle>Add New Student</DialogTitle>
          <DialogDescription>
            Register a new student in your institution.
          </DialogDescription>
        </DialogHeader>
        <StudentForm
          onSubmit={onAddConfirm}
          isSubmitting={isAdding}
          submitLabel="Register Student"
        />
      </DialogContent>
    </Dialog>
  )
}
