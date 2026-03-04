import { HugeiconsIcon } from '@hugeicons/react'
import { Book01Icon } from '@hugeicons/core-free-icons'
import { HStack, Stack, Text } from '@/components/primitives'
import { Badge } from '@/components/ui/badge'

interface CurriculumHeaderProps {
  total?: number
}

export function CurriculumHeader({ total }: CurriculumHeaderProps) {
  return (
    <Stack gap={1}>
      <HStack align="center" gap={2}>
        <HugeiconsIcon icon={Book01Icon} className="size-6 text-primary" />
        <Text size="2xl" className="font-bold tracking-tight">
          Curriculum Standards
        </Text>
        {total !== undefined && (
          <Badge variant="secondary" className="rounded-full">
            {total} Total
          </Badge>
        )}
      </HStack>
      <Text muted>
        Manage curriculum standards across different subjects, grades, and
        mediums.
      </Text>
    </Stack>
  )
}
