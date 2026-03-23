import { Delete02Icon, PencilEdit01Icon } from '@hugeicons/core-free-icons'
import { HugeiconsIcon } from '@hugeicons/react'
import { Button } from '@/components/ui/button'
import { HStack, Text } from '@/components/primitives'

interface StaffFloatingToolbarProps {
  selectedCount: number
  onBulkDelete: () => void
  onBulkEdit: () => void
}

export function StaffFloatingToolbar({
  selectedCount,
  onBulkDelete,
  onBulkEdit,
}: StaffFloatingToolbarProps) {
  return (
    <div className="fixed bottom-4 left-1/2 -translate-x-1/2 z-50">
      <div className="bg-primary text-primary-foreground rounded-lg shadow-lg px-4 py-2">
        <HStack gap={4} align="center">
          <Text size="sm" className="font-medium">
            {selectedCount} selected
          </Text>
          <HStack gap={2}>
            <Button
              variant="secondary"
              size="sm"
              onClick={onBulkEdit}
              className="h-8"
            >
              <HugeiconsIcon icon={PencilEdit01Icon} className="size-4 mr-1" />
              Edit
            </Button>
            <Button
              variant="destructive"
              size="sm"
              onClick={onBulkDelete}
              className="h-8"
            >
              <HugeiconsIcon icon={Delete02Icon} className="size-4 mr-1" />
              Delete
            </Button>
          </HStack>
        </HStack>
      </div>
    </div>
  )
}
