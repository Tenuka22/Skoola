'use client'

import { HugeiconsIcon } from '@hugeicons/react'
import { Delete02Icon } from '@hugeicons/core-free-icons'

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
import { getStaffA2C17Fd0026652C749Fc88Fc4Fd7Fd58Options } from '@/lib/api/@tanstack/react-query.gen'
import { authClient } from '@/lib/clients'

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
    ...getStaffA2C17Fd0026652C749Fc88Fc4Fd7Fd58Options({
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
      <AlertDialogContent className="rounded-[2.5rem] border-none p-10 shadow-2xl backdrop-blur-3xl ring-1 ring-white/20">
        <AlertDialogHeader>
          <div className="mx-auto mb-4 flex size-20 items-center justify-center rounded-3xl bg-destructive/10 text-destructive">
            <HugeiconsIcon icon={Delete02Icon} className="size-10" />
          </div>
          <AlertDialogTitle className="text-center text-3xl font-black tracking-tight">
            Remove Staff?
          </AlertDialogTitle>
          <AlertDialogDescription className="text-center text-base font-medium leading-relaxed opacity-70">
            This will permanently remove <strong>{staff?.name}</strong> from the
            institution records. This action cannot be undone.
          </AlertDialogDescription>
        </AlertDialogHeader>
        <AlertDialogFooter className="mt-10 sm:justify-center gap-3">
          <AlertDialogCancel className="h-14 min-w-[120px] rounded-2xl border-none bg-muted/50 font-black uppercase tracking-widest transition-colors hover:bg-muted">
            Abort
          </AlertDialogCancel>
          <AlertDialogAction
            onClick={() => staffToDeleteId && onDeleteConfirm(staffToDeleteId)}
            className="h-14 min-w-[160px] rounded-2xl bg-destructive font-black uppercase tracking-widest text-destructive-foreground shadow-2xl shadow-destructive/20 transition-all hover:bg-destructive/90 active:scale-95"
          >
            Confirm Removal
          </AlertDialogAction>
        </AlertDialogFooter>
      </AlertDialogContent>
    </AlertDialog>
  )
}
