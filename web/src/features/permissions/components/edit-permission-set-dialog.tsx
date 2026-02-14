'use client'

import * as React from 'react'
import { useForm } from 'react-hook-form'
import { zodResolver } from '@hookform/resolvers/zod'
import { z } from 'zod'
import type { PermissionSet } from '../types'
import { zUpdatePermissionSetRequest } from '@/lib/api/zod.gen'
// import { toast } from 'sonner'
// import { useMutation, useQueryClient } from '@tanstack/react-query'
// import { updatePermissionSet } from '../../permissions/api'
import { Button } from '@/components/ui/button'
import {
  Dialog,
  DialogContent,
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
import { Textarea } from '@/components/ui/textarea'

const formSchema = zUpdatePermissionSetRequest.extend({
  name: z.string().min(1, 'Name is required').optional(),
  description: z.string().min(1, 'Description is required').optional(),
})

interface EditPermissionSetDialogProps {
  open: boolean
  onOpenChange: (open: boolean) => void
  permissionSet: PermissionSet
}

export function EditPermissionSetDialog({
  open,
  onOpenChange,
  permissionSet,
}: EditPermissionSetDialogProps) {
  //   const queryClient = useQueryClient()
  const form = useForm<z.infer<typeof formSchema>>({
    resolver: zodResolver(formSchema),
    defaultValues: {
      name: permissionSet.name,
      description: permissionSet.description || '',
    },
  })

  React.useEffect(() => {
    if (permissionSet) {
      form.reset({
        name: permissionSet.name,
        description: permissionSet.description || '',
      })
    }
  }, [permissionSet, form])

  //   const updateMutation = useMutation({
  //     mutationFn: (values: z.infer<typeof formSchema>) =>
  //       updatePermissionSet(permissionSet.id, values.name, values.description),
  //     onSuccess: () => {
  //       toast.success('Permission set updated successfully.')
  //       onOpenChange(false)
  //       queryClient.invalidateQueries({ queryKey: ['permissionSets'] })
  //     },
  //     onError: (error) => {
  //       toast.error(`Failed to update permission set: ${error.message}`)  //     },
  //   })

  //   const onSubmit = (values: z.infer<typeof formSchema>) => {
  //     updateMutation.mutate(values)
  //   }

  return (
    <Dialog open={open} onOpenChange={onOpenChange}>
      <DialogContent>
        <DialogHeader>
          <DialogTitle>Edit Permission Set: {permissionSet.name}</DialogTitle>
        </DialogHeader>
        <Form {...form}>
          <form onSubmit={() => {}} className="space-y-4">
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
                    <Textarea
                      placeholder="Description of the permission set"
                      {...field}
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
              <Button type="submit">Update</Button>
            </DialogFooter>
          </form>
        </Form>
      </DialogContent>
    </Dialog>
  )
}
