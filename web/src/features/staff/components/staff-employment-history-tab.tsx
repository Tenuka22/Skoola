import * as React from 'react'
import { useQuery } from '@tanstack/react-query'
import { HugeiconsIcon } from '@hugeicons/react'
import { Delete02Icon, UserAdd01Icon } from '@hugeicons/core-free-icons'
import { format } from 'date-fns'
import type { StaffEmploymentHistory } from '@/lib/api'
import {
  getStaffEmploymentHistoryQueryOptions,
  useCreateStaffEmploymentHistory,
  useDeleteStaffEmploymentHistory,
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

interface StaffEmploymentHistoryTabProps {
  staffId?: string
}

export function StaffEmploymentHistoryTab({ staffId }: StaffEmploymentHistoryTabProps) {
  const [isCreateOpen, setIsCreateOpen] = React.useState(false)
  const [isDeleteOpen, setIsDeleteOpen] = React.useState(false)
  const [historyToDelete, setHistoryToDelete] = React.useState<string | null>(null)

  const { data, isLoading } = useQuery({
    ...getStaffEmploymentHistoryQueryOptions({
      client: authClient,
      query: staffId ? { filters: { staff_id: staffId } } : {},
    }),
  })

  const createHistory = useCreateStaffEmploymentHistory()
  const deleteHistory = useDeleteStaffEmploymentHistory()

  const [formData, setFormData] = React.useState({
    staff_id: staffId || '',
    previous_school: '',
    position: '',
    start_date: '',
    end_date: '',
    reason_for_leaving: '',
  })

  const handleSubmit = () => {
    createHistory.mutate(formData, {
      onSuccess: () => {
        setIsCreateOpen(false)
        setFormData({
          staff_id: staffId || '',
          previous_school: '',
          position: '',
          start_date: '',
          end_date: '',
          reason_for_leaving: '',
        })
      },
    })
  }

  const handleDelete = () => {
    if (historyToDelete) {
      deleteHistory.mutate(
        { id: historyToDelete },
        {
          onSuccess: () => {
            setIsDeleteOpen(false)
            setHistoryToDelete(null)
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
            <CardTitle>Employment History</CardTitle>
            <CardDescription>
              Track previous employment and work experience
            </CardDescription>
          </Stack>
          <Button size="sm" onClick={() => setIsCreateOpen(true)}>
            <HugeiconsIcon icon={UserAdd01Icon} className="size-4 mr-2" />
            Add History
          </Button>
        </HStack>
      </CardHeader>
      <CardContent>
        <ScrollArea className="h-[400px]">
          <Table>
            <TableHeader>
              <TableRow>
                <TableHead>Previous School</TableHead>
                <TableHead>Position</TableHead>
                <TableHead>Period</TableHead>
                <TableHead>Reason for Leaving</TableHead>
                <TableHead className="w-[100px]">Actions</TableHead>
              </TableRow>
            </TableHeader>
            <TableBody>
              {isLoading ? (
                <TableRow>
                  <TableCell colSpan={5} className="text-center py-8">
                    <Text muted>Loading...</Text>
                  </TableCell>
                </TableRow>
              ) : !data?.data || data.data.length === 0 ? (
                <TableRow>
                  <TableCell colSpan={5} className="text-center py-8">
                    <Text muted>No employment history found</Text>
                  </TableCell>
                </TableRow>
              ) : (
                data.data.map((history: StaffEmploymentHistory) => (
                  <TableRow key={history.id}>
                    <TableCell>{history.previous_school}</TableCell>
                    <TableCell>{history.position}</TableCell>
                    <TableCell>
                      {format(new Date(history.start_date), 'MMM yyyy')} -{' '}
                      {history.end_date
                        ? format(new Date(history.end_date), 'MMM yyyy')
                        : 'Present'}
                    </TableCell>
                    <TableCell>
                      <Text size="sm" muted className="max-w-[200px] truncate">
                        {history.reason_for_leaving || '—'}
                      </Text>
                    </TableCell>
                    <TableCell>
                      <Button
                        variant="ghost"
                        size="icon"
                        onClick={() => {
                          setHistoryToDelete(history.id)
                          setIsDeleteOpen(true)
                        }}
                      >
                        <HugeiconsIcon icon={Delete02Icon} className="size-4" />
                      </Button>
                    </TableCell>
                  </TableRow>
                ))
              )}
            </TableBody>
          </Table>
        </ScrollArea>
      </CardContent>

      {/* Create Dialog */}
      <Dialog open={isCreateOpen} onOpenChange={setIsCreateOpen}>
        <DialogContent>
          <DialogHeader>
            <DialogTitle>Add Employment History</DialogTitle>
            <DialogDescription>
              Add previous employment record
            </DialogDescription>
          </DialogHeader>
          <Stack gap={4}>
            <div className="grid gap-2">
              <Label htmlFor="previous_school">Previous School</Label>
              <Input
                id="previous_school"
                value={formData.previous_school}
                onChange={(e) =>
                  setFormData((prev) => ({ ...prev, previous_school: e.target.value }))
                }
                required
              />
            </div>
            <div className="grid gap-2">
              <Label htmlFor="position">Position</Label>
              <Input
                id="position"
                value={formData.position}
                onChange={(e) =>
                  setFormData((prev) => ({ ...prev, position: e.target.value }))
                }
                required
              />
            </div>
            <div className="grid gap-2">
              <Label htmlFor="start_date">Start Date</Label>
              <Input
                id="start_date"
                type="date"
                value={formData.start_date}
                onChange={(e) =>
                  setFormData((prev) => ({ ...prev, start_date: e.target.value }))
                }
                required
              />
            </div>
            <div className="grid gap-2">
              <Label htmlFor="end_date">End Date (optional)</Label>
              <Input
                id="end_date"
                type="date"
                value={formData.end_date}
                onChange={(e) =>
                  setFormData((prev) => ({ ...prev, end_date: e.target.value }))
                }
              />
            </div>
            <div className="grid gap-2">
              <Label htmlFor="reason">Reason for Leaving</Label>
              <Input
                id="reason"
                value={formData.reason_for_leaving}
                onChange={(e) =>
                  setFormData((prev) => ({ ...prev, reason_for_leaving: e.target.value }))
                }
              />
            </div>
          </Stack>
          <DialogFooter>
            <Button variant="outline" onClick={() => setIsCreateOpen(false)}>
              Cancel
            </Button>
            <Button onClick={handleSubmit} disabled={createHistory.isPending}>
              {createHistory.isPending ? 'Saving...' : 'Save'}
            </Button>
          </DialogFooter>
        </DialogContent>
      </Dialog>

      {/* Delete Confirmation */}
      <AlertDialog open={isDeleteOpen} onOpenChange={setIsDeleteOpen}>
        <AlertDialogContent>
          <AlertDialogHeader>
            <AlertDialogTitle>Delete Employment History</AlertDialogTitle>
            <AlertDialogDescription>
              Are you sure you want to delete this record? This action cannot
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
