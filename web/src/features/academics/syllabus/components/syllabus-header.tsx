import { HugeiconsIcon } from '@hugeicons/react'
import { StructureIcon } from '@hugeicons/core-free-icons'
import type { CurriculumStandardResponse } from '@/lib/api/types.gen'
import { HStack, Stack, Text } from '@/components/primitives'

interface SyllabusHeaderProps {
  standard?: CurriculumStandardResponse
}

export function SyllabusHeader({ standard }: SyllabusHeaderProps) {
  return (
    <Stack gap={1}>
      <HStack align="center" gap={2}>
        <HugeiconsIcon icon={StructureIcon} className="size-6 text-primary" />
        <Text size="2xl" className="font-bold tracking-tight">
          Syllabus Hierarchical
        </Text>
      </HStack>
      {standard && (
        <Text muted>
          Managing topics for standard:{' '}
          <span className="font-semibold text-foreground">
            {standard.standard_code}
          </span>{' '}
          ({standard.version_name})
        </Text>
      )}
    </Stack>
  )
}
