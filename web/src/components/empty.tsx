import { HugeiconsIcon } from '@hugeicons/react'
import { FileIcon, InboxIcon, SearchIcon } from '@hugeicons/core-free-icons'
import { Heading, Stack, Text } from '@/components/primitives'
import { cn } from '@/lib/utils'

const icons = {
  search: SearchIcon,
  empty: InboxIcon,
  file: FileIcon,
}

interface EmptyProps {
  title: string
  description: string
  icon?: keyof typeof icons
  className?: string
}

export function Empty({
  title,
  description,
  icon = 'empty',
  className,
}: EmptyProps) {
  const Icon = icons[icon]
  return (
    <Stack
      align="center"
      justify="center"
      gap={4}
      className={cn('h-full w-full rounded-lg bg-card p-8', className)}
    >
      <div className="flex size-16 items-center justify-center rounded-full bg-muted">
        <HugeiconsIcon icon={Icon} className="size-8 text-muted-foreground" />
      </div>
      <Stack align="center" gap={1}>
        <Heading as="h3" size="h3" className="font-semibold">
          {title}
        </Heading>
        <Text className="text-muted-foreground">{description}</Text>
      </Stack>
    </Stack>
  )
}
