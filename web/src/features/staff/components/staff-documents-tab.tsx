import * as React from 'react'
import { useQuery } from '@tanstack/react-query'
import { HugeiconsIcon } from '@hugeicons/react'
import { Delete02Icon, DownloadIcon, UserAdd01Icon } from '@hugeicons/core-free-icons'
import { format } from 'date-fns'
import type { StaffDocument } from '@/lib/api'
import {
  getStaffDocumentsQueryOptions,
  useCreateStaffDocument,
  useDeleteStaffDocument,
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

interface StaffDocumentsTabProps {
  staffId?: string
}

export function StaffDocumentsTab({ staffId }: StaffDocumentsTabProps) {
  const [isCreateOpen, setIsCreateOpen] = React.useState(false)
  const [isDeleteOpen, setIsDeleteOpen] = React.useState(false)
  const [documentToDelete, setDocumentToDelete] = React.useState<string | null>(null)

  const { data, isLoading } = useQuery({
    ...getStaffDocumentsQueryOptions({
      client: authClient,
      query: staffId ? { filters: { staff_id: staffId } } : {},
    }),
  })

  const createDocument = useCreateStaffDocument()
  const deleteDocument = useDeleteStaffDocument()

  const [formData, setFormData] = React.useState({
    staff_id: staffId || '',
    doc_type: 'Certificate',
    file_url: '',
  })

  const handleSubmit = () => {
    createDocument.mutate(formData, {
      onSuccess: () => {
        setIsCreateOpen(false)
        setFormData({
          staff_id: staffId || '',
          doc_type: 'Certificate',
          file_url: '',
        })
      },
    })
  }

  const handleDelete = () => {
    if (documentToDelete) {
      deleteDocument.mutate(
        { id: documentToDelete },
        {
          onSuccess: () => {
            setIsDeleteOpen(false)
            setDocumentToDelete(null)
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
            <CardTitle>Documents</CardTitle>
            <CardDescription>
              Manage staff documents and certificates
            </CardDescription>
          </Stack>
          <Button size="sm" onClick={() => setIsCreateOpen(true)}>
            <HugeiconsIcon icon={UserAdd01Icon} className="size-4 mr-2" />
            Add Document
          </Button>
        </HStack>
      </CardHeader>
      <CardContent>
        <ScrollArea className="h-[400px]">
          <Table>
            <TableHeader>
              <TableRow>
                <TableHead>Type</TableHead>
                <TableHead>Uploaded</TableHead>
                <TableHead>Expiry Date</TableHead>
                <TableHead className="w-[150px]">Actions</TableHead>
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
                    <Text muted>No documents found</Text>
                  </TableCell>
                </TableRow>
              ) : (
                data.data.map((doc: StaffDocument) => (
                  <TableRow key={doc.id}>
                    <TableCell className="font-medium">{doc.doc_type}</TableCell>
                    <TableCell>
                      <Badge variant="outline">{doc.doc_type}</Badge>
                    </TableCell>
                    <TableCell>
                      <Text size="sm" muted>
                        {format(new Date(doc.created_at), 'MMM dd, yyyy')}
                      </Text>
                    </TableCell>
                    <TableCell>
                      <Text size="sm" muted className="max-w-[200px] truncate">
                        {doc.expiry_date || '—'}
                      </Text>
                    </TableCell>
                    <TableCell>
                      <HStack gap={2}>
                        <Button variant="ghost" size="icon" onClick={() => window.open(doc.file_url, '_blank')}>
                          <HugeiconsIcon icon={DownloadIcon} className="size-4" />
                        </Button>
                        <Button
                          variant="ghost"
                          size="icon"
                          onClick={() => {
                            setDocumentToDelete(doc.id)
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
            <DialogTitle>Add Document</DialogTitle>
            <DialogDescription>
              Add a new staff document
            </DialogDescription>
          </DialogHeader>
          <Stack gap={4}>
            <div className="grid gap-2">
              <Label htmlFor="type">Document Type</Label>
              <Select
                value={formData.doc_type}
                onValueChange={(value) =>
                  setFormData((prev) => ({ ...prev, doc_type: value ?? 'Certificate' }))
                }
              >
                <SelectTrigger>
                  <SelectValue />
                </SelectTrigger>
                <SelectContent>
                  <SelectItem value="Certificate">Certificate</SelectItem>
                  <SelectItem value="ID">ID</SelectItem>
                  <SelectItem value="License">License</SelectItem>
                  <SelectItem value="Report">Report</SelectItem>
                  <SelectItem value="Other">Other</SelectItem>
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
            <Button onClick={handleSubmit} disabled={createDocument.isPending}>
              {createDocument.isPending ? 'Saving...' : 'Save'}
            </Button>
          </DialogFooter>
        </DialogContent>
      </Dialog>

      {/* Delete Confirmation */}
      <AlertDialog open={isDeleteOpen} onOpenChange={setIsDeleteOpen}>
        <AlertDialogContent>
          <AlertDialogHeader>
            <AlertDialogTitle>Delete Document</AlertDialogTitle>
            <AlertDialogDescription>
              Are you sure you want to delete this document? This action cannot
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
