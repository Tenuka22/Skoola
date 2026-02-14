'use client'

import { HugeiconsIcon } from '@hugeicons/react'
import { PencilEdit01Icon } from '@hugeicons/core-free-icons'
import { StudentForm } from './student-form'
import type { StudentResponse } from '../../../lib/api/types.gen'
import type { UpdateStudentRequest } from '../../../lib/api/types.gen'
import type { CreateStudentValues } from '../schemas'
import {
  Dialog,
  DialogContent,
  DialogDescription,
  DialogHeader,
  DialogTitle,
} from '../../../components/ui/dialog'

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
      <DialogContent className="max-w-2xl rounded-[2.5rem] border-none p-8 shadow-2xl backdrop-blur-3xl ring-1 ring-white/20">
        <DialogHeader className="mb-6">
          <div className="flex items-center gap-4">
            <div className="flex size-14 items-center justify-center rounded-2xl bg-primary/10 text-primary">
              <HugeiconsIcon icon={PencilEdit01Icon} className="size-7" />
            </div>
            <div>
              <DialogTitle className="text-2xl font-black tracking-tight">
                Edit Student Profile
              </DialogTitle>
              <DialogDescription className="font-medium">
                Update institutional records for {studentToEdit?.name_english}.
              </DialogDescription>
            </div>
          </div>
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