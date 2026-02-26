'use client'

import { useForm } from 'react-hook-form'
import { zodResolver } from '@hookform/resolvers/zod'
import { bulkEditStaffFormSchema } from '../schemas'
import type { BulkEditStaffFormValues } from '../schemas'
import {
  Dialog,
  DialogContent,
  DialogDescription,
  DialogFooter,
  DialogHeader,
  DialogTitle,
} from '@/components/ui/dialog'
import { Button } from '@/components/ui/button'
import {
  Form,
  FormControl,
  FormField,
  FormItem,
  FormLabel,
  FormMessage,
} from '@/components/ui/form'
import {
  Select,
  SelectContent,
  SelectItem,
  SelectTrigger,
  SelectValue,
} from '@/components/ui/select'
import { Spinner } from '@/components/ui/spinner'
import { zEmploymentStatus, zStaffType } from '@/lib/api/zod.gen'

interface StaffBulkEditDialogProps {
  open: boolean
  onOpenChange: (open: boolean) => void
  onConfirm: (data: BulkEditStaffFormValues) => void
  selectedCount: number
  isSubmitting?: boolean
}

export function StaffBulkEditDialog({
  open,
  onOpenChange,
  onConfirm,
  selectedCount,
  isSubmitting,
}: StaffBulkEditDialogProps) {
  const form = useForm<BulkEditStaffFormValues>({
    resolver: zodResolver(bulkEditStaffFormSchema),
  })

  const staffTypes = zStaffType.options
  const employmentStatuses = zEmploymentStatus.options

  const onSubmit = (values: BulkEditStaffFormValues) => {
    onConfirm(values)
  }

  return (
    <Dialog open={open} onOpenChange={onOpenChange}>
      <DialogContent>
        <DialogHeader>
          <DialogTitle>Bulk Edit Staff</DialogTitle>
          <DialogDescription>
            This will update {selectedCount} staff members.
          </DialogDescription>
        </DialogHeader>
        <Form {...form}>
          <form
            id="bulk-edit-staff-form"
            onSubmit={form.handleSubmit(onSubmit)}
            className="space-y-4"
          >
            <FormField
              control={form.control}
              name="staff_type"
              render={({ field }) => (
                <FormItem>
                  <FormLabel>Staff Type</FormLabel>
                  <Select
                    onValueChange={field.onChange}
                    defaultValue={field.value}
                  >
                    <FormControl>
                      <SelectTrigger>
                        <SelectValue placeholder="Select a type" />
                      </SelectTrigger>
                    </FormControl>
                    <SelectContent>
                      {staffTypes.map((type) => (
                        <SelectItem key={type} value={type}>
                          {type}
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
              name="employment_status"
              render={({ field }) => (
                <FormItem>
                  <FormLabel>Employment Status</FormLabel>
                  <Select
                    onValueChange={field.onChange}
                    defaultValue={field.value}
                  >
                    <FormControl>
                      <SelectTrigger>
                        <SelectValue placeholder="Select a status" />
                      </SelectTrigger>
                    </FormControl>
                    <SelectContent>
                      {employmentStatuses.map((status) => (
                        <SelectItem key={status} value={status}>
                          {status}
                        </SelectItem>
                      ))}
                    </SelectContent>
                  </Select>
                  <FormMessage />
                </FormItem>
              )}
            />
          </form>
        </Form>
        <DialogFooter>
          <Button variant="ghost" onClick={() => onOpenChange(false)}>
            Cancel
          </Button>
          <Button
            type="submit"
            form="bulk-edit-staff-form"
            disabled={isSubmitting}
          >
            {isSubmitting && <Spinner className="mr-2" />}
            Save changes
          </Button>
        </DialogFooter>
      </DialogContent>
    </Dialog>
  )
}
