import * as React from 'react'
import { useQuery } from '@tanstack/react-query'
import { HugeiconsIcon } from '@hugeicons/react'
import { Delete02Icon, DownloadIcon, UserAdd01Icon } from '@hugeicons/core-free-icons'
import type { StaffMediaResponse } from '@/lib/api'
import {
  getStaffMediaQueryOptions,
  useCreateStaffMedia,
  useDeleteStaffMedia,
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

interface StaffMediaTabProps {
  staffId?: string
}

export function StaffMediaTab({ staffId }: StaffMediaTabProps) {
  const [isCreateOpen, setIsCreateOpen] = React.useState(false)
  const [isDeleteOpen, setIsDeleteOpen] = React.useState(false)
  const [mediaToDelete, setMediaToDelete] = React.useState<string | null>(null)

  const { data, isLoading } = useQuery({
    ...getStaffMediaQueryOptions({
      client: authClient,
      query: staffId ? { filters: { staff_id: staffId } } : {},
    }),
  })

  const createMedia = useCreateStaffMedia()
  const deleteMedia = useDeleteStaffMedia()

  const [formData, setFormData] = React.useState({
    staff_id: staffId || '',
    media_type: 'Photo',
    file_url: '',
    caption: '',
  })

  const handleSubmit = () => {
    createMedia.mutate(formData, {
      onSuccess: () => {
        setIsCreateOpen(false)
        setFormData({
          staff_id: staffId || '',
          media_type: 'Photo',
          file_url: '',
          caption: '',
        })
      },
    })
  }

  const handleDelete = () => {
    if (mediaToDelete) {
      deleteMedia.mutate(
        { id: mediaToDelete },
        {
          onSuccess: () => {
            setIsDeleteOpen(false)
            setMediaToDelete(null)
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
            <CardTitle>Media Files</CardTitle>
            <CardDescription>
              Manage staff photos and media files
            </CardDescription>
          </Stack>
          <Button size="sm" onClick={() => setIsCreateOpen(true)}>
            <HugeiconsIcon icon={UserAdd01Icon} className="size-4 mr-2" />
            Add Media
          </Button>
        </HStack>
      </CardHeader>
      <CardContent>
        <ScrollArea className="h-[400px]">
          <Table>
            <TableHeader>
              <TableRow>
                <TableHead>Type</TableHead>
                <TableHead>Caption</TableHead>
                <TableHead>Uploaded</TableHead>
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
                    <Text muted>No media files found</Text>
                  </TableCell>
                </TableRow>
              ) : (
                data.data.map((media: StaffMediaResponse) => (
                  <TableRow key={media.staff_id}>
                    <TableCell>
                      <Text className="font-medium">Photo</Text>
                    </TableCell>
                    <TableCell>—</TableCell>
                    <TableCell>
                      <Text size="sm" muted>
                        —
                      </Text>
                    </TableCell>
                    <TableCell>
                      <HStack gap={2}>
                        <Button variant="ghost" size="icon" onClick={() => media.photo_url && window.open(media.photo_url, '_blank')}>
                          <HugeiconsIcon icon={DownloadIcon} className="size-4" />
                        </Button>
                        <Button
                          variant="ghost"
                          size="icon"
                          onClick={() => {
                            setMediaToDelete(media.staff_id)
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
            <DialogTitle>Add Media</DialogTitle>
            <DialogDescription>
              Add a new media file
            </DialogDescription>
          </DialogHeader>
          <Stack gap={4}>
            <div className="grid gap-2">
              <Label htmlFor="type">Media Type</Label>
              <Select
                value={formData.media_type}
                onValueChange={(value) => {
                  if (value) setFormData((prev) => ({ ...prev, media_type: value }))
                }}
              >
                <SelectTrigger>
                  <SelectValue />
                </SelectTrigger>
                <SelectContent>
                  <SelectItem value="Photo">Photo</SelectItem>
                  <SelectItem value="Video">Video</SelectItem>
                  <SelectItem value="Audio">Audio</SelectItem>
                  <SelectItem value="Document">Document</SelectItem>
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
            <div className="grid gap-2">
              <Label htmlFor="caption">Caption (optional)</Label>
              <Input
                id="caption"
                value={formData.caption}
                onChange={(e) =>
                  setFormData((prev) => ({ ...prev, caption: e.target.value }))
                }
              />
            </div>
          </Stack>
          <DialogFooter>
            <Button variant="outline" onClick={() => setIsCreateOpen(false)}>
              Cancel
            </Button>
            <Button onClick={handleSubmit} disabled={createMedia.isPending}>
              {createMedia.isPending ? 'Saving...' : 'Save'}
            </Button>
          </DialogFooter>
        </DialogContent>
      </Dialog>

      {/* Delete Confirmation */}
      <AlertDialog open={isDeleteOpen} onOpenChange={setIsDeleteOpen}>
        <AlertDialogContent>
          <AlertDialogHeader>
            <AlertDialogTitle>Delete Media</AlertDialogTitle>
            <AlertDialogDescription>
              Are you sure you want to delete this media file? This action cannot
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
