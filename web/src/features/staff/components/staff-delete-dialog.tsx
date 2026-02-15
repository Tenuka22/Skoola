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
import { getStaffByIdOptions } from '@/lib/api/@tanstack/react-query.gen'

interface StaffDeleteDialogProps {
  staffToDeleteId: string | null
  setStaffToDeleteId: (id: string | null) => void
  onDeleteConfirm: (id: string) => void
}

export function StaffDeleteDialog({
  staffToDeleteId,
  setStaffToDeleteId,
  onDeleteConfirm,
}: StaffDeleteDialogProps) {
  const staffQuery = useQuery({
    ...getStaffByIdOptions({
      client: authClient,
      path: { staff_id: staffToDeleteId || '' },
    }),
    enabled: !!staffToDeleteId,
  })

  const staff = staffQuery.data

  return (
    <AlertDialog
      open={!!staffToDeleteId}
      onOpenChange={(open) => !open && setStaffToDeleteId(null)}
    >
      <AlertDialogContent>
        <AlertDialogHeader>
          <AlertDialogTitle>Remove Staff?</AlertDialogTitle>
          <AlertDialogDescription>
            This will permanently remove <strong>{staff?.name}</strong> from the
            institution records. This action cannot be undone.
          </AlertDialogDescription>
        </AlertDialogHeader>
        <AlertDialogFooter>
          <AlertDialogCancel>Abort</AlertDialogCancel>
          <AlertDialogAction
            onClick={() => staffToDeleteId && onDeleteConfirm(staffToDeleteId)}
          >
            Confirm Removal
          </AlertDialogAction>
        </AlertDialogFooter>
      </AlertDialogContent>
    </AlertDialog>
  )
}
