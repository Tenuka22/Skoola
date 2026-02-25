import { HugeiconsIcon } from '@hugeicons/react'
import { Add01Icon } from '@hugeicons/core-free-icons'
import { useTermsStore } from '../store'
import { Button } from '@/components/ui/button'

interface TermsToolbarProps {}

export function TermsToolbar(_props: TermsToolbarProps) {
  const { setIsCreateTermOpen } = useTermsStore()

  return (
    <div className="flex items-center justify-end px-8 py-4">
      <Button onClick={() => setIsCreateTermOpen(true)}>
        <HugeiconsIcon icon={Add01Icon} className="size-4 mr-2" />
        Add Term
      </Button>
    </div>
  )
}
