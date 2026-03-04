import { HugeiconsIcon } from '@hugeicons/react'
import { Note01Icon } from '@hugeicons/core-free-icons'
import { HStack, Stack, Text } from '@/components/primitives'

export function LessonProgressHeader() {
  return (
    <Stack gap={1}>
      <HStack align="center" gap={2}>
        <HugeiconsIcon icon={Note01Icon} className="size-6 text-primary" />
        <Text size="2xl" className="font-bold tracking-tight">
          Lesson Progress (Record Book)
        </Text>
      </HStack>
      <Text muted>
        Track daily teaching progress, topics covered, and student engagement.
      </Text>
    </Stack>
  )
}
