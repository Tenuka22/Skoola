import { HugeiconsIcon  } from '@hugeicons/react'
import type {HugeiconsIconProps} from '@hugeicons/react'; // Import HugeiconsIconProps
import { cn } from '@/lib/utils'

interface SpinnerProps extends HugeiconsIconProps {
  className?: string;
}

function Spinner({ className, ...props }: SpinnerProps) {
  return (
    <HugeiconsIcon
      role="status"
      aria-label="Loading"
      className={cn('size-4 animate-spin', className)}
      {...props}
    />
  )
}

export { Spinner }
