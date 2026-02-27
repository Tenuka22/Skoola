import { HugeiconsIcon } from '@hugeicons/react'
import { Calendar01Icon, FloppyDiskIcon } from '@hugeicons/core-free-icons'
import { useMutation, useQuery, useQueryClient } from '@tanstack/react-query'
import { format } from 'date-fns'
import { toast } from 'sonner'
import type { LeaveBalanceResponse, StaffResponse } from '@/lib/api/types.gen'
import type { z } from 'zod'
import type { UseFormReturn } from 'react-hook-form'
import { Button } from '@/components/ui/button'
import {
  Dialog,
  DialogContent,
  DialogDescription,
  DialogHeader,
  DialogTitle,
} from '@/components/ui/dialog'
import { ScrollArea } from '@/components/ui/scroll-area'
import { Spinner } from '@/components/ui/spinner'
import { authClient } from '@/lib/clients'
import {
  applyForLeaveMutation,
  viewLeaveBalanceOptions,
} from '@/lib/api/@tanstack/react-query.gen'
import { Badge } from '@/components/ui/badge'
import { Input } from '@/components/ui/input'
import { Label } from '@/components/ui/label'
import { zApplyLeaveRequest } from '@/lib/api/zod.gen'
import { FormBuilder, defineFormConfig } from '@/components/form-builder'

type LeaveFormValues = z.infer<typeof zApplyLeaveRequest>

interface StaffLeaveDialogProps {
  staff: StaffResponse | null
  open: boolean
  onOpenChange: (open: boolean) => void
}

export function StaffLeaveDialog({
  staff,
  open,
  onOpenChange,
}: StaffLeaveDialogProps) {
  const queryClient = useQueryClient()

  const {
    data: balanceData,
    isLoading,
    isError,
    error,
  } = useQuery({
    ...viewLeaveBalanceOptions({
      client: authClient,
      path: { staff_id: staff?.id ?? '' },
    }),
    enabled: !!staff,
  })

  const applyLeave = useMutation({
    ...applyForLeaveMutation({ client: authClient }),
    onError: (error) => {
      toast.error(
        `Failed to apply for leave: ${error.message || 'Unknown error'}`,
      )
    },
  })

  const onSubmit = (
    data: LeaveFormValues,
    form: UseFormReturn<LeaveFormValues, unknown, LeaveFormValues>,
  ) => {
    if (staff) {
      applyLeave.mutate(
        {
          path: { staff_id: staff.id },
          body: data,
        },
        {
          onSuccess: () => {
            toast.success('Leave application submitted successfully.')
            queryClient.invalidateQueries({
              queryKey: ['viewLeaveBalance', { staff_id: staff?.id }],
            })
            form.reset()
          },
        },
      )
    }
  }

  const balances = balanceData || []

  const config = defineFormConfig(zApplyLeaveRequest, {
    structure: [],
    extras: {
      top: (form) => (
        <div className="grid grid-cols-2 gap-4">
          <div className="space-y-2">
            <Label htmlFor="leave_type">Leave Type</Label>
            <Input
              id="leave_type"
              {...form.register('leave_type')}
              placeholder="e.g. Sick, Casual"
            />
            {form.formState.errors.leave_type && (
              <p className="text-xs text-red-500">
                {form.formState.errors.leave_type.message}
              </p>
            )}
          </div>
          <div className="space-y-2">
            <Label htmlFor="from_date">From Date</Label>
            <Input id="from_date" type="date" {...form.register('from_date')} />
            {form.formState.errors.from_date && (
              <p className="text-xs text-red-500">
                {form.formState.errors.from_date.message}
              </p>
            )}
          </div>
          <div className="space-y-2">
            <Label htmlFor="to_date">To Date</Label>
            <Input id="to_date" type="date" {...form.register('to_date')} />
            {form.formState.errors.to_date && (
              <p className="text-xs text-red-500">
                {form.formState.errors.to_date.message}
              </p>
            )}
          </div>
          <div className="col-span-2 space-y-2">
            <Label htmlFor="reason">Reason</Label>
            <Input
              id="reason"
              placeholder="Brief reason for leave"
              {...form.register('reason')}
            />
            {form.formState.errors.reason && (
              <p className="text-xs text-red-500">
                {form.formState.errors.reason.message}
              </p>
            )}
          </div>
        </div>
      ),
      bottom: (
        <Button
          type="submit"
          className="w-full"
          disabled={applyLeave.isPending}
        >
          {applyLeave.isPending ? (
            <Spinner className="mr-2" />
          ) : (
            <HugeiconsIcon icon={FloppyDiskIcon} className="size-4 mr-2" />
          )}
          Submit Application
        </Button>
      ),
    },
  })

  return (
    <Dialog open={open} onOpenChange={onOpenChange}>
      <DialogContent className="max-w-2xl flex flex-col h-[85vh]">
        <DialogHeader>
          <DialogTitle>Leave Management: {staff?.name}</DialogTitle>
          <DialogDescription>
            Apply for leave and view current balances.
          </DialogDescription>
        </DialogHeader>

        <div className="flex flex-col gap-6 flex-1 overflow-hidden">
          {/* Leave Balances */}
          <div className="grid grid-cols-2 gap-4">
            {isLoading ? (
              <Spinner />
            ) : isError ? (
              <p className="text-xs text-destructive">
                Error loading balances: {error?.message}
              </p>
            ) : balances.length === 0 ? (
              <p className="text-xs text-muted-foreground italic text-center col-span-2">
                No leave data available.
              </p>
            ) : (
              balances.map((b: LeaveBalanceResponse, i: number) => (
                <div
                  key={i}
                  className="p-3 border rounded-lg bg-muted/20 flex justify-between items-center"
                >
                  <span className="text-sm font-medium">{b.leave_type}</span>
                  <Badge variant="secondary">
                    {b.total_days_taken} days taken
                  </Badge>
                </div>
              ))
            )}
          </div>

          {/* Apply Leave Form */}
          <div className="flex-1 flex flex-col min-h-0">
            <h3 className="text-sm font-semibold mb-2 flex items-center gap-2">
              <HugeiconsIcon icon={Calendar01Icon} className="size-4" />
              Apply for New Leave
            </h3>
            <ScrollArea className="flex-1">
              <FormBuilder
                schema={zApplyLeaveRequest}
                config={config}
                defaultValues={{
                  leave_type: 'Sick',
                  from_date: format(new Date(), 'yyyy-MM-dd'),
                  to_date: format(new Date(), 'yyyy-MM-dd'),
                  reason: '',
                }}
                onSubmit={onSubmit}
                isLoading={applyLeave.isPending}
                showErrorSummary={false}
                toastErrors={false}
                showSuccessAlert={false}
                actions={[]}
                className="space-y-4 p-4 border rounded-xl bg-muted/30"
              />
            </ScrollArea>
          </div>
        </div>
      </DialogContent>
    </Dialog>
  )
}
