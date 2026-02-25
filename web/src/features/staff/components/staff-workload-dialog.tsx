import { HugeiconsIcon } from '@hugeicons/react'
import { AlertCircleIcon, Book01Icon, Chart01Icon, School01Icon } from '@hugeicons/core-free-icons'
import { useQuery } from '@tanstack/react-query'
import type { StaffResponse } from '@/lib/api/types.gen'
import { Button } from '@/components/ui/button'
import {
  Dialog,
  DialogContent,
  DialogDescription,
  DialogHeader,
  DialogTitle,
} from '@/components/ui/dialog'
import { Spinner } from '@/components/ui/spinner'
import { authClient } from '@/lib/clients'
import { getTeacherWorkloadOptions } from '@/lib/api/@tanstack/react-query.gen'
import { Card, CardContent, CardHeader, CardTitle } from '@/components/ui/card'

interface StaffWorkloadDialogProps {
  staff: StaffResponse | null
  open: boolean
  onOpenChange: (open: boolean) => void
}

export function StaffWorkloadDialog({
  staff,
  open,
  onOpenChange,
}: StaffWorkloadDialogProps) {
  const { data: workload, isLoading, isError, error } = useQuery({
    ...getTeacherWorkloadOptions({
      client: authClient,
      path: { teacher_id: staff?.id ?? '' },
    }),
    enabled: !!staff,
  })

  return (
    <Dialog open={open} onOpenChange={onOpenChange}>
      <DialogContent className="max-w-xl">
        <DialogHeader>
          <DialogTitle>Teacher Workload: {staff?.name}</DialogTitle>
          <DialogDescription>
            Summary of classes and subjects assigned to this teacher.
          </DialogDescription>
        </DialogHeader>

        <div className="flex flex-col gap-4 py-4">
          {isLoading ? (
            <div className="grid place-items-center py-8">
              <Spinner />
            </div>
          ) : isError ? (
            <div className="grid place-items-center px-4 py-8 text-center border rounded-lg">
              <HugeiconsIcon
                icon={AlertCircleIcon}
                className="size-12 text-destructive"
              />
              <p className="text-sm text-muted-foreground mt-2">
                Error loading workload: {error?.message}
              </p>
            </div>
          ) : !workload ? (
            <div className="grid place-items-center px-4 py-8 text-center border rounded-lg">
              <HugeiconsIcon
                icon={Chart01Icon}
                className="size-12 text-muted-foreground opacity-20"
              />
              <p className="text-sm text-muted-foreground mt-2">
                No workload information found.
              </p>
            </div>
          ) : (
            <div className="grid grid-cols-2 gap-4">
              <Card>
                <CardHeader className="flex flex-row items-center justify-between pb-2">
                  <CardTitle className="text-sm font-medium">Classes</CardTitle>
                  <HugeiconsIcon icon={School01Icon} className="size-4 text-muted-foreground" />
                </CardHeader>
                <CardContent>
                  <div className="text-2xl font-bold">{workload.total_classes_assigned}</div>
                  <p className="text-xs text-muted-foreground">Total classes assigned</p>
                </CardContent>
              </Card>
              <Card>
                <CardHeader className="flex flex-row items-center justify-between pb-2">
                  <CardTitle className="text-sm font-medium">Subjects</CardTitle>
                  <HugeiconsIcon icon={Book01Icon} className="size-4 text-muted-foreground" />
                </CardHeader>
                <CardContent>
                  <div className="text-2xl font-bold">{workload.total_subjects_assigned}</div>
                  <p className="text-xs text-muted-foreground">Total subjects assigned</p>
                </CardContent>
              </Card>
            </div>
          )}
        </div>
        <Button onClick={() => onOpenChange(false)} className="mt-4">
          Close
        </Button>
      </DialogContent>
    </Dialog>
  )
}
