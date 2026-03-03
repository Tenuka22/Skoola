import * as React from 'react'
import { useForm } from 'react-hook-form'
import { zodResolver } from '@hookform/resolvers/zod'
import { useQuery } from '@tanstack/react-query'
import { behaviorIncidentSchema } from '../schemas'
import { getAllBehaviorIncidentTypesQueryOptions } from '../api'
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
import {
  Form,
  FormControl,
  FormField,
  FormItem,
  FormLabel,
  FormMessage,
} from '@/components/ui/form'
import { Input } from '@/components/ui/input'
import { Button } from '@/components/ui/button'
import { Textarea } from '@/components/ui/textarea'
import {
  Select,
  SelectContent,
  SelectItem,
  SelectTrigger,
  SelectValue,
} from '@/components/ui/select'

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

  const form = useForm<BehaviorIncidentFormValues>({
    resolver: zodResolver(behaviorIncidentSchema),
    defaultValues: {
      student_id: '',
      incident_type_id: '',
      incident_date: new Date().toISOString().slice(0, 16),
      description: '',
      points_awarded: 0,
    },
  })

  React.useEffect(() => {
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
  }, [incident, student, open, form])

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

  const handleTypeChange = (typeId: string | null) => {
    if (!typeId) return
    const selectedType = types.find((t) => t.id === typeId)
    if (selectedType) {
      form.setValue('points_awarded', selectedType.default_points)
    }
    form.setValue('incident_type_id', typeId)
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
        <Form {...form}>
          <form onSubmit={form.handleSubmit(onSubmit)} className="space-y-4">
            <FormField
              control={form.control}
              name="incident_type_id"
              render={({ field }) => (
                <FormItem>
                  <FormLabel>Incident Type</FormLabel>
                  <Select onValueChange={handleTypeChange} value={field.value}>
                    <FormControl>
                      <SelectTrigger>
                        <SelectValue placeholder="Select type" />
                      </SelectTrigger>
                    </FormControl>
                    <SelectContent>
                      {types.map((type) => (
                        <SelectItem key={type.id} value={type.id}>
                          {type.type_name}
                        </SelectItem>
                      ))}
                    </SelectContent>
                  </Select>
                  <FormMessage />
                </FormItem>
              )}
            />
            <FormField
              control={form.control}
              name="incident_date"
              render={({ field }) => (
                <FormItem>
                  <FormLabel>Date & Time</FormLabel>
                  <FormControl>
                    <Input type="datetime-local" {...field} />
                  </FormControl>
                  <FormMessage />
                </FormItem>
              )}
            />
            <FormField
              control={form.control}
              name="description"
              render={({ field }) => (
                <FormItem>
                  <FormLabel>Description</FormLabel>
                  <FormControl>
                    <Textarea
                      placeholder="Describe the incident..."
                      {...field}
                    />
                  </FormControl>
                  <FormMessage />
                </FormItem>
              )}
            />
            <FormField
              control={form.control}
              name="points_awarded"
              render={({ field }) => (
                <FormItem>
                  <FormLabel>Points</FormLabel>
                  <FormControl>
                    <Input
                      type="number"
                      {...field}
                      onChange={(e) => field.onChange(parseInt(e.target.value))}
                    />
                  </FormControl>
                  <FormMessage />
                </FormItem>
              )}
            />
            <DialogFooter>
              <Button
                type="button"
                variant="outline"
                onClick={() => onOpenChange(false)}
              >
                Cancel
              </Button>
              <Button type="submit" disabled={isSubmitting}>
                {incident ? 'Update' : 'Record'}
              </Button>
            </DialogFooter>
          </form>
        </Form>
      </DialogContent>
    </Dialog>
  )
}
