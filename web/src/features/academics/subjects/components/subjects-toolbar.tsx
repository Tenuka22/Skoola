import { HugeiconsIcon } from '@hugeicons/react'
import {
  Add01Icon,
  Download01Icon,
  Search01Icon,
} from '@hugeicons/core-free-icons'
import { useSubjectsStore } from '../store'
import { Button } from '@/components/ui/button'
import { Input } from '@/components/ui/input'

interface SubjectsToolbarProps {
  onExport: () => void
}

export function SubjectsToolbar({ onExport }: SubjectsToolbarProps) {
  const { search, setSearch, setIsCreateSubjectOpen } = useSubjectsStore()

  return (
    <div className="flex items-center justify-between px-8 py-4">
      <div className="relative">
        <HugeiconsIcon
          icon={Search01Icon}
          className="absolute left-3 top-1/2 -translate-y-1/2 size-4 text-muted-foreground"
        />
        <Input
          placeholder="Search subjects..."
          className="w-72 pl-9"
          value={search}
          onChange={(e) => setSearch(e.target.value)}
        />
      </div>
      <div className="flex items-center gap-2">
        <Button variant="outline" onClick={onExport}>
          <HugeiconsIcon icon={Download01Icon} className="size-4 mr-2" />
          Export
        </Button>
        <Button onClick={() => setIsCreateSubjectOpen(true)}>
          <HugeiconsIcon icon={Add01Icon} className="size-4 mr-2" />
          Add Subject
        </Button>
      </div>
    </div>
  )
}
