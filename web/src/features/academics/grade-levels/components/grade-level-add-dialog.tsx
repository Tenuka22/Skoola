import { HugeiconsIcon } from '@hugeicons/react'
import { FloppyDiskIcon } from '@hugeicons/core-free-icons'
import * as React from 'react'
import { gradeLevelFormSchema } from '../schemas'
import type { GradeLevelFormValues } from '../schemas'
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
import {
  Select,
  SelectContent,
  SelectItem,
  SelectTrigger,
  SelectValue,
} from '@/components/ui/select'
import { zEducationLevel } from '@/lib/api/zod.gen'
import { FormBuilder, defineFormConfig } from '@/components/form-builder'

interface GradeLevelAddDialogProps {
  open: boolean
  onOpenChange: (open: boolean) => void
  onConfirm: (data: GradeLevelFormValues) => void
  isSubmitting?: boolean
}

const educationLevels = zEducationLevel.options

export function GradeLevelAddDialog({
  open,
  onOpenChange,
  onConfirm,
  isSubmitting,
}: GradeLevelAddDialogProps) {
  const preload = React.useCallback(
    (
      form: UseFormReturn<GradeLevelFormValues, unknown, GradeLevelFormValues>,
    ) => {
      if (!open) {
        form.reset()
      }
    },
    [open],
  )

  const config = defineFormConfig(gradeLevelFormSchema, {
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
              placeholder="e.g. GRADE-1"
              className="col-span-3"
            />
            {form.formState.errors.id && (
              <p className="col-span-4 col-start-2 text-sm font-medium text-red-500">
                {form.formState.errors.id.message}
              </p>
            )}
          </div>
          <div className="grid grid-cols-4 items-center gap-4">
            <Label htmlFor="grade_name" className="text-right">
              Grade Name
            </Label>
            <Input
              id="grade_name"
              {...form.register('grade_name')}
              placeholder="e.g. Grade 1"
              className="col-span-3"
            />
            {form.formState.errors.grade_name && (
              <p className="col-span-4 col-start-2 text-sm font-medium text-red-500">
                {form.formState.errors.grade_name.message}
              </p>
            )}
          </div>
          <div className="grid grid-cols-4 items-center gap-4">
            <Label htmlFor="grade_number" className="text-right">
              Grade Number
            </Label>
            <Input
              id="grade_number"
              type="number"
              {...form.register('grade_number', { valueAsNumber: true })}
              className="col-span-3"
            />
            {form.formState.errors.grade_number && (
              <p className="col-span-4 col-start-2 text-sm font-medium text-red-500">
                {form.formState.errors.grade_number.message}
              </p>
            )}
          </div>
          <div className="grid grid-cols-4 items-center gap-4">
            <Label htmlFor="education_level" className="text-right">
              Education Level
            </Label>
            <Select
              onValueChange={(val) =>
                form.setValue('education_level', val ?? 'Primary')
              }
              value={form.watch('education_level')}
            >
              <SelectTrigger className="col-span-3">
                <SelectValue placeholder="Select level" />
              </SelectTrigger>
              <SelectContent>
                {educationLevels.map((level) => (
                  <SelectItem key={level} value={level}>
                    {level}
                  </SelectItem>
                ))}
              </SelectContent>
            </Select>
            {form.formState.errors.education_level && (
              <p className="col-span-4 col-start-2 text-sm font-medium text-red-500">
                {form.formState.errors.education_level.message}
              </p>
            )}
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
            Add Grade Level
          </Button>
        </DialogFooter>
      ),
    },
  })

  return (
    <Dialog open={open} onOpenChange={(val) => onOpenChange(val)}>
      <DialogContent>
        <DialogHeader>
          <DialogTitle>Add New Grade Level</DialogTitle>
        </DialogHeader>
        <FormBuilder
          schema={gradeLevelFormSchema}
          config={config}
          defaultValues={{
            id: '',
            grade_name: '',
            grade_number: 1,
            education_level: 'Primary',
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
