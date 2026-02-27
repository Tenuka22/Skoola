import { HStack, Heading, Stack, Text } from '@/components/primitives'
import { Badge } from '@/components/ui/badge'

interface StaffHeaderProps {
  totalStaff?: number
}

export function StaffHeader({ totalStaff = 0 }: StaffHeaderProps) {
  return (
    <Stack gap={1}>
      <HStack>
        <Heading size="h2">Staff Management</Heading>
        <Badge
          variant="secondary"
          className="rounded-md bg-muted px-2 py-0.5 text-xs font-normal text-muted-foreground hover:bg-muted"
        >
          {totalStaff} Total
        </Badge>
      </HStack>
      <Text muted as="p">
        Manage your school's staff members, teachers, and administrators.
      </Text>
    </Stack>
  )
}
