import { behaviorIncidentTypeSchema } from '../schemas'
import type { BehaviorIncidentTypeFormValues } from '../schemas'
import type { BehaviorIncidentTypeResponse } from '@/lib/api/types.gen'
import {
  Dialog,
  DialogContent,
  DialogDescription,
  DialogHeader,
  DialogTitle,
} from '@/components/ui/dialog'
import { FormBuilder } from '@/components/form-builder'

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
          defaultValues={{
            type_name: type?.type_name || '',
            description: type?.description || '',
            default_points: type?.default_points || 0,
          }}
          onSubmit={(values) => onConfirm(values as BehaviorIncidentTypeFormValues)}
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
          config={{
            structure: [
              [
                {
                  field: 'type_name',
                  type: 'input',
                  label: 'Type Name',
                  placeholder: 'e.g. Late Arrival',
                },
              ],
              [
                {
                  field: 'description',
                  type: 'textarea',
                  label: 'Description',
                  placeholder: 'Describe this behavior type...',
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
          }}
          actions={[
            {
              label: 'Cancel',
              onClick: () => onOpenChange(false),
              variant: 'outline',
            },
            {
              label: type ? 'Update' : 'Create',
              type: 'submit',
              variant: 'default',
              loading: isSubmitting,
            },
          ]}
          className="py-4"
        />
      </DialogContent>
    </Dialog>
  )
}
