import * as React from 'react'
import { useQuery } from '@tanstack/react-query'
import { HugeiconsIcon } from '@hugeicons/react'
import { Delete02Icon, PencilEdit01Icon, UserAdd01Icon } from '@hugeicons/core-free-icons'
import { format } from 'date-fns'
import type { StaffEmploymentStatusResponse } from '@/lib/api'
import {
  getStaffEmploymentStatusQueryOptions,
  useCreateStaffEmploymentStatus,
  useDeleteStaffEmploymentStatus,
  useUpdateStaffEmploymentStatus,
} from '@/features/staff/api'
import { authClient } from '@/lib/clients'
import { Button } from '@/components/ui/button'
import {
  Table,
  TableBody,
  TableCell,
  TableHead,
  TableHeader,
  TableRow,
} from '@/components/ui/table'
import {
  Dialog,
  DialogContent,
  DialogDescription,
  DialogFooter,
  DialogHeader,
  DialogTitle,
} from '@/components/ui/dialog'
import {
  AlertDialog,
  AlertDialogAction,
  AlertDialogCancel,
  AlertDialogContent,
  AlertDialogDescription,
  AlertDialogFooter,
  AlertDialogHeader,
  AlertDialogTitle,
} from '@/components/ui/alert-dialog'
import { Input } from '@/components/ui/input'
import { Label } from '@/components/ui/label'
import { HStack, Stack, Text } from '@/components/primitives'
import { Card, CardContent, CardDescription, CardHeader, CardTitle } from '@/components/ui/card'
import { ScrollArea } from '@/components/ui/scroll-area'
import { Badge } from '@/components/ui/badge'
import {
  Select,
  SelectContent,
  SelectItem,
  SelectTrigger,
  SelectValue,
} from '@/components/ui/select'

interface StaffEmploymentStatusTabProps {
  staffId?: string
}

