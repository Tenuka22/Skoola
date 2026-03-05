import { useQuery } from '@tanstack/react-query'
import { format } from 'date-fns'
import { behaviorIncidentSchema } from '../schemas'
import { getAllBehaviorIncidentTypesQueryOptions } from '../api'
import type { UseFormReturn } from 'react-hook-form'
import type { BehaviorIncidentFormValues } from '../schemas'
import type {
  BehaviorIncidentResponse,
  StudentResponse,
} from '@/lib/api/types.gen'
import {
  Dialog,
  DialogContent,
  DialogDescription,
  DialogHeader,
  DialogTitle,
} from '@/components/ui/dialog'
import { FormBuilder } from '@/components/form-builder'

interface BehaviorIncidentDialogProps {
  student: StudentResponse | null
  incident?: BehaviorIncidentResponse | null
  open: boolean
  onOpenChange: (open: boolean) => void
  onConfirm: (data: BehaviorIncidentFormValues) => void
  isSubmitting?: boolean
}

export function BehaviorIncidentDialog({
  student,
  incident,
  open,
  onOpenChange,
  onConfirm,
  isSubmitting,
}: BehaviorIncidentDialogProps) {
  const { data: typesData } = useQuery({
    ...getAllBehaviorIncidentTypesQueryOptions(),
    enabled: open,
  })

  const types = typesData || []

  const handleTypeChange = (
    typeId: string,
    form: UseFormReturn<BehaviorIncidentFormValues>,
  ) => {
    if (!typeId) return
    const selectedType = types.find((t) => t.id === typeId)
    if (selectedType) {
      form.setValue('points_awarded', selectedType.default_points)
    }
  }

  return (
    <Dialog open={open} onOpenChange={onOpenChange}>
      <DialogContent className="sm:max-w-[425px]">
        <DialogHeader>
          <DialogTitle>
            {incident ? 'Edit' : 'Record'} Behavior Incident
          </DialogTitle>
          <DialogDescription>
            {student
              ? `For student: ${student.name_english}`
              : 'Record a new behavior incident.'}
          </DialogDescription>
        </DialogHeader>
        <FormBuilder
          schema={behaviorIncidentSchema as any}
          defaultValues={{
            student_id: student?.id || '',
            incident_type_id: incident?.incident_type_id || '',
            incident_date:
              incident?.incident_date.slice(0, 10) ||
              format(new Date(), 'yyyy-MM-dd'),
            description: incident?.description || '',
            points_awarded: incident?.points_awarded || 0,
          }}
          onSubmit={(values) => onConfirm(values as BehaviorIncidentFormValues)}
          preload={(form) => {
            if (open) {
              if (incident) {
                form.reset({
                  student_id: incident.student_id,
                  incident_type_id: incident.incident_type_id,
                  incident_date: incident.incident_date.slice(0, 10),
                  description: incident.description,
                  points_awarded: incident.points_awarded,
                })
              } else if (student) {
                form.reset({
                  student_id: student.id,
                  incident_type_id: '',
                  incident_date: format(new Date(), 'yyyy-MM-dd'),
                  description: '',
                  points_awarded: 0,
                })
              }
            }
          }}
          config={{
            structure: [
              [
                {
                  field: 'incident_type_id',
                  type: 'select',
                  label: 'Incident Type',
                  items: types.map((type) => ({
                    label: type.type_name,
                    value: type.id,
                  })),
                  onValueChange: (value, form) => handleTypeChange(value, form as any),
                  parse: (value: string) => value,
                },
              ],
              [
                {
                  field: 'incident_date',
                  type: 'date-picker',
                  label: 'Incident Date',
                },
              ],
              [
                {
                  field: 'description',
                  type: 'textarea',
                  label: 'Description',
                  placeholder: 'Describe the incident...',
                },
              ],
              [
                {
                  field: 'points_awarded',
                  type: 'input',
                  inputType: 'number',
                  label: 'Points',
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
              label: incident ? 'Update' : 'Record',
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
