import * as React from 'react'
import { useQuery } from '@tanstack/react-query'
import { HugeiconsIcon } from '@hugeicons/react'
import { Delete02Icon, DownloadIcon, UserAdd01Icon } from '@hugeicons/core-free-icons'
import { format } from 'date-fns'
import type { StaffCv } from '@/lib/api'
import {
  getStaffCvsQueryOptions,
  useCreateStaffCv,
  useDeleteStaffCv,
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
import {
  Select,
  SelectContent,
  SelectItem,
  SelectTrigger,
  SelectValue,
} from '@/components/ui/select'

interface StaffCvsTabProps {
  staffId?: string
}

export function StaffCvsTab({ staffId }: StaffCvsTabProps) {
  const [isCreateOpen, setIsCreateOpen] = React.useState(false)
  const [isDeleteOpen, setIsDeleteOpen] = React.useState(false)
  const [cvToDelete, setCvToDelete] = React.useState<string | null>(null)

  const { data, isLoading } = useQuery({
    ...getStaffCvsQueryOptions({
      client: authClient,
      query: staffId ? { filters: { staff_id: staffId } } : {},
    }),
  })

  const createCv = useCreateStaffCv()
  const deleteCv = useDeleteStaffCv()

  const [formData, setFormData] = React.useState({
    staff_id: staffId || '',
    file_name: '',
    file_type: 'PDF',
    file_url: '',
  })

  const handleSubmit = () => {
    createCv.mutate(formData, {
      onSuccess: () => {
        setIsCreateOpen(false)
        setFormData({
          staff_id: staffId || '',
          file_name: '',
          file_type: 'PDF',
          file_url: '',
        })
      },
    })
  }

  const handleDelete = () => {
    if (cvToDelete) {
      deleteCv.mutate(
        { id: cvToDelete },
        {
          onSuccess: () => {
            setIsDeleteOpen(false)
            setCvToDelete(null)
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
            <CardTitle>CV Documents</CardTitle>
            <CardDescription>
              Manage staff CV and resume documents
            </CardDescription>
          </Stack>
          <Button size="sm" onClick={() => setIsCreateOpen(true)}>
            <HugeiconsIcon icon={UserAdd01Icon} className="size-4 mr-2" />
            Add CV
          </Button>
        </HStack>
      </CardHeader>
      <CardContent>
        <ScrollArea className="h-[400px]">
          <Table>
            <TableHeader>
              <TableRow>
                <TableHead>File Name</TableHead>
                <TableHead>Type</TableHead>
                <TableHead>Uploaded</TableHead>
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
                    <Text muted>No CV documents found</Text>
                  </TableCell>
                </TableRow>
              ) : (
                data.data.map((cv: StaffCv) => (
                  <TableRow key={cv.id}>
                    <TableCell className="font-medium">{cv.file_name}</TableCell>
                    <TableCell>
                      <Text className="uppercase">{cv.file_type}</Text>
                    </TableCell>
                    <TableCell>
                      <Text size="sm" muted>
                        {format(new Date(cv.uploaded_at), 'MMM dd, yyyy')}
                      </Text>
                    </TableCell>
                    <TableCell>
                      <HStack gap={2}>
                        <Button variant="ghost" size="icon" onClick={() => window.open(cv.file_url, '_blank')}>
                          <HugeiconsIcon icon={DownloadIcon} className="size-4" />
                        </Button>
                        <Button
                          variant="ghost"
                          size="icon"
                          onClick={() => {
                            setCvToDelete(cv.id)
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

      {/* Create Dialog */}
      <Dialog open={isCreateOpen} onOpenChange={setIsCreateOpen}>
        <DialogContent>
          <DialogHeader>
            <DialogTitle>Add CV</DialogTitle>
            <DialogDescription>
              Add a new CV document
            </DialogDescription>
          </DialogHeader>
          <Stack gap={4}>
            <div className="grid gap-2">
              <Label htmlFor="name">File Name</Label>
              <Input
                id="name"
                value={formData.file_name}
                onChange={(e) =>
                  setFormData((prev) => ({ ...prev, file_name: e.target.value }))
                }
                placeholder="e.g., John_Doe_CV.pdf"
                required
              />
            </div>
            <div className="grid gap-2">
              <Label htmlFor="type">File Type</Label>
              <Select
                value={formData.file_type}
                onValueChange={(value) =>
                  setFormData((prev) => ({ ...prev, file_type: value ?? 'PDF' }))
                }
              >
                <SelectTrigger>
                  <SelectValue />
                </SelectTrigger>
                <SelectContent>
                  <SelectItem value="PDF">PDF</SelectItem>
                  <SelectItem value="DOC">DOC</SelectItem>
                  <SelectItem value="DOCX">DOCX</SelectItem>
                  <SelectItem value="TXT">TXT</SelectItem>
                </SelectContent>
              </Select>
            </div>
            <div className="grid gap-2">
              <Label htmlFor="url">File URL</Label>
              <Input
                id="url"
                value={formData.file_url}
                onChange={(e) =>
                  setFormData((prev) => ({ ...prev, file_url: e.target.value }))
                }
                placeholder="https://..."
                required
              />
            </div>
          </Stack>
          <DialogFooter>
            <Button variant="outline" onClick={() => setIsCreateOpen(false)}>
              Cancel
            </Button>
            <Button onClick={handleSubmit} disabled={createCv.isPending}>
              {createCv.isPending ? 'Saving...' : 'Save'}
            </Button>
          </DialogFooter>
        </DialogContent>
      </Dialog>

      {/* Delete Confirmation */}
      <AlertDialog open={isDeleteOpen} onOpenChange={setIsDeleteOpen}>
        <AlertDialogContent>
          <AlertDialogHeader>
            <AlertDialogTitle>Delete CV</AlertDialogTitle>
            <AlertDialogDescription>
              Are you sure you want to delete this CV? This action cannot
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
