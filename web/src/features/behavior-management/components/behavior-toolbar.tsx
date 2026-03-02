import { Add01Icon } from '@hugeicons/core-free-icons'
import { HugeiconsIcon } from '@hugeicons/react'
import { useBehaviorStore } from '../store'
import { Button } from '@/components/ui/button'
import { HStack } from '@/components/primitives'

export function BehaviorToolbar() {
  const store = useBehaviorStore()

  return (
    <HStack className="justify-end">
      <Button size="sm" onClick={() => store.setIsCreateTypeOpen(true)}>
        <HugeiconsIcon icon={Add01Icon} className="mr-2 size-4" />
        Add Behavior Type
      </Button>
    </HStack>
  )
}
