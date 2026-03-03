import { HugeiconsIcon } from '@hugeicons/react'
import { Add01Icon } from '@hugeicons/core-free-icons'
import { Button } from '@/components/ui/button'

interface TermsToolbarProps {
  setIsCreateTermOpen: (open: boolean) => void
}

export function TermsToolbar({ setIsCreateTermOpen }: TermsToolbarProps) {
  return (
    <div className="flex items-center justify-end px-8 py-4">
      <Button onClick={() => setIsCreateTermOpen(true)}>
        <HugeiconsIcon icon={Add01Icon} className="size-4 mr-2" />
        Add Term
      </Button>
    </div>
  )
}
