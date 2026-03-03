import { HugeiconsIcon } from '@hugeicons/react'
import { LoadingIcon } from '@hugeicons/core-free-icons'

export function FullPageSpinner() {
  return (
    <div className="flex h-full w-full items-center justify-center">
      <HugeiconsIcon
        icon={LoadingIcon}
        className="h-8 w-8 animate-spin text-primary"
      />
    </div>
  )
}
