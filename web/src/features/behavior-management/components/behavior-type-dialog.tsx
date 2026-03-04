import { behaviorIncidentTypeSchema } from '../schemas'
import type { BehaviorIncidentTypeFormValues } from '../schemas'
import type { BehaviorIncidentTypeResponse } from '@/lib/api/types.gen'
import {
  Dialog,
  DialogContent,
  DialogDescription,
  DialogFooter,
  DialogHeader,
  DialogTitle,
} from '@/components/ui/dialog'
import { Button } from '@/components/ui/button'
import { FormBuilder, defineFormConfig } from '@/components/form-builder'
import { Spinner } from '@/components/ui/spinner'

interface BehaviorTypeDialogProps {
  type?: BehaviorIncidentTypeResponse | null
  open: boolean
  onOpenChange: (open: boolean) => void
  onConfirm: (data: BehaviorIncidentTypeFormValues) => void
  isSubmitting?: boolean
}

export function BehaviorTypeDialog({
  type,
  open,
  onOpenChange,
  onConfirm,
  isSubmitting,
}: BehaviorTypeDialogProps) {
  const onSubmit = (data: BehaviorIncidentTypeFormValues) => {
    onConfirm(data)
  }

  const config = defineFormConfig(behaviorIncidentTypeSchema, {
    structure: [
      [
        {
          field: 'type_name',
          type: 'input',
          label: 'Type Name',
          placeholder: 'e.g. Late Arrival',
          parse: (value: string) => value,
        },
      ],
      [
        {
          field: 'description',
          type: 'textarea',
          label: 'Description',
          placeholder: 'Describe this behavior type...',
          parse: (value: string) => value,
        },
      ],
      [
        {
          field: 'default_points',
          type: 'input',
          inputType: 'number',
          label: 'Default Points',
          parse: (value: string) => parseInt(value, 10),
        },
      ],
    ],
    extras: {
      bottom: (
        <DialogFooter>
          <Button
            type="button"
            variant="outline"
            onClick={() => onOpenChange(false)}
          >
            Cancel
          </Button>
          <Button type="submit" disabled={isSubmitting}>
            {isSubmitting ? <Spinner className="mr-2" /> : null}
            {type ? 'Update' : 'Create'}
          </Button>
        </DialogFooter>
      ),
    },
  })

  return (
    <Dialog open={open} onOpenChange={onOpenChange}>
      <DialogContent className="sm:max-w-[425px]">
        <DialogHeader>
          <DialogTitle>{type ? 'Edit' : 'Add'} Behavior Type</DialogTitle>
          <DialogDescription>
            {type
              ? 'Update the behavior incident type details.'
              : 'Create a new type of behavior incident.'}
          </DialogDescription>
        </DialogHeader>
        <FormBuilder
          schema={behaviorIncidentTypeSchema}
          config={config}
          defaultValues={{
            type_name: type?.type_name || '',
            description: type?.description || '',
            default_points: type?.default_points || 0,
          }}
          onSubmit={(values) => onSubmit(values)}
          preload={(form) => {
            if (open) {
              if (type) {
                form.reset({
                  type_name: type.type_name,
                  description: type.description || '',
                  default_points: type.default_points,
                })
              } else {
                form.reset({
                  type_name: '',
                  description: '',
                  default_points: 0,
                })
              }
            }
          }}
          isLoading={isSubmitting}
          showErrorSummary={false}
          toastErrors={false}
          showSuccessAlert={false}
          actions={[]}
          className="space-y-4"
        />
      </DialogContent>
    </Dialog>
  )
}
