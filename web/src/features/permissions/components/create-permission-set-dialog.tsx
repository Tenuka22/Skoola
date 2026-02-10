'use client'

import { useForm } from 'react-hook-form'
import { zodResolver } from '@hookform/resolvers/zod'
import { z } from 'zod'
import { Button } from '@/components/ui/button'
import { Dialog, DialogContent, DialogHeader, DialogTitle, DialogFooter } from '@/components/ui/dialog'
import { Form, FormControl, FormField, FormItem, FormLabel, FormMessage } from '@/components/ui/form'
import { Input } from '@/components/ui/input'
import { Textarea } from '@/components/ui/textarea'
import { toast } from 'sonner'
import { useMutation, useQueryClient } from '@tanstack/react-query'
import { createPermissionSet } from '../../permissions/api'

const formSchema = z.object({
  name: z.string().min(1, "Name is required"),
  description: z.string().min(1, "Description is required"),
})

interface CreatePermissionSetDialogProps {
  open: boolean
  onOpenChange: (open: boolean) => void
}

export function CreatePermissionSetDialog({ open, onOpenChange }: CreatePermissionSetDialogProps) {
  const queryClient = useQueryClient()
  const form = useForm<z.infer<typeof formSchema>>({
    resolver: zodResolver(formSchema),
    defaultValues: {
      name: '',
      description: '',
    },
  })

  const createMutation = useMutation({
    mutationFn: (values: z.infer<typeof formSchema>) =>
      createPermissionSet(values.name, values.description),
    onSuccess: () => {
      toast.success('Permission set created successfully.')
      onOpenChange(false)
      form.reset()
      queryClient.invalidateQueries({ queryKey: ['permissionSets'] })
    },
    onError: (error) => {
      toast.error(`Failed to create permission set: ${(error as any).message}`)
    },
  })

  const onSubmit = (values: z.infer<typeof formSchema>) => {
    createMutation.mutate(values)
  }

  return (
    <Dialog open={open} onOpenChange={onOpenChange}>
      <DialogContent>
        <DialogHeader>
          <DialogTitle>Create New Permission Set</DialogTitle>
        </DialogHeader>
        <Form {...form}>
          <form onSubmit={form.handleSubmit(onSubmit)} className="space-y-4">
            <FormField
              control={form.control}
              name="name"
              render={({ field }) => (
                <FormItem>
                  <FormLabel>Name</FormLabel>
                  <FormControl>
                    <Input placeholder="Permission Set Name" {...field} />
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
                    <Textarea placeholder="Description of the permission set" {...field} />
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
              <Button type="submit">
                Create
              </Button>
            </DialogFooter>
          </form>
        </Form>
      </DialogContent>
    </Dialog>
  )
}

