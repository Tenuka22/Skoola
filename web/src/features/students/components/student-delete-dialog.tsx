'use client'

import { useQuery } from '@tanstack/react-query'
import {
  AlertDialog,
  AlertDialogAction,
  AlertDialogCancel,
  AlertDialogContent,
  AlertDialogDescription,
  AlertDialogFooter,
  AlertDialogHeader,
  AlertDialogTitle,
} from '../../../components/ui/alert-dialog'

import { authClient } from '@/lib/clients'
import { getStudentByIdOptions } from '@/lib/api/@tanstack/react-query.gen'

interface StudentDeleteDialogProps {
  studentToDeleteId: string | null
  setStudentToDeleteId: (id: string | null) => void
  onDeleteConfirm: (id: string) => void
}

export function StudentDeleteDialog({
  studentToDeleteId,
  setStudentToDeleteId,
  onDeleteConfirm,
}: StudentDeleteDialogProps) {
  const studentQuery = useQuery({
    ...getStudentByIdOptions({
      client: authClient,
      path: { student_id: studentToDeleteId || '' },
    }),
    enabled: !!studentToDeleteId,
  })

  const student = studentQuery.data

  return (
    <AlertDialog
      open={!!studentToDeleteId}
      onOpenChange={(open) => !open && setStudentToDeleteId(null)}
    >
      <AlertDialogContent>
        <AlertDialogHeader>
          <AlertDialogTitle>Remove Student?</AlertDialogTitle>
          <AlertDialogDescription>
            This will permanently remove{' '}
            <strong>{student?.name_english}</strong> from the institution
            records. This action cannot be undone.
          </AlertDialogDescription>
        </AlertDialogHeader>
        <AlertDialogFooter>
          <AlertDialogCancel>Abort</AlertDialogCancel>
          <AlertDialogAction
            onClick={() =>
              studentToDeleteId && onDeleteConfirm(studentToDeleteId)
            }
          >
            Confirm Removal
          </AlertDialogAction>
        </AlertDialogFooter>
      </AlertDialogContent>
    </AlertDialog>
  )
}
