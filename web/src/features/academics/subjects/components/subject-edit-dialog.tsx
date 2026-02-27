import { HugeiconsIcon } from '@hugeicons/react'
import { FloppyDiskIcon } from '@hugeicons/core-free-icons'
import * as React from 'react'
import { subjectFormSchema } from '../schemas'
import type { SubjectResponse } from '@/lib/api/types.gen'
import type { SubjectFormValues } from '../schemas'
import type { UseFormReturn } from 'react-hook-form'
import { Button } from '@/components/ui/button'
import {
  Dialog,
  DialogContent,
  DialogFooter,
  DialogHeader,
  DialogTitle,
} from '@/components/ui/dialog'
import { Input } from '@/components/ui/input'
import { Label } from '@/components/ui/label'
import { Spinner } from '@/components/ui/spinner'
import { Checkbox } from '@/components/ui/checkbox'
import { FormBuilder, defineFormConfig } from '@/components/form-builder'

interface SubjectEditDialogProps {
  subject: SubjectResponse | null
  open: boolean
  onOpenChange: (open: boolean) => void
  onConfirm: (data: SubjectFormValues) => void
  isSubmitting?: boolean
}

export function SubjectEditDialog({
  subject,
  open,
  onOpenChange,
  onConfirm,
  isSubmitting,
}: SubjectEditDialogProps) {
  const preload = React.useCallback(
    (form: UseFormReturn<SubjectFormValues, unknown, SubjectFormValues>) => {
      if (subject) {
        form.reset({
          id: subject.id,
          subject_code: subject.subject_code,
          subject_name_en: subject.subject_name_en,
          is_core: subject.is_core,
          subject_name_si: subject.subject_name_si || '',
          subject_name_ta: subject.subject_name_ta || '',
        })
      } else if (!open) {
        form.reset()
      }
    },
    [subject, open],
  )

  const config = defineFormConfig(subjectFormSchema, {
    structure: [],
    extras: {
      top: (form) => (
        <>
          <div className="grid grid-cols-4 items-center gap-4">
            <Label htmlFor="id" className="text-right">
              ID
            </Label>
            <Input
              id="id"
              {...form.register('id')}
              disabled
              className="col-span-3"
            />
          </div>
          <div className="grid grid-cols-4 items-center gap-4">
            <Label htmlFor="subject_code" className="text-right">
              Code
            </Label>
            <Input
              id="subject_code"
              {...form.register('subject_code')}
              className="col-span-3"
            />
            {form.formState.errors.subject_code && (
              <p className="col-span-4 col-start-2 text-sm font-medium text-red-500">
                {form.formState.errors.subject_code.message}
              </p>
            )}
          </div>
          <div className="grid grid-cols-4 items-center gap-4">
            <Label htmlFor="subject_name_en" className="text-right">
              Name (EN)
            </Label>
            <Input
              id="subject_name_en"
              {...form.register('subject_name_en')}
              className="col-span-3"
            />
            {form.formState.errors.subject_name_en && (
              <p className="col-span-4 col-start-2 text-sm font-medium text-red-500">
                {form.formState.errors.subject_name_en.message}
              </p>
            )}
          </div>
          <div className="grid grid-cols-4 items-center gap-4">
            <Label htmlFor="subject_name_si" className="text-right text-xs">
              Name (SI)
            </Label>
            <Input
              id="subject_name_si"
              {...form.register('subject_name_si')}
              className="col-span-3"
            />
          </div>
          <div className="grid grid-cols-4 items-center gap-4">
            <Label htmlFor="subject_name_ta" className="text-right text-xs">
              Name (TA)
            </Label>
            <Input
              id="subject_name_ta"
              {...form.register('subject_name_ta')}
              className="col-span-3"
            />
          </div>
          <div className="flex items-center space-x-2 pl-[120px]">
            <Checkbox
              id="is_core"
              checked={form.watch('is_core') === true}
              onCheckedChange={(checked) =>
                form.setValue('is_core', checked === true)
              }
            />
            <Label htmlFor="is_core">Is Core Subject</Label>
          </div>
        </>
      ),
      bottom: (
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
              <HugeiconsIcon icon={FloppyDiskIcon} className="size-4 mr-2" />
            )}
            Save Changes
          </Button>
        </DialogFooter>
      ),
    },
  })

  return (
    <Dialog open={open} onOpenChange={(val) => onOpenChange(val)}>
      <DialogContent className="max-w-md">
        <DialogHeader>
          <DialogTitle>Edit Subject</DialogTitle>
        </DialogHeader>
        <FormBuilder
          schema={subjectFormSchema}
          config={config}
          defaultValues={{
            id: '',
            subject_code: '',
            subject_name_en: '',
            is_core: true,
            subject_name_si: '',
            subject_name_ta: '',
          }}
          onSubmit={(values) => onConfirm(values)}
          preload={preload}
          isLoading={isSubmitting}
          showErrorSummary={false}
          toastErrors={false}
          showSuccessAlert={false}
          actions={[]}
          className="grid gap-4 py-4"
        />
      </DialogContent>
    </Dialog>
  )
}
