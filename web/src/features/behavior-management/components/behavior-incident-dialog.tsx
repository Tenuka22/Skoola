import { useQuery } from '@tanstack/react-query'
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
  DialogFooter,
  DialogHeader,
  DialogTitle,
} from '@/components/ui/dialog'
import { Button } from '@/components/ui/button'
import { FormBuilder, defineFormConfig } from '@/components/form-builder'
import { Spinner } from '@/components/ui/spinner'

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
    form.setValue('incident_type_id', typeId)
  }

  const onSubmit = (data: BehaviorIncidentFormValues) => {
    // Append :00 to ensure seconds are present if not already there
    const formattedDate =
      data.incident_date.length === 16
        ? `${data.incident_date}:00`
        : data.incident_date

    onConfirm({
      ...data,
      incident_date: formattedDate,
    })
  }

  const config = defineFormConfig(behaviorIncidentSchema, {
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
          onValueChange: (
            value: string,
            form: UseFormReturn<BehaviorIncidentFormValues>,
          ) => handleTypeChange(value, form),
          parse: (value: string) => value,
        },
      ],
      [
        {
          field: 'incident_date',
          type: 'input',
          inputType: 'datetime-local',
          label: 'Date & Time',
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
            {incident ? 'Update' : 'Record'}
          </Button>
        </DialogFooter>
      ),
    },
  })

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
          schema={behaviorIncidentSchema}
          config={config}
          defaultValues={{
            student_id: student?.id || '',
            incident_type_id: incident?.incident_type_id || '',
            incident_date:
              incident?.incident_date.slice(0, 16) ||
              new Date().toISOString().slice(0, 16),
            description: incident?.description || '',
            points_awarded: incident?.points_awarded || 0,
          }}
          onSubmit={(values) => onSubmit(values)}
          preload={(form) => {
            if (open) {
              if (incident) {
                form.reset({
                  student_id: incident.student_id,
                  incident_type_id: incident.incident_type_id,
                  incident_date: incident.incident_date.slice(0, 16),
                  description: incident.description,
                  points_awarded: incident.points_awarded,
                })
              } else if (student) {
                form.reset({
                  student_id: student.id,
                  incident_type_id: '',
                  incident_date: new Date().toISOString().slice(0, 16),
                  description: '',
                  points_awarded: 0,
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
