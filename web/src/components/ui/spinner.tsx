import { cn } from '@/lib/utils'
import { HugeiconsIcon, type HugeiconsIconProps } from '@hugeicons/react' // Import HugeiconsIconProps

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
