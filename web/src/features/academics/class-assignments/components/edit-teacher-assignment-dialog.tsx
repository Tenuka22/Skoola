import { HugeiconsIcon } from '@hugeicons/react'
import { Tick01Icon } from '@hugeicons/core-free-icons'
import { useForm } from 'react-hook-form'
import { zodResolver } from '@hookform/resolvers/zod'
import { useEffect } from 'react'
import { useQuery } from '@tanstack/react-query'
import { updateTeacherAssignmentFormSchema } from '../schemas'
import type { ClassAssignmentRow } from './class-assignments-table-columns'
import type { UpdateTeacherAssignmentFormValues } from '../schemas'
import { Button } from '@/components/ui/button'
import {
  Dialog,
  DialogContent,
  DialogFooter,
  DialogHeader,
  DialogTitle,
} from '@/components/ui/dialog'
import { Label } from '@/components/ui/label'
import { Spinner } from '@/components/ui/spinner'
import {
  Select,
  SelectContent,
  SelectItem,
  SelectTrigger,
  SelectValue,
} from '@/components/ui/select'
import { authClient } from '@/lib/clients'
import { getAllStaffOptions } from '@/lib/api/@tanstack/react-query.gen'

interface EditTeacherAssignmentDialogProps {
  assignment: ClassAssignmentRow | null
  open: boolean
  onOpenChange: (open: boolean) => void
  onConfirm: (data: UpdateTeacherAssignmentFormValues) => void
  isSubmitting?: boolean
}

export function EditTeacherAssignmentDialog({
  assignment,
  open,
  onOpenChange,
  onConfirm,
  isSubmitting,
}: EditTeacherAssignmentDialogProps) {
  const form = useForm<UpdateTeacherAssignmentFormValues>({
    resolver: zodResolver(updateTeacherAssignmentFormSchema),
    defaultValues: {
      teacher_id: '',
    },
  })

  const { data: staffData } = useQuery(
    getAllStaffOptions({ client: authClient }),
  )
  const staff = staffData?.data || []

  useEffect(() => {
    if (assignment) {
      form.reset({
        teacher_id: assignment.teacher_id,
      })
    }
  }, [assignment, form])

  const handleSubmit = (data: UpdateTeacherAssignmentFormValues) => {
    onConfirm(data)
  }

  return (
    <Dialog
      open={open}
      onOpenChange={(val) => {
        if (!val) form.reset()
        onOpenChange(val)
      }}
    >
      <DialogContent>
        <DialogHeader>
          <DialogTitle>Edit Teacher Assignment</DialogTitle>
        </DialogHeader>
        <form
          onSubmit={form.handleSubmit(handleSubmit)}
          className="grid gap-4 py-4"
        >
          <p className="text-sm text-muted-foreground">
            Update the teacher for the assignment:
            <span className="font-medium text-foreground ml-1">
              Class: {assignment?.class_id}, Subject: {assignment?.subject_id}
            </span>
          </p>
          <div className="grid grid-cols-4 items-center gap-4">
            <Label htmlFor="teacher_id" className="text-right">
              Teacher
            </Label>
            <Select
              onValueChange={(value) =>
                form.setValue('teacher_id', value || '')
              }
              value={form.watch('teacher_id')}
            >
              <SelectTrigger id="teacher_id" className="col-span-3">
                <SelectValue placeholder="Select a teacher" />
              </SelectTrigger>
              <SelectContent>
                {staff.map((teacher) => (
                  <SelectItem key={teacher.id} value={teacher.id}>
                    {teacher.name}
                  </SelectItem>
                ))}
              </SelectContent>
            </Select>
            {form.formState.errors.teacher_id && (
              <p className="col-span-4 col-start-2 text-sm font-medium text-red-500">
                {form.formState.errors.teacher_id.message}
              </p>
            )}
          </div>
          <DialogFooter className="mt-4">
            <Button
              type="button"
              variant="ghost"
              onClick={() => onOpenChange(false)}
            >
              Cancel
            </Button>
            <Button type="submit" disabled={isSubmitting}>
              {isSubmitting ? (
                <Spinner className="mr-2" />
              ) : (
                <HugeiconsIcon icon={Tick01Icon} className="size-4 mr-2" />
              )}
              Save Changes
            </Button>
          </DialogFooter>
        </form>
      </DialogContent>
    </Dialog>
  )
}
