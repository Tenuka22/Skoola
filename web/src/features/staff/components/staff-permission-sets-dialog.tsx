import * as React from 'react'
import { HugeiconsIcon } from '@hugeicons/react'
import {
  AlertCircleIcon,
  Delete02Icon,
  Layers01Icon,
} from '@hugeicons/core-free-icons'
import { useMutation, useQuery, useQueryClient } from '@tanstack/react-query'
import { toast } from 'sonner'
import type { StaffResponse, UserSet } from '@/lib/api/types.gen'
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
import {
  assignPermissionSetToStaffMutation,
  getAllPermissionSetsOptions,
  getStaffPermissionSetsOptions,
  unassignPermissionSetFromStaffMutation,
} from '@/lib/api/@tanstack/react-query.gen'
import { Badge } from '@/components/ui/badge'
import {
  Select,
  SelectContent,
  SelectItem,
  SelectTrigger,
  SelectValue,
} from '@/components/ui/select'
import { Label } from '@/components/ui/label'

interface StaffPermissionSetsDialogProps {
  staff: StaffResponse | null
  open: boolean
  onOpenChange: (open: boolean) => void
}

export function StaffPermissionSetsDialog({
  staff,
  open,
  onOpenChange,
}: StaffPermissionSetsDialogProps) {
  const queryClient = useQueryClient()

  const { data: allSetsData } = useQuery(
    getAllPermissionSetsOptions({ client: authClient }),
  )
  const allSets = allSetsData || []

  const {
    data: staffSetsData,
    isLoading,
    isError,
    error,
  } = useQuery({
    ...getStaffPermissionSetsOptions({
      client: authClient,
      path: { staff_id: staff?.id ?? '' },
    }),
    enabled: !!staff,
  })
  const staffSets = staffSetsData || []

  const assignSet = useMutation({
    ...assignPermissionSetToStaffMutation({ client: authClient }),
    onSuccess: () => {
      toast.success('Permission set assigned.')
      queryClient.invalidateQueries({
        queryKey: ['getStaffPermissionSets', { staff_id: staff?.id }],
      })
    },
    onError: (error) => {
      toast.error(`Failed to assign set: ${error.message || 'Unknown error'}`)
    },
  })

  const unassignSet = useMutation({
    ...unassignPermissionSetFromStaffMutation({ client: authClient }),
    onSuccess: () => {
      toast.success('Permission set removed.')
      queryClient.invalidateQueries({
        queryKey: ['getStaffPermissionSets', { staff_id: staff?.id }],
      })
    },
    onError: (error) => {
      toast.error(`Failed to remove set: ${error.message || 'Unknown error'}`)
    },
  })

  const handleAssignSet = React.useCallback(
    (value: string | null) => {
      if (!value || !staff) return
      assignSet.mutate({
        path: { staff_id: staff.id, set_id: value },
      })
    },
    [assignSet, staff],
  )

  const availableSets = allSets.filter(
    (set: UserSet) => !staffSets.some((ss: UserSet) => ss.id === set.id),
  )

  return (
    <Dialog open={open} onOpenChange={onOpenChange}>
      <DialogContent className="max-w-xl flex flex-col h-[70vh]">
        <DialogHeader>
          <DialogTitle>Permission Sets: {staff?.name}</DialogTitle>
          <DialogDescription>
            Assign specialized permission sets to this staff member.
          </DialogDescription>
        </DialogHeader>

        <div className="flex flex-col gap-6 flex-1 overflow-hidden">
          {/* Add New Set */}
          <div className="space-y-2">
            <Label>Assign New Set</Label>
            <div className="flex gap-2">
              <Select onValueChange={handleAssignSet}>
                <SelectTrigger>
                  <SelectValue placeholder="Select a set to assign" />
                </SelectTrigger>
                <SelectContent>
                  {availableSets.map((set: UserSet) => (
                    <SelectItem key={set.id} value={set.id}>
                      {set.name}
                    </SelectItem>
                  ))}
                </SelectContent>
              </Select>
            </div>
          </div>

          {/* Current Sets */}
          <div className="flex-1 flex flex-col min-h-0">
            <h3 className="text-sm font-semibold mb-2 flex items-center gap-2">
              <HugeiconsIcon icon={Layers01Icon} className="size-4" />
              Assigned Permission Sets
            </h3>
            {isLoading ? (
              <div className="grid flex-1 place-items-center">
                <Spinner />
              </div>
            ) : isError ? (
              <div className="grid flex-1 place-items-center text-center">
                <HugeiconsIcon
                  icon={AlertCircleIcon}
                  className="size-8 text-destructive opacity-50"
                />
                <p className="text-xs text-muted-foreground mt-2">
                  Error: {error?.message}
                </p>
              </div>
            ) : staffSets.length === 0 ? (
              <p className="text-xs text-muted-foreground italic text-center p-8">
                No sets assigned.
              </p>
            ) : (
              <div className="flex flex-wrap gap-2">
                {staffSets.map((set: UserSet) => (
                  <Badge
                    key={set.id}
                    variant="secondary"
                    className="pl-3 pr-1 py-1 gap-2"
                  >
                    {set.name}
                    <Button
                      variant="ghost"
                      size="icon"
                      className="size-4 p-0 hover:bg-destructive/20 hover:text-destructive"
                      onClick={() =>
                        staff &&
                        unassignSet.mutate({
                          path: { staff_id: staff.id, set_id: set.id },
                        })
                      }
                    >
                      <HugeiconsIcon icon={Delete02Icon} className="size-3" />
                    </Button>
                  </Badge>
                ))}
              </div>
            )}
          </div>
        </div>
      </DialogContent>
    </Dialog>
  )
}
