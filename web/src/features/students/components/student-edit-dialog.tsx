'use client'

import {
  Dialog,
  DialogContent,
  DialogDescription,
  DialogHeader,
  DialogTitle,
} from '../../../components/ui/dialog'
import { StudentForm } from './student-form'
import type {
  StudentResponse,
  UpdateStudentRequest,
} from '../../../lib/api/types.gen'
import type { CreateStudentValues } from '../schemas'

interface StudentEditDialogProps {
  studentToEdit: StudentResponse | null
  setStudentToEdit: (student: StudentResponse | null) => void
  onEditConfirm: (values: UpdateStudentRequest) => void
  isEditing?: boolean
}

export function StudentEditDialog({
  studentToEdit,
  setStudentToEdit,
  onEditConfirm,
  isEditing,
}: StudentEditDialogProps) {
  return (
    <Dialog
      open={!!studentToEdit}
      onOpenChange={(open) => !open && setStudentToEdit(null)}
    >
      <DialogContent>
        <DialogHeader>
          <DialogTitle>Edit Student Profile</DialogTitle>
          <DialogDescription>
            Update institutional records for {studentToEdit?.name_english}.
          </DialogDescription>
        </DialogHeader>
        {studentToEdit && (
          <StudentForm
            initialValues={studentToEdit as Partial<CreateStudentValues>}
            onSubmit={onEditConfirm}
            isSubmitting={isEditing}
            submitLabel="Update Records"
          />
        )}
      </DialogContent>
    </Dialog>
  )
}
