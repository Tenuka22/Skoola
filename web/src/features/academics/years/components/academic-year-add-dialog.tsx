'use client'

import { useForm } from 'react-hook-form'
import { zodResolver } from '@hookform/resolvers/zod'
import { academicYearFormSchema } from '../schemas'
import type { AcademicYearFormValues } from '../schemas'
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
import { Input } from '@/components/ui/input'
import { Spinner } from '@/components/ui/spinner'

interface AcademicYearAddDialogProps {
  open: boolean
  onOpenChange: (open: boolean) => void
  onConfirm: (data: AcademicYearFormValues) => void
  isSubmitting?: boolean
}

export function AcademicYearAddDialog({
  open,
  onOpenChange,
  onConfirm,
  isSubmitting,
}: AcademicYearAddDialogProps) {
  const form = useForm<AcademicYearFormValues>({
    resolver: zodResolver(academicYearFormSchema),
  })

  const onSubmit = (values: AcademicYearFormValues) => {
    onConfirm(values)
  }

  return (
    <Dialog open={open} onOpenChange={onOpenChange}>
      <DialogContent>
        <DialogHeader>
          <DialogTitle>Add Academic Year</DialogTitle>
          <DialogDescription>Create a new academic year.</DialogDescription>
        </DialogHeader>
        <Form {...form}>
          <form
            id="add-academic-year-form"
            onSubmit={form.handleSubmit(onSubmit)}
            className="space-y-4"
          >
            <FormField
              control={form.control}
              name="name"
              render={({ field }) => (
                <FormItem>
                  <FormLabel>Name</FormLabel>
                  <FormControl>
                    <Input placeholder="e.g., 2024-2025" {...field} />
                  </FormControl>
                  <FormMessage />
                </FormItem>
              )}
            />
            <FormField
              control={form.control}
              name="start_date"
              render={({ field }) => (
                <FormItem>
                  <FormLabel>Start Date</FormLabel>
                  <FormControl>
                    <Input type="date" {...field} />
                  </FormControl>
                  <FormMessage />
                </FormItem>
              )}
            />
            <FormField
              control={form.control}
              name="end_date"
              render={({ field }) => (
                <FormItem>
                  <FormLabel>End Date</FormLabel>
                  <FormControl>
                    <Input type="date" {...field} />
                  </FormControl>
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
            form="add-academic-year-form"
            disabled={isSubmitting}
          >
            {isSubmitting && <Spinner className="mr-2" />}
            Create
          </Button>
        </DialogFooter>
      </DialogContent>
    </Dialog>
  )
}
