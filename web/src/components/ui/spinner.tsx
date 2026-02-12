import { HugeiconsIcon } from '@hugeicons/react'
import type { HugeiconsIconProps } from '@hugeicons/react' // Import HugeiconsIconProps
import { cn } from '@/lib/utils'
import { Loader01Icon } from '@hugeicons/core-free-icons' // Import Loader01Icon

interface SpinnerProps extends HugeiconsIconProps {
  className?: string
}

function Spinner({ className, icon = Loader01Icon, ...props }: SpinnerProps) {
  return (
    <HugeiconsIcon
      role="status"
      aria-label="Loading"
      className={cn('size-4 animate-spin', className)}
      icon={icon} // Use the icon prop
      {...props}
    />
  )
}

export { Spinner }
