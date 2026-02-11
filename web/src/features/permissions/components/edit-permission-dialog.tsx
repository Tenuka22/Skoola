'use client'

import * as React from 'react'
import { useForm } from 'react-hook-form'
import { zodResolver } from '@hookform/resolvers/zod'
import { z } from 'zod'
// import { toast } from 'sonner'
// import { useMutation, useQueryClient } from '@tanstack/react-query'
// import { updatePermission } from '../../permissions/api'
import { PERMISSION_NAMES, PERMISSION_SEVERITIES } from '../constants'
import type { Permission } from '@/lib/api/types.gen'
import { Button } from '@/components/ui/button'
import { Dialog, DialogContent, DialogFooter, DialogHeader, DialogTitle } from '@/components/ui/dialog'
import { Form, FormControl, FormDescription, FormField, FormItem, FormLabel, FormMessage } from '@/components/ui/form'
import { Select, SelectContent, SelectItem, SelectTrigger, SelectValue } from '@/components/ui/select'
import { Textarea } from '@/components/ui/textarea'

const formSchema = z.object({
  name: z.enum(PERMISSION_NAMES as [string, ...Array<string>]).optional(),
  description: z.string().min(1, "Description is required").optional(),
  safety_level: z.enum(PERMISSION_SEVERITIES as [string, ...Array<string>]).optional(),
  is_admin_only: z.boolean().optional(),
})

interface EditPermissionDialogProps {
  open: boolean
  onOpenChange: (open: boolean) => void
  permission: Permission
}

export function EditPermissionDialog({ open, onOpenChange, permission }: EditPermissionDialogProps) {
//   const queryClient = useQueryClient()
  const form = useForm<z.infer<typeof formSchema>>({
    resolver: zodResolver(formSchema),
    defaultValues: {
      name: permission.name,
      description: permission.description,
      safety_level: permission.safety_level,
      is_admin_only: permission.is_admin_only,
    },
  })

  React.useEffect(() => {
    if (permission) {
      form.reset({
        name: permission.name,
        description: permission.description,
        safety_level: permission.safety_level,
        is_admin_only: permission.is_admin_only,
      })
    }
  }, [permission, form])

//   const updateMutation = useMutation({
//     mutationFn: (values: z.infer<typeof formSchema>) =>
//       updatePermission(
//         permission.id,
//         values.name as PermissionEnum,
//         values.description,
//         values.safety_level as PermissionSeverity,
//       ),
//     onSuccess: () => {
//       toast.success('Permission updated successfully.')
//       onOpenChange(false)
//       queryClient.invalidateQueries({ queryKey: ['permissions'] })
//     },
//     onError: (error) => {
//       toast.error(`Failed to update permission: ${(error as any).message}`)
//     },
//   })

//   const onSubmit = (values: z.infer<typeof formSchema>) => {
//     updateMutation.mutate(values)
//   }

  return (
    <Dialog open={open} onOpenChange={onOpenChange}>
      <DialogContent>
        <DialogHeader>
          <DialogTitle>Edit Permission: {permission.name}</DialogTitle>
        </DialogHeader>
        <Form {...form}>
          <form onSubmit={() => {}} className="space-y-4">
            <FormField
              control={form.control}
              name="name"
              render={({ field }) => (
                <FormItem>
                  <FormLabel>Name</FormLabel>
                  <Select onValueChange={field.onChange} defaultValue={field.value}>
                    <FormControl>
                      <SelectTrigger>
                        <SelectValue placeholder="Select a permission name" />
                      </SelectTrigger>
                    </FormControl>
                    <SelectContent>
                      {PERMISSION_NAMES.map((permissionName) => (
                        <SelectItem key={permissionName} value={permissionName}>
                          {permissionName}
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
              name="description"
              render={({ field }) => (
                <FormItem>
                  <FormLabel>Description</FormLabel>
                  <FormControl>
                    <Textarea placeholder="Description of the permission" {...field} />
                  </FormControl>
                  <FormMessage />
                </FormItem>
              )}
            />
            <FormField
              control={form.control}
              name="safety_level"
              render={({ field }) => (
                <FormItem>
                  <FormLabel>Safety Level</FormLabel>
                  <Select onValueChange={field.onChange} defaultValue={field.value}>
                    <FormControl>
                      <SelectTrigger>
                        <SelectValue placeholder="Select a safety level" />
                      </SelectTrigger>
                    </FormControl>
                    <SelectContent>
                      {PERMISSION_SEVERITIES.map((level) => (
                        <SelectItem key={level} value={level}>
                          {level}
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
              name="is_admin_only"
              render={({ field }) => (
                <FormItem className="flex flex-row items-center justify-between rounded-lg border p-3 shadow-sm">
                  <div className="space-y-0.5">
                    <FormLabel>Admin Only</FormLabel>
                    <FormDescription>This permission can only be assigned by an admin.</FormDescription>
                  </div>
                  <FormControl>
                    <input
                      type="checkbox"
                      checked={field.value}
                      onChange={(e) => field.onChange(e.target.checked)}
                      className="h-4 w-4"
                    />
                  </FormControl>
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
                Update
              </Button>
            </DialogFooter>
          </form>
        </Form>
      </DialogContent>
    </Dialog>
  )
}
