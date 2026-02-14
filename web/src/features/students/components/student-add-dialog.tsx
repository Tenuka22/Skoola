'use client'

import { HugeiconsIcon } from '@hugeicons/react'
import { UserAdd01Icon } from '@hugeicons/core-free-icons'
import { StudentForm } from './student-form'
import type { CreateStudentValues } from '../schemas'
import {
  Dialog,
  DialogContent,
  DialogDescription,
  DialogHeader,
  DialogTitle,
} from '../../../components/ui/dialog'

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
      <DialogContent className="max-w-2xl rounded-[2.5rem] border-none p-8 shadow-2xl backdrop-blur-3xl ring-1 ring-white/20">
        <DialogHeader className="mb-6">
          <div className="flex items-center gap-4">
            <div className="flex size-14 items-center justify-center rounded-2xl bg-primary/10 text-primary">
              <HugeiconsIcon icon={UserAdd01Icon} className="size-7" />
            </div>
            <div>
              <DialogTitle className="text-2xl font-black tracking-tight">
                Add New Student
              </DialogTitle>
              <DialogDescription className="font-medium">
                Register a new student in your institution.
              </DialogDescription>
            </div>
          </div>
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