import { Heading, Stack, Text } from '@/components/primitives'

export function LessonProgressHeader() {
  return (
    <Stack gap={1}>
      <Heading size="h2">Lesson Progress (Record Book)</Heading>
      <Text muted>
        Track daily teaching progress, topics covered, and student engagement.
      </Text>
    </Stack>
  )
}
