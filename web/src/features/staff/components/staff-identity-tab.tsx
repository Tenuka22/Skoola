import * as React from 'react'
import { useQuery } from '@tanstack/react-query'
import { HugeiconsIcon } from '@hugeicons/react'
import { Delete02Icon, PencilEdit01Icon, UserAdd01Icon } from '@hugeicons/core-free-icons'
import type { StaffIdentityResponse } from '@/lib/api'
import {
  getStaffIdentityQueryOptions,
  useCreateStaffIdentity,
  useDeleteStaffIdentity,
  useUpdateStaffIdentity,
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

interface StaffIdentityTabProps {
  staffId?: string
}

export function StaffIdentityTab({ staffId }: StaffIdentityTabProps) {
  const [isEditOpen, setIsEditOpen] = React.useState(false)
  const [isDeleteOpen, setIsDeleteOpen] = React.useState(false)
  const [identityToDelete, setIdentityToDelete] = React.useState<string | null>(null)
  const [identityToEdit, setIdentityToEdit] = React.useState<StaffIdentityResponse | null>(null)

  const { data, isLoading } = useQuery({
    ...getStaffIdentityQueryOptions({
      client: authClient,
    }),
  })

  const updateIdentity = useUpdateStaffIdentity()
  const createIdentity = useCreateStaffIdentity()
  const deleteIdentity = useDeleteStaffIdentity()

  const [formData, setFormData] = React.useState({
    staff_id: staffId ?? '',
    nic: '',
  })

  const handleSubmit = () => {
    if (identityToEdit) {
      updateIdentity.mutate(
        { path: { id: identityToEdit.staff_id }, body: formData },
        {
          onSuccess: () => {
            setIsEditOpen(false)
            setIdentityToEdit(null)
          },
        },
      )
    } else {
      createIdentity.mutate(formData, {
        onSuccess: () => {
          setIsEditOpen(false)
        },
      })
    }
  }

  const openEdit = (identity: StaffIdentityResponse) => {
    setIdentityToEdit(identity)
    setFormData({
      staff_id: identity.staff_id,
      nic: identity.nic,
    })
    setIsEditOpen(true)
  }

  const handleDelete = () => {
    if (identityToDelete) {
      deleteIdentity.mutate(
        { id: identityToDelete },
        {
          onSuccess: () => {
            setIsDeleteOpen(false)
            setIdentityToDelete(null)
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
            <CardTitle>Identity Information</CardTitle>
            <CardDescription>
              Manage staff identity and personal details
            </CardDescription>
          </Stack>
          <Button size="sm" onClick={() => setIsEditOpen(true)}>
            <HugeiconsIcon icon={UserAdd01Icon} className="size-4 mr-2" />
            Add Identity
          </Button>
        </HStack>
      </CardHeader>
      <CardContent>
        <ScrollArea className="h-[400px]">
          <Table>
            <TableHeader>
              <TableRow>
                <TableHead>NIC</TableHead>
                <TableHead className="w-[100px]">Actions</TableHead>
              </TableRow>
            </TableHeader>
            <TableBody>
              {isLoading ? (
                <TableRow>
                  <TableCell colSpan={2} className="text-center py-8">
                    <Text muted>Loading...</Text>
                  </TableCell>
                </TableRow>
              ) : !data?.data || data.data.length === 0 ? (
                <TableRow>
                  <TableCell colSpan={2} className="text-center py-8">
                    <Text muted>No identity records found</Text>
                  </TableCell>
                </TableRow>
              ) : (
                data.data.map((identity) => (
                  <TableRow key={identity.staff_id}>
                    <TableCell>{identity.nic}</TableCell>
                    <TableCell>
                      <HStack gap={2}>
                        <Button
                          variant="ghost"
                          size="icon"
                          onClick={() => openEdit(identity)}
                        >
                          <HugeiconsIcon icon={PencilEdit01Icon} className="size-4" />
                        </Button>
                        <Button
                          variant="ghost"
                          size="icon"
                          onClick={() => {
                            setIdentityToDelete(identity.staff_id)
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
      <Dialog
        open={isEditOpen}
        onOpenChange={(open) => {
          if (!open) {
            setIsEditOpen(false)
            setIdentityToEdit(null)
            setFormData({
              staff_id: staffId ?? '',
              nic: '',
            })
          }
        }}
      >
        <DialogContent>
          <DialogHeader>
            <DialogTitle>
              {identityToEdit ? 'Edit Identity' : 'Add Identity'}
            </DialogTitle>
            <DialogDescription>
              {identityToEdit
                ? 'Update identity information'
                : 'Add new identity information'}
            </DialogDescription>
          </DialogHeader>
          <Stack gap={4}>
            <div className="grid gap-2">
              <Label htmlFor="nic">NIC / ID Number</Label>
              <Input
                id="nic"
                value={formData.nic}
                onChange={(e) =>
                  setFormData((prev) => ({ ...prev, nic: e.target.value }))
                }
              />
            </div>
          </Stack>
          <DialogFooter>
            <Button variant="outline" onClick={() => setIsEditOpen(false)}>
              Cancel
            </Button>
            <Button
              onClick={handleSubmit}
              disabled={updateIdentity.isPending || createIdentity.isPending}
            >
              {updateIdentity.isPending || createIdentity.isPending
                ? 'Saving...'
                : 'Save'}
            </Button>
          </DialogFooter>
        </DialogContent>
      </Dialog>

      {/* Delete Confirmation */}
      <AlertDialog open={isDeleteOpen} onOpenChange={setIsDeleteOpen}>
        <AlertDialogContent>
          <AlertDialogHeader>
            <AlertDialogTitle>Delete Identity</AlertDialogTitle>
            <AlertDialogDescription>
              Are you sure you want to delete this identity record? This action
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
