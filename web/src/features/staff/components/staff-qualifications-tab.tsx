import * as React from 'react'
import { useQuery } from '@tanstack/react-query'
import { HugeiconsIcon } from '@hugeicons/react'
import { Delete02Icon, PencilEdit01Icon, UserAdd01Icon } from '@hugeicons/core-free-icons'
import type { StaffQualification } from '@/lib/api'
import {
  getStaffQualificationsQueryOptions,
  useCreateStaffQualification,
  useDeleteStaffQualification,
  useUpdateStaffQualification,
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

interface StaffQualificationsTabProps {
  staffId?: string
}

export function StaffQualificationsTab({ staffId }: StaffQualificationsTabProps) {
  const [isCreateOpen, setIsCreateOpen] = React.useState(false)
  const [isEditOpen, setIsEditOpen] = React.useState(false)
  const [isDeleteOpen, setIsDeleteOpen] = React.useState(false)
  const [qualificationToDelete, setQualificationToDelete] = React.useState<string | null>(null)
  const [qualificationToEdit, setQualificationToEdit] = React.useState<StaffQualification | null>(null)

  const { data, isLoading } = useQuery({
    ...getStaffQualificationsQueryOptions({
      client: authClient,
      query: staffId ? { filters: { staff_id: staffId } } : {},
    }),
  })

  const createQualification = useCreateStaffQualification()
  const updateQualification = useUpdateStaffQualification()
  const deleteQualification = useDeleteStaffQualification()

  const [formData, setFormData] = React.useState({
    staff_id: staffId || '',
    degree: '',
    institution: '',
    year_of_completion: new Date().getFullYear(),
  })

  const handleSubmit = () => {
    if (qualificationToEdit) {
      updateQualification.mutate(
        { path: { id: qualificationToEdit.id }, body: formData },
        {
          onSuccess: () => {
            setIsEditOpen(false)
            setQualificationToEdit(null)
          },
        },
      )
    } else {
      createQualification.mutate(formData, {
        onSuccess: () => {
          setIsCreateOpen(false)
          setFormData({
            staff_id: staffId || '',
            degree: '',
            institution: '',
            year_of_completion: new Date().getFullYear(),
          })
        },
      })
    }
  }

  const openEdit = (qualification: StaffQualification) => {
    setQualificationToEdit(qualification)
    setFormData({
      staff_id: qualification.staff_id,
      degree: qualification.degree,
      institution: qualification.institution,
      year_of_completion: qualification.year_of_completion,
    })
    setIsEditOpen(true)
  }

  const handleDelete = () => {
    if (qualificationToDelete) {
      deleteQualification.mutate(
        { id: qualificationToDelete },
        {
          onSuccess: () => {
            setIsDeleteOpen(false)
            setQualificationToDelete(null)
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
            <CardTitle>Qualifications</CardTitle>
            <CardDescription>
              Manage academic and professional qualifications
            </CardDescription>
          </Stack>
          <Button size="sm" onClick={() => setIsCreateOpen(true)}>
            <HugeiconsIcon icon={UserAdd01Icon} className="size-4 mr-2" />
            Add Qualification
          </Button>
        </HStack>
      </CardHeader>
      <CardContent>
        <ScrollArea className="h-[400px]">
          <Table>
            <TableHeader>
              <TableRow>
                <TableHead>Degree</TableHead>
                <TableHead>Institution</TableHead>
                <TableHead>Year</TableHead>
                <TableHead className="w-[100px]">Actions</TableHead>
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
                    <Text muted>No qualifications found</Text>
                  </TableCell>
                </TableRow>
              ) : (
                data.data.map((qual: StaffQualification) => (
                  <TableRow key={qual.id}>
                    <TableCell className="font-medium">{qual.degree}</TableCell>
                    <TableCell>{qual.institution}</TableCell>
                    <TableCell>{qual.year_of_completion}</TableCell>
                    <TableCell>
                      <HStack gap={2}>
                        <Button
                          variant="ghost"
                          size="icon"
                          onClick={() => openEdit(qual)}
                        >
                          <HugeiconsIcon icon={PencilEdit01Icon} className="size-4" />
                        </Button>
                        <Button
                          variant="ghost"
                          size="icon"
                          onClick={() => {
                            setQualificationToDelete(qual.id)
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
          setQualificationToEdit(null)
          setFormData({
            staff_id: staffId || '',
            degree: '',
            institution: '',
            year_of_completion: new Date().getFullYear(),
          })
        }
      }}>
        <DialogContent>
          <DialogHeader>
            <DialogTitle>
              {qualificationToEdit ? 'Edit Qualification' : 'Add Qualification'}
            </DialogTitle>
            <DialogDescription>
              {qualificationToEdit
                ? 'Update qualification information'
                : 'Add new qualification'}
            </DialogDescription>
          </DialogHeader>
          <Stack gap={4}>
            <div className="grid gap-2">
              <Label htmlFor="degree">Degree/Qualification</Label>
              <Input
                id="degree"
                value={formData.degree}
                onChange={(e) =>
                  setFormData((prev) => ({ ...prev, degree: e.target.value }))
                }
                placeholder="e.g., Bachelor of Education"
                required
              />
            </div>
            <div className="grid gap-2">
              <Label htmlFor="institution">Institution</Label>
              <Input
                id="institution"
                value={formData.institution}
                onChange={(e) =>
                  setFormData((prev) => ({ ...prev, institution: e.target.value }))
                }
                placeholder="e.g., University of Colombo"
                required
              />
            </div>
            <div className="grid gap-2">
              <Label htmlFor="year">Year of Completion</Label>
              <Input
                id="year"
                type="number"
                value={formData.year_of_completion}
                onChange={(e) =>
                  setFormData((prev) => ({ ...prev, year_of_completion: Number(e.target.value) }))
                }
                required
              />
            </div>
          </Stack>
          <DialogFooter>
            <Button
              variant="outline"
              onClick={() => {
                setIsCreateOpen(false)
                setIsEditOpen(false)
                setQualificationToEdit(null)
              }}
            >
              Cancel
            </Button>
            <Button onClick={handleSubmit} disabled={createQualification.isPending || updateQualification.isPending}>
              {createQualification.isPending || updateQualification.isPending ? 'Saving...' : 'Save'}
            </Button>
          </DialogFooter>
        </DialogContent>
      </Dialog>

      {/* Delete Confirmation */}
      <AlertDialog open={isDeleteOpen} onOpenChange={setIsDeleteOpen}>
        <AlertDialogContent>
          <AlertDialogHeader>
            <AlertDialogTitle>Delete Qualification</AlertDialogTitle>
            <AlertDialogDescription>
              Are you sure you want to delete this qualification? This action
              cannot be undone.
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
