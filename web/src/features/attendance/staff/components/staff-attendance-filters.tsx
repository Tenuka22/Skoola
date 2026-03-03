import { format } from 'date-fns'
import { HugeiconsIcon } from '@hugeicons/react'
import {
  Calendar as CalendarIcon,
  Download as DownloadIcon,
} from '@hugeicons/core-free-icons'
import { Button } from '@/components/ui/button'
import { Calendar } from '@/components/ui/calendar'
import { HStack } from '@/components/primitives'
import {
  Popover,
  PopoverContent,
  PopoverTrigger,
} from '@/components/ui/popover'
import { cn } from '@/lib/utils'

export function StaffAttendanceFilters({
  selectedDate,
  onDateChange,
  onExport,
  isExporting,
}: {
  selectedDate: Date
  onDateChange: (date: Date) => void
  onExport: () => void
  isExporting: boolean
}) {
  return (
    <HStack justify="between" className="px-6 py-5 border-b bg-muted/20">
      <HStack gap={3}>
        <Popover>
          <PopoverTrigger>
            <Button
              variant={'outline'}
              className={cn(
                'w-[240px] justify-start text-left font-bold rounded-xl h-10 shadow-sm ring-1 ring-border',
                !selectedDate && 'text-muted-foreground',
              )}
            >
              <HStack gap={2} p={0}>
                <HugeiconsIcon icon={CalendarIcon} className="size-4" />
                <span>
                  {selectedDate ? format(selectedDate, 'PPP') : 'Pick a date'}
                </span>
              </HStack>
            </Button>
          </PopoverTrigger>
          <PopoverContent className="w-auto p-0" align="start">
            <Calendar
              mode="single"
              selected={selectedDate}
              onSelect={(d) => d && onDateChange(d)}
              initialFocus
            />
          </PopoverContent>
        </Popover>
      </HStack>
      <Button
        variant="outline"
        size="sm"
        onClick={onExport}
        disabled={isExporting}
        className="rounded-xl font-bold h-10 px-4"
      >
        <HStack gap={2} p={0}>
          <HugeiconsIcon icon={DownloadIcon} className="size-4" />
          <span>Export CSV</span>
        </HStack>
      </Button>
    </HStack>
  )
}
