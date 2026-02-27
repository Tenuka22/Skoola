'use client'

import { HugeiconsIcon } from '@hugeicons/react'
import { Delete01Icon, PencilEdit01Icon } from '@hugeicons/core-free-icons'
import { Button } from '@/components/ui/button'
import { ButtonGroup } from '@/components/ui/button-group'
import { HStack } from '@/components/primitives'

interface StaffBulkActionsToolbarProps {
  selectedStaff: Set<string>
  onBulkDelete: () => void
  onBulkEdit: () => void
}

export function StaffBulkActionsToolbar({
  selectedStaff,
  onBulkDelete,
  onBulkEdit,
}: StaffBulkActionsToolbarProps) {
  const isVisible = selectedStaff.size > 0

  return isVisible ? (
    <div className="fixed bottom-6 left-1/2 z-50 -translate-x-1/2">
      <ButtonGroup>
        <Button variant="outline" disabled>
          {selectedStaff.size} Selected
        </Button>

        <Button variant="outline" onClick={onBulkEdit}>
          <HStack gap={2} p={0}>
            <HugeiconsIcon icon={PencilEdit01Icon} className="size-4" />
            <span>Edit</span>
          </HStack>
        </Button>

        <Button onClick={onBulkDelete} variant="destructive">
          <HStack gap={2} p={0}>
            <HugeiconsIcon icon={Delete01Icon} className="size-4" />
            <span>Delete</span>
          </HStack>
        </Button>
      </ButtonGroup>
    </div>
  ) : null
}
