import { HugeiconsIcon } from '@hugeicons/react'
import {
  Add01Icon,
  Download01Icon,
  Search01Icon,
} from '@hugeicons/core-free-icons'
import { useAcademicYearsStore } from '../store'
import { Button } from '@/components/ui/button'
import { Input } from '@/components/ui/input'
import { HStack } from '@/components/primitives'

interface AcademicYearsToolbarProps {
  onExport: () => void
}

export function AcademicYearsToolbar({ onExport }: AcademicYearsToolbarProps) {
  const { search, setSearch, setIsCreateYearOpen } = useAcademicYearsStore()

  return (
    <HStack justify="between">
      <div className="relative">
        <HugeiconsIcon
          icon={Search01Icon}
          className="absolute left-3 top-1/2 -translate-y-1/2 size-4 text-muted-foreground"
        />
        <Input
          placeholder="Search academic years..."
          className="w-72 pl-9"
          value={search}
          onChange={(e) => setSearch(e.target.value)}
        />
      </div>
      <HStack gap={2}>
        <Button variant="outline" onClick={onExport}>
          <HugeiconsIcon icon={Download01Icon} className="size-4 mr-2" />
          Export
        </Button>
        <Button onClick={() => setIsCreateYearOpen(true)}>
          <HugeiconsIcon icon={Add01Icon} className="size-4 mr-2" />
          Add Year
        </Button>
      </HStack>
    </HStack>
  )
}
