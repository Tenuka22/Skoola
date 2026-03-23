import * as React from 'react'
import { useQuery } from '@tanstack/react-query'
import { HugeiconsIcon } from '@hugeicons/react'
import { Delete02Icon, PencilEdit01Icon, UserAdd01Icon } from '@hugeicons/core-free-icons'
import type { StaffContactResponse } from '@/lib/api'
import {
  getStaffContactsQueryOptions,
  useCreateStaffContact,
  useDeleteStaffContact,
  useUpdateStaffContact,
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

interface StaffContactsTabProps {
  staffId?: string
}

export function StaffContactsTab({ staffId }: StaffContactsTabProps) {
  const [isCreateOpen, setIsCreateOpen] = React.useState(false)
  const [isEditOpen, setIsEditOpen] = React.useState(false)
  const [isDeleteOpen, setIsDeleteOpen] = React.useState(false)
  const [contactToEdit, setContactToEdit] = React.useState<StaffContactResponse | null>(null)
  const [contactToDelete, setContactToDelete] = React.useState<string | null>(null)

  const { data, isLoading } = useQuery({
    ...getStaffContactsQueryOptions({
      client: authClient,
    }),
  })

  const createContact = useCreateStaffContact()
  const updateContact = useUpdateStaffContact()
  const deleteContact = useDeleteStaffContact()

  const [formData, setFormData] = React.useState<{
    staff_id: string
    email: string
    phone: string
    address: string
    address_latitude: number | null
    address_longitude: number | null
  }>({
    staff_id: staffId ?? '',
    email: '',
    phone: '',
    address: '',
    address_latitude: null,
    address_longitude: null,
  })

  const handleSubmit = () => {
    if (contactToEdit) {
      updateContact.mutate(
        { path: { id: contactToEdit.staff_id }, body: formData },
        {
          onSuccess: () => {
            setIsEditOpen(false)
            setContactToEdit(null)
          },
        },
      )
    } else {
      createContact.mutate(formData, {
        onSuccess: () => {
          setIsCreateOpen(false)
          setFormData({
            staff_id: staffId ?? '',
            email: '',
            phone: '',
            address: '',
            address_latitude: null,
            address_longitude: null,
          })
        },
      })
    }
  }

  const openEdit = (contact: StaffContactResponse) => {
    setContactToEdit(contact)
    setFormData({
      staff_id: contact.staff_id,
      email: contact.email,
      phone: contact.phone,
      address: contact.address,
      address_latitude: contact.address_latitude ?? null,
      address_longitude: contact.address_longitude ?? null,
    })
    setIsEditOpen(true)
  }

  const handleDelete = () => {
    if (contactToDelete) {
      deleteContact.mutate(
        { id: contactToDelete },
        {
          onSuccess: () => {
            setIsDeleteOpen(false)
            setContactToDelete(null)
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
            <CardTitle>Contact Information</CardTitle>
            <CardDescription>
              Manage staff contact details and addresses
            </CardDescription>
          </Stack>
          <Button size="sm" onClick={() => setIsCreateOpen(true)}>
            <HugeiconsIcon icon={UserAdd01Icon} className="size-4 mr-2" />
            Add Contact
          </Button>
        </HStack>
      </CardHeader>
      <CardContent>
        <ScrollArea className="h-[400px]">
          <Table>
            <TableHeader>
              <TableRow>
                <TableHead>Email</TableHead>
                <TableHead>Phone</TableHead>
                <TableHead>Address</TableHead>
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
                    <Text muted>No contacts found</Text>
                  </TableCell>
                </TableRow>
              ) : (
                data.data.map((contact) => (
                  <TableRow key={contact.staff_id}>
                    <TableCell>{contact.email}</TableCell>
                    <TableCell>{contact.phone}</TableCell>
                    <TableCell>{contact.address}</TableCell>
                    <TableCell>
                      <HStack gap={2}>
                        <Button
                          variant="ghost"
                          size="icon"
                          onClick={() => openEdit(contact)}
                        >
                          <HugeiconsIcon icon={PencilEdit01Icon} className="size-4" />
                        </Button>
                        <Button
                          variant="ghost"
                          size="icon"
                          onClick={() => {
                            setContactToDelete(contact.staff_id)
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
        open={isCreateOpen || isEditOpen}
        onOpenChange={(open) => {
          if (!open) {
            setIsCreateOpen(false)
            setIsEditOpen(false)
            setContactToEdit(null)
            setFormData({
              staff_id: staffId ?? '',
              email: '',
              phone: '',
              address: '',
              address_latitude: null,
              address_longitude: null,
            })
          }
        }}
      >
        <DialogContent>
          <DialogHeader>
            <DialogTitle>
              {contactToEdit ? 'Edit Contact' : 'Add Contact'}
            </DialogTitle>
            <DialogDescription>
              {contactToEdit
                ? 'Update contact information'
                : 'Add new contact information'}
            </DialogDescription>
          </DialogHeader>
          <Stack gap={4}>
            <div className="grid gap-2">
              <Label htmlFor="email">Email</Label>
              <Input
                id="email"
                type="email"
                value={formData.email}
                onChange={(e) =>
                  setFormData((prev) => ({ ...prev, email: e.target.value }))
                }
                required
              />
            </div>
            <div className="grid gap-2">
              <Label htmlFor="phone">Phone</Label>
              <Input
                id="phone"
                value={formData.phone}
                onChange={(e) =>
                  setFormData((prev) => ({ ...prev, phone: e.target.value }))
                }
                required
              />
            </div>
            <div className="grid gap-2">
              <Label htmlFor="address">Address</Label>
              <Input
                id="address"
                value={formData.address}
                onChange={(e) =>
                  setFormData((prev) => ({ ...prev, address: e.target.value }))
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
                setContactToEdit(null)
              }}
            >
              Cancel
            </Button>
            <Button
              onClick={handleSubmit}
              disabled={createContact.isPending || updateContact.isPending}
            >
              {createContact.isPending || updateContact.isPending
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
            <AlertDialogTitle>Delete Contact</AlertDialogTitle>
            <AlertDialogDescription>
              Are you sure you want to delete this contact? This action cannot be
              undone.
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
