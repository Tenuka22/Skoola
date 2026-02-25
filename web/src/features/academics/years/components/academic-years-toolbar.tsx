'use client'

import { HugeiconsIcon } from '@hugeicons/react'
import { Download01Icon, Search01Icon } from '@hugeicons/core-free-icons'
import { useAcademicYearsStore } from '../store'
import { Button } from '@/components/ui/button'
import {
  InputGroup,
  InputGroupInput,
  InputGroupText,
} from '@/components/ui/input-group'

interface AcademicYearsToolbarProps {
  onExport: () => void
}

export function AcademicYearsToolbar({ onExport }: AcademicYearsToolbarProps) {
  const { search, setSearch } = useAcademicYearsStore()

  return (
    <div className="flex items-center justify-between p-4">
      <div className="w-full max-w-sm">
        <InputGroup>
          <InputGroupText>
            <HugeiconsIcon icon={Search01Icon} className="size-4" />
          </InputGroupText>
          <InputGroupInput
            placeholder="Search by name or year..."
            value={search}
            onChange={(e) => setSearch(e.target.value)}
          />
        </InputGroup>
      </div>
      <Button variant="outline" onClick={onExport}>
        <HugeiconsIcon icon={Download01Icon} className="mr-2 size-4" />
        Export
      </Button>
    </div>
  )
}
