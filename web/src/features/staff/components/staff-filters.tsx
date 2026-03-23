import * as React from 'react'
import { HugeiconsIcon } from '@hugeicons/react'
import { ArrowLeftIcon } from '@hugeicons/core-free-icons'
import { Button } from '@/components/ui/button'
import { HStack } from '@/components/primitives'
import {
  Select,
  SelectContent,
  SelectItem,
  SelectTrigger,
  SelectValue,
} from '@/components/ui/select'

interface FilterOption {
  label: string
  value: string
}

interface FacetedFilter {
  columnId: string
  title: string
  options: Array<FilterOption>
}

interface StaffFiltersProps {
  facetedFilters: Array<FacetedFilter>
  onClose: () => void
}

export function StaffFilters({ facetedFilters, onClose }: StaffFiltersProps) {
  const [filters, setFilters] = React.useState<Record<string, string>>({})

  const handleFilterChange = (columnId: string, value: string) => {
    setFilters((prev) => ({ ...prev, [columnId]: value }))
  }

  const handleClose = React.useCallback(() => {
    onClose()
  }, [onClose])

  return (
    <HStack gap={4} className="border-t p-4 bg-muted/30">
      <HStack gap={2} className="flex-1">
        {facetedFilters.map((filter) => (
          <Select
            key={filter.columnId}
            value={filters[filter.columnId] || ''}
            onValueChange={(value) => {
              if (value) handleFilterChange(filter.columnId, value)
            }}
          >
            <SelectTrigger className="w-fit min-w-32">
              <SelectValue placeholder={filter.title} />
            </SelectTrigger>
            <SelectContent>
              {filter.options.map((option) => (
                <SelectItem
                  key={option.value}
                  value={option.value}
                  className="capitalize"
                >
                  {option.label}
                </SelectItem>
              ))}
            </SelectContent>
          </Select>
        ))}
      </HStack>
      <Button variant="ghost" size="icon" onClick={handleClose} aria-label="Close filters">
        <HugeiconsIcon icon={ArrowLeftIcon} className="size-4" />
      </Button>
    </HStack>
  )
}
