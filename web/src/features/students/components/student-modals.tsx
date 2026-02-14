import { StudentBulkDeleteDialog } from './student-bulk-delete-dialog'
import { StudentEditDialog } from './student-edit-dialog'
import { StudentDeleteDialog } from './student-delete-dialog'
import type { StudentResponse, UpdateStudentRequest } from '@/lib/api/types.gen'

interface StudentModalsProps {
  studentToDelete: string | null
  setStudentToDelete: (id: string | null) => void
  onDeleteConfirm: (id: string) => void
  isBulkDeleteOpen: boolean
  setIsBulkDeleteOpen: (open: boolean) => void
  onBulkDeleteConfirm: () => void
  selectedCount: number
  studentToEdit: StudentResponse | null
  setStudentToEdit: (student: StudentResponse | null) => void
  onEditConfirm: (values: UpdateStudentRequest) => void
  isEditing?: boolean
}

export function StudentModals({
  studentToDelete,
  setStudentToDelete,
  onDeleteConfirm,
  isBulkDeleteOpen,
  setIsBulkDeleteOpen,
  onBulkDeleteConfirm,
  selectedCount,
  studentToEdit,
  setStudentToEdit,
  onEditConfirm,
  isEditing,
}: StudentModalsProps) {
  return (
    <>
      <StudentDeleteDialog
        studentToDeleteId={studentToDelete}
        setStudentToDeleteId={setStudentToDelete}
        onDeleteConfirm={onDeleteConfirm}
      />

      <StudentBulkDeleteDialog
        isBulkDeleteOpen={isBulkDeleteOpen}
        setIsBulkDeleteOpen={setIsBulkDeleteOpen}
        onBulkDeleteConfirm={onBulkDeleteConfirm}
        selectedCount={selectedCount}
      />

      <StudentEditDialog
        studentToEdit={studentToEdit}
        setStudentToEdit={setStudentToEdit}
        onEditConfirm={onEditConfirm}
        isEditing={isEditing}
      />
    </>
  )
}
