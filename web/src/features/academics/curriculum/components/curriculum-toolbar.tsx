import { HugeiconsIcon } from '@hugeicons/react'
import { PlusSignIcon, Search01Icon } from '@hugeicons/core-free-icons'
import { Button } from '@/components/ui/button'
import { Input } from '@/components/ui/input'
import { HStack } from '@/components/primitives'

interface CurriculumToolbarProps {
  search: string
  onSearchChange: (value: string) => void
  onAdd: () => void
}

export function CurriculumToolbar({
  search,
  onSearchChange,
  onAdd,
}: CurriculumToolbarProps) {
  return (
    <HStack className="justify-between">
      <div className="relative w-72">
        <HugeiconsIcon
          icon={Search01Icon}
          className="absolute left-3 top-1/2 size-4 -translate-y-1/2 text-muted-foreground"
        />
        <Input
          placeholder="Search standards..."
          value={search}
          onChange={(e) => onSearchChange(e.target.value)}
          className="pl-9"
        />
      </div>
      <Button onClick={onAdd} size="sm" className="gap-2">
        <HugeiconsIcon icon={PlusSignIcon} className="size-4" />
        Add Standard
      </Button>
    </HStack>
  )
}
