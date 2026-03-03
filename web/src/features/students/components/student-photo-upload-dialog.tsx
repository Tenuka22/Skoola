import { HugeiconsIcon } from '@hugeicons/react'
import { Upload01Icon } from '@hugeicons/core-free-icons'
import * as React from 'react'
import { useUploadStudentPhoto } from '../api'
import type { StudentResponse } from '@/features/students/types'
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

interface StudentPhotoUploadDialogProps {
  student: StudentResponse | null
  open: boolean
  onOpenChange: (open: boolean) => void
}

export function StudentPhotoUploadDialog({
  student,
  open,
  onOpenChange,
}: StudentPhotoUploadDialogProps) {
  const [selectedFile, setSelectedFile] = React.useState<File | null>(null)

  const uploadPhoto = useUploadStudentPhoto()

  const handleFileChange = (event: React.ChangeEvent<HTMLInputElement>) => {
    if (event.target.files && event.target.files.length > 0) {
      setSelectedFile(event.target.files[0])
    } else {
      setSelectedFile(null)
    }
  }

  const handleSubmit = (event: React.FormEvent) => {
    event.preventDefault()
    if (student && selectedFile) {
      uploadPhoto.mutate(
        {
          path: { student_id: student.id },
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
          <DialogTitle>Upload Photo for {student?.name_english}</DialogTitle>
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
