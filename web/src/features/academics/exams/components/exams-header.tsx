import { HugeiconsIcon } from '@hugeicons/react'
import { Note01Icon } from '@hugeicons/core-free-icons'
import { HStack, Stack, Text } from '@/components/primitives'

export function ExamsHeader() {
  return (
    <Stack gap={1}>
      <HStack align="center" gap={2}>
        <HugeiconsIcon icon={Note01Icon} className="size-6 text-primary" />
        <Text size="2xl" className="font-bold tracking-tight">
          Exams & Assessments
        </Text>
      </HStack>
      <Text muted>Manage school-wide exams, terms, and grading types.</Text>
    </Stack>
  )
}
