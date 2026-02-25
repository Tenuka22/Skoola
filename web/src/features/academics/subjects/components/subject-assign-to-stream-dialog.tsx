import { HugeiconsIcon } from '@hugeicons/react'
import { Tick01Icon } from '@hugeicons/core-free-icons'
import { useForm } from 'react-hook-form'
import { zodResolver } from '@hookform/resolvers/zod'
import { assignSubjectToStreamSchema } from '../schemas'
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

type FormValues = z.infer<typeof assignSubjectToStreamSchema>

interface SubjectAssignToStreamDialogProps {
  subject: SubjectResponse | null
  open: boolean
  onOpenChange: (open: boolean) => void
  onConfirm: (streamId: string) => void
  isSubmitting?: boolean
}

export function SubjectAssignToStreamDialog({
  subject,
  open,
  onOpenChange,
  onConfirm,
  isSubmitting,
}: SubjectAssignToStreamDialogProps) {
  const form = useForm<FormValues>({
    resolver: zodResolver(assignSubjectToStreamSchema),
    defaultValues: {
      stream_id: '',
      subject_id: subject?.id || '',
    },
  })

  // TODO: Fetch streams from backend once implemented
  const streams = [
    { id: 'stream-1', name: 'Science' },
    { id: 'stream-2', name: 'Arts' },
    { id: 'stream-3', name: 'Commerce' },
  ]

  const handleSubmit = (data: FormValues) => {
    onConfirm(data.stream_id)
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
          <DialogTitle>Assign Subject to Stream</DialogTitle>
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
            to a specific stream.
          </p>
          <div className="grid grid-cols-4 items-center gap-4">
            <Label htmlFor="stream_id" className="text-right">
              Stream
            </Label>
            <Select
              onValueChange={(value) => form.setValue('stream_id', value || '')}
              value={form.watch('stream_id')}
            >
              <SelectTrigger className="col-span-3">
                <SelectValue placeholder="Select a stream" />
              </SelectTrigger>
              <SelectContent>
                {streams.map((stream) => (
                  <SelectItem key={stream.id} value={stream.id}>
                    {stream.name}
                  </SelectItem>
                ))}
              </SelectContent>
            </Select>
            {form.formState.errors.stream_id && (
              <p className="col-span-4 col-start-2 text-sm font-medium text-red-500">
                {form.formState.errors.stream_id.message}
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
