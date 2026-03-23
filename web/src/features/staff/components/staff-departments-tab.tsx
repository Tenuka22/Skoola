import * as React from 'react'
import { useQuery } from '@tanstack/react-query'
import { HugeiconsIcon } from '@hugeicons/react'
import { Delete02Icon, PencilEdit01Icon, UserAdd01Icon } from '@hugeicons/core-free-icons'
import type { StaffDepartment } from '@/lib/api'
import {
  getStaffDepartmentsQueryOptions,
  useCreateStaffDepartment,
  useDeleteStaffDepartment,
  useUpdateStaffDepartment,
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
import { Textarea } from '@/components/ui/textarea'

interface StaffDepartmentsTabProps {
  staffId?: string
}

export function StaffDepartmentsTab(_props: StaffDepartmentsTabProps) {
  const [isCreateOpen, setIsCreateOpen] = React.useState(false)
  const [isEditOpen, setIsEditOpen] = React.useState(false)
  const [isDeleteOpen, setIsDeleteOpen] = React.useState(false)
  const [departmentToDelete, setDepartmentToDelete] = React.useState<string | null>(null)
  const [departmentToEdit, setDepartmentToEdit] = React.useState<StaffDepartment | null>(null)

  const { data, isLoading } = useQuery({
    ...getStaffDepartmentsQueryOptions({
      client: authClient,
    }),
  })

  const createDepartment = useCreateStaffDepartment()
  const updateDepartment = useUpdateStaffDepartment()
  const deleteDepartment = useDeleteStaffDepartment()

  const [formData, setFormData] = React.useState({
    name: '',
    description: '',
  })

  const handleSubmit = () => {
    if (departmentToEdit) {
      updateDepartment.mutate(
        { path: { id: departmentToEdit.id }, body: formData },
        {
          onSuccess: () => {
            setIsEditOpen(false)
            setDepartmentToEdit(null)
          },
        },
      )
    } else {
      createDepartment.mutate(formData, {
        onSuccess: () => {
          setIsCreateOpen(false)
          setFormData({
            name: '',
            description: '',
          })
        },
      })
    }
  }

  const openEdit = (department: StaffDepartment) => {
    setDepartmentToEdit(department)
    setFormData({
      name: department.name,
      description: department.description || '',
    })
    setIsEditOpen(true)
  }

  const handleDelete = () => {
    if (departmentToDelete) {
      deleteDepartment.mutate(
        { id: departmentToDelete },
        {
          onSuccess: () => {
            setIsDeleteOpen(false)
            setDepartmentToDelete(null)
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
            <CardTitle>Departments</CardTitle>
            <CardDescription>
              Manage staff departments and organizational units
            </CardDescription>
          </Stack>
          <Button size="sm" onClick={() => setIsCreateOpen(true)}>
            <HugeiconsIcon icon={UserAdd01Icon} className="size-4 mr-2" />
            Add Department
          </Button>
        </HStack>
      </CardHeader>
      <CardContent>
        <ScrollArea className="h-[400px]">
          <Table>
            <TableHeader>
              <TableRow>
                <TableHead>Department Name</TableHead>
                <TableHead>Description</TableHead>
                <TableHead className="w-[150px]">Actions</TableHead>
              </TableRow>
            </TableHeader>
            <TableBody>
              {isLoading ? (
                <TableRow>
                  <TableCell colSpan={3} className="text-center py-8">
                    <Text muted>Loading...</Text>
                  </TableCell>
                </TableRow>
              ) : !data?.data || data.data.length === 0 ? (
                <TableRow>
                  <TableCell colSpan={3} className="text-center py-8">
                    <Text muted>No departments found</Text>
                  </TableCell>
                </TableRow>
              ) : (
                data.data.map((dept: StaffDepartment) => (
                  <TableRow key={dept.id}>
                    <TableCell className="font-medium">{dept.name}</TableCell>
                    <TableCell>
                      <Text size="sm" muted className="max-w-[300px] truncate">
                        {dept.description || '—'}
                      </Text>
                    </TableCell>
                    <TableCell>
                      <HStack gap={2}>
                        <Button
                          variant="ghost"
                          size="icon"
                          onClick={() => openEdit(dept)}
                        >
                          <HugeiconsIcon icon={PencilEdit01Icon} className="size-4" />
                        </Button>
                        <Button
                          variant="ghost"
                          size="icon"
                          onClick={() => {
                            setDepartmentToDelete(dept.id)
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
          setDepartmentToEdit(null)
          setFormData({
            name: '',
            description: '',
          })
        }
      }}>
        <DialogContent>
          <DialogHeader>
            <DialogTitle>
              {departmentToEdit ? 'Edit Department' : 'Add Department'}
            </DialogTitle>
            <DialogDescription>
              {departmentToEdit
                ? 'Update department information'
                : 'Add new department'}
            </DialogDescription>
          </DialogHeader>
          <Stack gap={4}>
            <div className="grid gap-2">
              <Label htmlFor="name">Department Name</Label>
              <Input
                id="name"
                value={formData.name}
                onChange={(e) =>
                  setFormData((prev) => ({ ...prev, name: e.target.value }))
                }
                placeholder="e.g., Mathematics Department"
                required
              />
            </div>
            <div className="grid gap-2">
              <Label htmlFor="description">Description (optional)</Label>
              <Textarea
                id="description"
                value={formData.description}
                onChange={(e) =>
                  setFormData((prev) => ({ ...prev, description: e.target.value }))
                }
                placeholder="Brief description of the department"
                rows={3}
              />
            </div>
          </Stack>
          <DialogFooter>
            <Button
              variant="outline"
              onClick={() => {
                setIsCreateOpen(false)
                setIsEditOpen(false)
                setDepartmentToEdit(null)
              }}
            >
              Cancel
            </Button>
            <Button onClick={handleSubmit} disabled={createDepartment.isPending || updateDepartment.isPending}>
              {createDepartment.isPending || updateDepartment.isPending ? 'Saving...' : 'Save'}
            </Button>
          </DialogFooter>
        </DialogContent>
      </Dialog>

      {/* Delete Confirmation */}
      <AlertDialog open={isDeleteOpen} onOpenChange={setIsDeleteOpen}>
        <AlertDialogContent>
          <AlertDialogHeader>
            <AlertDialogTitle>Delete Department</AlertDialogTitle>
            <AlertDialogDescription>
              Are you sure you want to delete this department? This action cannot
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
