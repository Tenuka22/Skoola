import { Heading, Stack, Text } from '@/components/primitives'

export function ExamsHeader() {
  return (
    <Stack gap={1}>
      <Heading size="h2">Exams & Assessments</Heading>
      <Text muted>Manage school-wide exams, terms, and grading types.</Text>
    </Stack>
  )
}
