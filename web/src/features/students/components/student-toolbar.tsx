'use client'

import { HugeiconsIcon } from '@hugeicons/react'
import { Delete02Icon, PencilEdit01Icon } from '@hugeicons/core-free-icons'

import { Button } from '@/components/ui/button'
import { ButtonGroup } from '@/components/ui/button-group'

interface StudentToolbarProps {
  selectedStudents: Set<string>
  onBulkDelete: () => void
  onBulkEdit: () => void
}

export function StudentToolbar({
  selectedStudents,
  onBulkDelete,
  onBulkEdit,
}: StudentToolbarProps) {
  if (selectedStudents.size === 0) return null

  return (
    <div className="fixed bottom-6 left-1/2 z-50 -translate-x-1/2">
      <ButtonGroup>
        <Button variant="outline" disabled>
          {selectedStudents.size} Selected
        </Button>

        <Button variant="outline" onClick={onBulkEdit}>
          <HugeiconsIcon icon={PencilEdit01Icon} className="mr-2 size-4" />
          Edit
        </Button>

        <Button onClick={onBulkDelete} variant="destructive">
          <HugeiconsIcon icon={Delete02Icon} />
          Delete
        </Button>
      </ButtonGroup>
    </div>
  )
}
