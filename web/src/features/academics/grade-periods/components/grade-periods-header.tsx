import { Heading, HStack, Stack, Text } from '@/components/primitives'

interface GradePeriodsHeaderProps {
  gradeName?: string
  total?: number
}

export function GradePeriodsHeader({ gradeName, total }: GradePeriodsHeaderProps) {
  return (
    <HStack justify="between" align="center">
      <Stack gap={1}>
        <HStack align="center" gap={2}>
          <Heading size="h2">Grade Periods</Heading>
          {gradeName && (
            <Text muted size="xl">
              for {gradeName}
            </Text>
          )}
          {total !== undefined && (
            <div className="rounded-md bg-muted px-2 py-0.5 text-xs font-normal text-muted-foreground">
              {total} Total
            </div>
          )}
        </HStack>
        <Text muted>
          Define and manage the daily schedule structure for this grade level.
        </Text>
      </Stack>
    </HStack>
  )
}
