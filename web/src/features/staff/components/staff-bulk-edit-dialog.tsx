'use client'

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
import { FormBuilder, defineFormConfig } from '@/components/form-builder'

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
  const staffTypes = zStaffType.options
  const employmentStatuses = zEmploymentStatus.options

  const config = defineFormConfig(bulkEditStaffFormSchema, {
    structure: [],
    extras: {
      top: (form) => (
        <>
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
        </>
      ),
      bottom: (
        <DialogFooter>
          <Button variant="ghost" onClick={() => onOpenChange(false)}>
            Cancel
          </Button>
          <Button type="submit" disabled={isSubmitting}>
            {isSubmitting && <Spinner className="mr-2" />}
            Save changes
          </Button>
        </DialogFooter>
      ),
    },
  })

  return (
    <Dialog open={open} onOpenChange={onOpenChange}>
      <DialogContent>
        <DialogHeader>
          <DialogTitle>Bulk Edit Staff</DialogTitle>
          <DialogDescription>
            This will update {selectedCount} staff members.
          </DialogDescription>
        </DialogHeader>
        <FormBuilder
          schema={bulkEditStaffFormSchema}
          config={config}
          onSubmit={(values) => onConfirm(values)}
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
