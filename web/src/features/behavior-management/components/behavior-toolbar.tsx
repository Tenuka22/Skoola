import { Add01Icon } from '@hugeicons/core-free-icons'
import { HugeiconsIcon } from '@hugeicons/react'
import { Button } from '@/components/ui/button'
import { HStack } from '@/components/primitives'

interface BehaviorToolbarProps {
  setIsCreateTypeOpen: (open: boolean) => void
}

export function BehaviorToolbar({ setIsCreateTypeOpen }: BehaviorToolbarProps) {
  return (
    <HStack className="justify-end">
      <Button size="sm" onClick={() => setIsCreateTypeOpen(true)}>
        <HugeiconsIcon icon={Add01Icon} className="mr-2 size-4" />
        Add Behavior Type
      </Button>
    </HStack>
  )
}
