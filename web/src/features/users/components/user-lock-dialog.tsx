import { Calendar01Icon, LockIcon } from '@hugeicons/core-free-icons'
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
      <DialogContent className="max-w-md rounded-[2.5rem] border-none p-10 shadow-2xl backdrop-blur-3xl ring-1 ring-white/20">
        <DialogHeader>
          <div className="mx-auto mb-4 flex size-20 items-center justify-center rounded-3xl bg-destructive/10 text-destructive">
            <HugeiconsIcon icon={LockIcon} className="size-10" />
          </div>
          <DialogTitle className="text-center text-3xl font-black tracking-tight">
            Lock User Account
          </DialogTitle>
          <DialogDescription className="text-center text-base font-medium leading-relaxed opacity-70">
            Select a date until which {user?.email} will be locked out.
          </DialogDescription>
        </DialogHeader>

        <div className="flex flex-col gap-4 py-4">
          <Popover>
            <PopoverTrigger render={<Button
              variant={'outline'}
              className={cn(
                'w-full justify-start text-left font-normal',
                !date && 'text-muted-foreground',
              )}
            >
              <HugeiconsIcon icon={Calendar01Icon} className="mr-2 h-4 w-4" />
              {date ? format(date, 'PPP') : 'Pick a date'}
            </Button>
          }>
              </PopoverTrigger>
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

        <DialogFooter className="gap-3 sm:justify-center">
          <Button
            variant="ghost"
            onClick={() => onOpenChange(false)}
            className="h-14 min-w-[120px] rounded-2xl border-none bg-muted/50 font-black uppercase tracking-widest hover:bg-muted"
          >
            Cancel
          </Button>
          <Button
            onClick={() => date && onConfirm(date)}
            disabled={!date || isSubmitting}
            className="h-14 min-w-[160px] rounded-2xl bg-destructive font-black uppercase tracking-widest text-destructive-foreground shadow-2xl shadow-destructive/20 transition-all hover:bg-destructive/90 active:scale-95"
          >
            {isSubmitting ? 'Locking...' : 'Lock User'}
          </Button>
        </DialogFooter>
      </DialogContent>
    </Dialog>
  )
}
