import { Heading, Stack, Text } from '@/components/primitives'

export function BehaviorHeader() {
  return (
    <Stack gap={1} className="pb-0">
      <Heading size="h2">Behavior Management</Heading>
      <Text muted as="p">
        Configure behavior incident types and manage student behavior records.
      </Text>
    </Stack>
  )
}
