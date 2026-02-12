import { HugeiconsIcon } from '@hugeicons/react'
import { Add01Icon, ArrowRight01Icon, FilterIcon } from '@hugeicons/core-free-icons'
import { Button } from '@/components/ui/button'

export function UsersFilters() {
  return (
    <div className="mb-6 flex items-center gap-3 overflow-x-auto px-8 pb-2">
      <div className="flex cursor-pointer items-center gap-1 rounded-lg border border-border/60 bg-background px-3 py-1.5 shadow-sm transition-colors hover:bg-muted/50">
        <HugeiconsIcon
          icon={FilterIcon}
          className="size-3.5 text-muted-foreground"
        />
        <span className="text-xs font-medium">Role</span>
        <HugeiconsIcon
          icon={ArrowRight01Icon}
          className="ml-1 size-3 rotate-90 text-muted-foreground"
        />
      </div>

      <Button
        variant="ghost"
        size="sm"
        className="h-8 gap-1.5 px-2 text-muted-foreground hover:text-foreground"
      >
        <HugeiconsIcon icon={Add01Icon} className="size-3.5" />
        <span className="text-xs font-medium">Add filter</span>
      </Button>
    </div>
  )
}
