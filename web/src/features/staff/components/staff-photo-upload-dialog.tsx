import { HugeiconsIcon } from '@hugeicons/react'
import { Upload01Icon } from '@hugeicons/core-free-icons'
import * as React from 'react'
import { useUploadStaffPhoto } from '../api'
import type { StaffResponse } from '@/lib/api'
import { Button } from '@/components/ui/button'
import {
  Dialog,
  DialogContent,
  DialogFooter,
  DialogHeader,
  DialogTitle,
} from '@/components/ui/dialog'
import { Label } from '@/components/ui/label'
import { Spinner } from '@/components/ui/spinner'
import { Input } from '@/components/ui/input'

interface StaffPhotoUploadDialogProps {
  staff: StaffResponse | null
  open: boolean
  onOpenChange: (open: boolean) => void
}

export function StaffPhotoUploadDialog({
  staff,
  open,
  onOpenChange,
}: StaffPhotoUploadDialogProps) {
  const [selectedFile, setSelectedFile] = React.useState<File | null>(null)

  const uploadPhoto = useUploadStaffPhoto()

  const handleFileChange = (event: React.ChangeEvent<HTMLInputElement>) => {
    if (event.target.files && event.target.files.length > 0) {
      setSelectedFile(event.target.files[0])
    } else {
      setSelectedFile(null)
    }
  }

  const handleSubmit = (event: React.FormEvent) => {
    event.preventDefault()
    if (staff && selectedFile) {
      uploadPhoto.mutate(
        {
          path: { staff_id: staff.id },
          body: {
            photo: selectedFile,
          },
        },
        {
          onSuccess: () => {
            onOpenChange(false)
            setSelectedFile(null)
          },
        },
      )
    }
  }

  return (
    <Dialog
      open={open}
      onOpenChange={(val) => {
        if (!val) setSelectedFile(null)
        onOpenChange(val)
      }}
    >
      <DialogContent>
        <DialogHeader>
          <DialogTitle>Upload Photo for {staff?.name}</DialogTitle>
        </DialogHeader>
        <form onSubmit={handleSubmit} className="grid gap-4 py-4">
          <div className="grid grid-cols-4 items-center gap-4">
            <Label htmlFor="photo" className="text-right">
              Photo File
            </Label>
            <Input
              id="photo"
              type="file"
              accept="image/*"
              onChange={handleFileChange}
              className="col-span-3"
            />
          </div>
          <DialogFooter className="mt-4">
            <Button
              type="button"
              variant="ghost"
              onClick={() => onOpenChange(false)}
            >
              Cancel
            </Button>
            <Button
              type="submit"
              disabled={!selectedFile || uploadPhoto.isPending}
            >
              {uploadPhoto.isPending ? (
                <Spinner className="mr-2" />
              ) : (
                <HugeiconsIcon icon={Upload01Icon} className="size-4 mr-2" />
              )}
              Upload Photo
            </Button>
          </DialogFooter>
        </form>
      </DialogContent>
    </Dialog>
  )
}
