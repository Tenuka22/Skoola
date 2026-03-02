import * as React from 'react'
import { useForm } from 'react-hook-form'
import { zodResolver } from '@hookform/resolvers/zod'
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
  const form = useForm<BehaviorIncidentTypeFormValues>({
    resolver: zodResolver(behaviorIncidentTypeSchema),
    defaultValues: {
      type_name: '',
      description: '',
      default_points: 0,
    },
  })

  React.useEffect(() => {
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
  }, [type, form])

  const onSubmit = (data: BehaviorIncidentTypeFormValues) => {
    onConfirm(data)
  }

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
        <Form {...form}>
          <form onSubmit={form.handleSubmit(onSubmit)} className="space-y-4">
            <FormField
              control={form.control}
              name="type_name"
              render={({ field }) => (
                <FormItem>
                  <FormLabel>Type Name</FormLabel>
                  <FormControl>
                    <Input placeholder="e.g. Late Arrival" {...field} />
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
                      placeholder="Describe this behavior type..."
                      {...field}
                    />
                  </FormControl>
                  <FormMessage />
                </FormItem>
              )}
            />
            <FormField
              control={form.control}
              name="default_points"
              render={({ field }) => (
                <FormItem>
                  <FormLabel>Default Points</FormLabel>
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
                {type ? 'Update' : 'Create'}
              </Button>
            </DialogFooter>
          </form>
        </Form>
      </DialogContent>
    </Dialog>
  )
}
