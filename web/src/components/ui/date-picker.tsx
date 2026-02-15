import { format } from 'date-fns'
import { HugeiconsIcon } from '@hugeicons/react'
import { Calendar01Icon } from '@hugeicons/core-free-icons'

import { Label } from './label'
import { cn } from '@/lib/utils'
import { Button } from '@/components/ui/button'
import { Calendar } from '@/components/ui/calendar'
import {
  Popover,
  PopoverContent,
  PopoverTrigger,
} from '@/components/ui/popover'

interface DatePickerProps {
  label: string
  value: string | null
  onChange: (date: string | null) => void
  className?: string
}

export function DatePicker({
  label,
  value,
  onChange,
  className,
}: DatePickerProps) {
  const selectedDate = value ? new Date(value) : undefined

  return (
    <div className={cn('grid gap-2', className)}>
      <Label htmlFor={label}>{label}</Label>
      <Popover>
        <PopoverTrigger
          render={
            <Button
              variant={'outline'}
              className={cn(
                'w-[200px] justify-start text-left font-normal h-10 rounded-xl border-none bg-muted/50 ring-1 ring-border focus-visible:ring-2 focus-visible:ring-primary shadow-sm',
                !value && 'text-muted-foreground',
              )}
            >
              <HugeiconsIcon icon={Calendar01Icon} className="mr-2 h-4 w-4" />
              {value ? (
                format(selectedDate as Date, 'PPP')
              ) : (
                <span>Pick a date</span>
              )}
            </Button>
          }
        />
        <PopoverContent className="w-auto p-0 rounded-xl bg-background border-none shadow-xl">
          <Calendar
            mode="single"
            selected={selectedDate}
            onSelect={(date) =>
              onChange(date ? format(date, 'yyyy-MM-dd') : null)
            }
            initialFocus
          />
        </PopoverContent>
      </Popover>
    </div>
  )
}
