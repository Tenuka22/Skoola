import { HugeiconsIcon } from '@hugeicons/react'
import { Tick01Icon } from '@hugeicons/core-free-icons'
import { useForm } from 'react-hook-form'
import { zodResolver } from '@hookform/resolvers/zod'
import { useQuery } from '@tanstack/react-query'
import { assignSubjectToGradeSchema } from '../schemas'
import type { SubjectResponse } from '@/lib/api/types.gen'
import type { z } from 'zod'
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
import { getAllGradeLevelsOptions } from '@/lib/api/@tanstack/react-query.gen'

type FormValues = z.infer<typeof assignSubjectToGradeSchema>

interface SubjectAssignToGradeDialogProps {
  subject: SubjectResponse | null
  open: boolean
  onOpenChange: (open: boolean) => void
  onConfirm: (gradeId: string) => void
  isSubmitting?: boolean
}

export function SubjectAssignToGradeDialog({
  subject,
  open,
  onOpenChange,
  onConfirm,
  isSubmitting,
}: SubjectAssignToGradeDialogProps) {
  const form = useForm<FormValues>({
    resolver: zodResolver(assignSubjectToGradeSchema),
    defaultValues: {
      grade_id: '',
      subject_id: subject?.id || '',
    },
  })

  const { data: gradeLevelsData } = useQuery(
    getAllGradeLevelsOptions({ client: authClient }),
  )
  const gradeLevels = gradeLevelsData?.data || []

  const handleSubmit = (data: FormValues) => {
    onConfirm(data.grade_id)
  }

  return (
    <Dialog
      open={open}
      onOpenChange={(val) => {
        if (!val) form.reset()
        if (val && subject) form.setValue('subject_id', subject.id)
        onOpenChange(val)
      }}
    >
      <DialogContent>
        <DialogHeader>
          <DialogTitle>Assign Subject to Grade Level</DialogTitle>
        </DialogHeader>
        <form
          onSubmit={form.handleSubmit(handleSubmit)}
          className="grid gap-4 py-4"
        >
          <p className="text-sm text-muted-foreground">
            Assign{' '}
            <span className="font-medium text-foreground">
              {subject?.subject_name_en}
            </span>{' '}
            to a specific grade level.
          </p>
          <div className="grid grid-cols-4 items-center gap-4">
            <Label htmlFor="grade_id" className="text-right">
              Grade Level
            </Label>
            <Select
              onValueChange={(value) => form.setValue('grade_id', value || '')}
              value={form.watch('grade_id')}
            >
              <SelectTrigger className="col-span-3">
                <SelectValue placeholder="Select a grade level" />
              </SelectTrigger>
              <SelectContent>
                {gradeLevels.map((gl) => (
                  <SelectItem key={gl.id} value={gl.id}>
                    {gl.grade_name}
                  </SelectItem>
                ))}
              </SelectContent>
            </Select>
            {form.formState.errors.grade_id && (
              <p className="col-span-4 col-start-2 text-sm font-medium text-red-500">
                {form.formState.errors.grade_id.message}
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
              Assign
            </Button>
          </DialogFooter>
        </form>
      </DialogContent>
    </Dialog>
  )
}
