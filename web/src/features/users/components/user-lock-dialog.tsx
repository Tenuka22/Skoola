import { Calendar01Icon } from '@hugeicons/core-free-icons'
import { HugeiconsIcon } from '@hugeicons/react'
import { format } from 'date-fns'
import { useEffect, useState } from 'react'

import type { User } from '../types'
import { Button } from '@/components/ui/button'
import { Calendar } from '@/components/ui/calendar'
import {
  Dialog,
  DialogContent,
  DialogDescription,
  DialogFooter,
  DialogHeader,
  DialogTitle,
} from '@/components/ui/dialog'
import {
  Popover,
  PopoverContent,
  PopoverTrigger,
} from '@/components/ui/popover'
import { cn } from '@/lib/utils'
import { Spinner } from '@/components/ui/spinner'

interface UserLockDialogProps {
  user: User | null
  open: boolean
  onOpenChange: (open: boolean) => void
  onConfirm: (date: Date) => void
  isSubmitting?: boolean
}

export function UserLockDialog({
  user,
  open,
  onOpenChange,
  onConfirm,
  isSubmitting,
}: UserLockDialogProps) {
  const [date, setDate] = useState<Date | undefined>()

  useEffect(() => {
    if (open) {
      setDate(undefined)
    }
  }, [open])

  return (
    <Dialog open={open} onOpenChange={onOpenChange}>
      <DialogContent className="sm:max-w-[425px]">
        <DialogHeader>
          <DialogTitle>Lock User Account</DialogTitle>
          <DialogDescription>
            Select a date until which {user?.email} will be locked out.
          </DialogDescription>
        </DialogHeader>

        <div className="grid gap-4 py-4">
          <Popover>
            <PopoverTrigger
              render={
                <Button
                  variant={'outline'}
                  className={cn(
                    'w-full justify-start text-left font-normal',
                    !date && 'text-muted-foreground',
                  )}
                >
                  <HugeiconsIcon
                    icon={Calendar01Icon}
                    className="mr-2 h-4 w-4"
                  />
                  {date ? format(date, 'PPP') : 'Pick a date'}
                </Button>
              }
            />
            <PopoverContent className="w-auto p-0">
              <Calendar
                mode="single"
                selected={date}
                onSelect={setDate}
                disabled={(date) => date < new Date()}
              />
            </PopoverContent>
          </Popover>
        </div>

        <DialogFooter>
          <Button variant="ghost" onClick={() => onOpenChange(false)}>
            Cancel
          </Button>
          <Button
            onClick={() => date && onConfirm(date)}
            disabled={!date || isSubmitting}
          >
            {isSubmitting && <Spinner className="mr-2 h-4 w-4" />}
            {isSubmitting ? 'Locking...' : 'Lock User'}
          </Button>
        </DialogFooter>
      </DialogContent>
    </Dialog>
  )
}