export function StaffEmploymentStatusTab({ staffId }: StaffEmploymentStatusTabProps) {
  const [isCreateOpen, setIsCreateOpen] = React.useState(false)
  const [isEditOpen, setIsEditOpen] = React.useState(false)
  const [isDeleteOpen, setIsDeleteOpen] = React.useState(false)
  const [statusToDelete, setStatusToDelete] = React.useState<string | null>(null)
  const [statusToEdit, setStatusToEdit] = React.useState<StaffEmploymentStatusResponse | null>(null)

  const { data, isLoading } = useQuery({
    ...getStaffEmploymentStatusQueryOptions({
      client: authClient,
      query: staffId ? { search: staffId } : {},
    }),
  })

  const createStatus = useCreateStaffEmploymentStatus()
  const updateStatus = useUpdateStaffEmploymentStatus()
  const deleteStatus = useDeleteStaffEmploymentStatus()

  const [formData, setFormData] = React.useState({
    staff_id: staffId || '',
    employment_status: 'Active',
    effective_date: new Date().toISOString().split('T')[0],
    notes: '',
  })

  const handleSubmit = () => {
    if (statusToEdit) {
      updateStatus.mutate(
        { path: { id: statusToEdit.staff_id }, body: { employment_status: formData.employment_status } },
        {
          onSuccess: () => {
            setIsEditOpen(false)
            setStatusToEdit(null)
          },
        },
      )
    } else {
      createStatus.mutate(formData, {
        onSuccess: () => {
          setIsCreateOpen(false)
          setFormData({
            staff_id: staffId || '',
            employment_status: 'Active',
            effective_date: new Date().toISOString().split('T')[0],
            notes: '',
          })
        },
      })
    }
  }

  const openEdit = (status: StaffEmploymentStatusResponse) => {
    setStatusToEdit(status)
    setFormData({
      staff_id: status.staff_id,
      employment_status: status.employment_status,
      effective_date: status.created_at,
      notes: '',
    })
    setIsEditOpen(true)
  }

  const handleDelete = () => {
    if (statusToDelete) {
      deleteStatus.mutate(
        { id: statusToDelete },
        {
          onSuccess: () => {
            setIsDeleteOpen(false)
            setStatusToDelete(null)
          },
        },
      )
    }
  }

  return (
    <Card>
      <CardHeader>
        <HStack justify="between" align="center">
          <Stack gap={1}>
            <CardTitle>Employment Status</CardTitle>
            <CardDescription>
              Track staff employment status changes
            </CardDescription>
          </Stack>
          <Button size="sm" onClick={() => setIsCreateOpen(true)}>
            <HugeiconsIcon icon={UserAdd01Icon} className="size-4 mr-2" />
            Add Status
          </Button>
        </HStack>
      </CardHeader>
      <CardContent>
        <ScrollArea className="h-[400px]">
          <Table>
            <TableHeader>
              <TableRow>
                <TableHead>Status</TableHead>
                <TableHead>Effective Date</TableHead>
                <TableHead>Notes</TableHead>
                <TableHead className="w-[150px]">Actions</TableHead>
              </TableRow>
            </TableHeader>
            <TableBody>
              {isLoading ? (
                <TableRow>
                  <TableCell colSpan={4} className="text-center py-8">
                    <Text muted>Loading...</Text>
                  </TableCell>
                </TableRow>
              ) : !data?.data || data.data.length === 0 ? (
                <TableRow>
                  <TableCell colSpan={4} className="text-center py-8">
                    <Text muted>No employment status records found</Text>
                  </TableCell>
                </TableRow>
              ) : (
                data.data.map((status: StaffEmploymentStatusResponse) => (
                  <TableRow key={status.staff_id}>
                    <TableCell>
                      <Badge
                        variant="outline"
                        className={
                          status.employment_status === 'Active'
                            ? 'bg-green-500/10 text-green-500'
                            : status.employment_status === 'Inactive'
                            ? 'bg-red-500/10 text-red-500'
                            : status.employment_status === 'OnLeave'
                            ? 'bg-yellow-500/10 text-yellow-500'
                            : 'bg-gray-500/10 text-gray-500'
                        }
                      >
                        {status.employment_status}
                      </Badge>
                    </TableCell>
                    <TableCell>
                      {format(new Date(status.created_at), 'MMM dd, yyyy')}
                    </TableCell>
                    <TableCell>
                      <Text size="sm" muted className="max-w-[200px] truncate">
                        —
                      </Text>
                    </TableCell>
                    <TableCell>
                      <HStack gap={2}>
                        <Button
                          variant="ghost"
                          size="icon"
                          onClick={() => openEdit(status)}
                        >
                          <HugeiconsIcon icon={PencilEdit01Icon} className="size-4" />
                        </Button>
                        <Button
                          variant="ghost"
                          size="icon"
                          onClick={() => {
                            setStatusToDelete(status.staff_id)
                            setIsDeleteOpen(true)
                          }}
                        >
                          <HugeiconsIcon icon={Delete02Icon} className="size-4" />
                        </Button>
                      </HStack>
                    </TableCell>
                  </TableRow>
                ))
              )}
            </TableBody>
          </Table>
        </ScrollArea>
      </CardContent>

      {/* Create/Edit Dialog */}
      <Dialog open={isCreateOpen || isEditOpen} onOpenChange={(open) => {
        if (!open) {
          setIsCreateOpen(false)
          setIsEditOpen(false)
          setStatusToEdit(null)
          setFormData({
            staff_id: staffId || '',
            employment_status: 'Active',
            effective_date: new Date().toISOString().split('T')[0],
            notes: '',
          })
        }
      }}>
        <DialogContent>
          <DialogHeader>
            <DialogTitle>
              {statusToEdit ? 'Edit Status' : 'Add Employment Status'}
            </DialogTitle>
            <DialogDescription>
              {statusToEdit
                ? 'Update employment status'
                : 'Add new employment status record'}
            </DialogDescription>
          </DialogHeader>
          <Stack gap={4}>
            <div className="grid gap-2">
              <Label htmlFor="status">Status</Label>
              <Select
                value={formData.employment_status}
                onValueChange={(value) =>
                  setFormData((prev) => ({ ...prev, employment_status: value ?? 'Active' }))
                }
              >
                <SelectTrigger>
                  <SelectValue />
                </SelectTrigger>
                <SelectContent>
                  <SelectItem value="Active">Active</SelectItem>
                  <SelectItem value="Inactive">Inactive</SelectItem>
                  <SelectItem value="OnLeave">On Leave</SelectItem>
                  <SelectItem value="Suspended">Suspended</SelectItem>
                  <SelectItem value="Terminated">Terminated</SelectItem>
                  <SelectItem value="Retired">Retired</SelectItem>
                </SelectContent>
              </Select>
            </div>
            <div className="grid gap-2">
              <Label htmlFor="effective_date">Effective Date</Label>
              <Input
                id="effective_date"
                type="date"
                value={formData.effective_date}
                onChange={(e) =>
                  setFormData((prev) => ({ ...prev, effective_date: e.target.value }))
                }
                required
              />
            </div>
            <div className="grid gap-2">
              <Label htmlFor="notes">Notes (optional)</Label>
              <Input
                id="notes"
                value={formData.notes}
                onChange={(e) =>
                  setFormData((prev) => ({ ...prev, notes: e.target.value }))
                }
              />
            </div>
          </Stack>
          <DialogFooter>
            <Button
              variant="outline"
              onClick={() => {
                setIsCreateOpen(false)
                setIsEditOpen(false)
                setStatusToEdit(null)
              }}
            >
              Cancel
            </Button>
            <Button onClick={handleSubmit} disabled={createStatus.isPending || updateStatus.isPending}>
              {createStatus.isPending || updateStatus.isPending ? 'Saving...' : 'Save'}
            </Button>
          </DialogFooter>
        </DialogContent>
      </Dialog>

      {/* Delete Confirmation */}
      <AlertDialog open={isDeleteOpen} onOpenChange={setIsDeleteOpen}>
        <AlertDialogContent>
          <AlertDialogHeader>
            <AlertDialogTitle>Delete Status</AlertDialogTitle>
            <AlertDialogDescription>
              Are you sure you want to delete this status record? This action cannot
              be undone.
            </AlertDialogDescription>
          </AlertDialogHeader>
          <AlertDialogFooter>
            <AlertDialogCancel>Cancel</AlertDialogCancel>
            <AlertDialogAction
              onClick={handleDelete}
              className="bg-destructive text-destructive-foreground hover:bg-destructive/90"
            >
              Delete
            </AlertDialogAction>
          </AlertDialogFooter>
        </AlertDialogContent>
      </AlertDialog>
    </Card>
  )
}
