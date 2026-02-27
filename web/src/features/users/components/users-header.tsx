import { useQuery } from '@tanstack/react-query'
import { Badge } from '@/components/ui/badge'
import { getUserStatisticsOptions } from '@/lib/api/@tanstack/react-query.gen'
import { authClient } from '@/lib/clients'
import { Heading, HStack, Stack, Text } from '@/components/primitives'

export function UsersHeader() {
  const { data: stats } = useQuery(
    getUserStatisticsOptions({
      client: authClient,
    }),
  )

  return (
    <Stack gap={1}>
      <HStack >
        <Heading size="h2">
          User management
        </Heading>
        <Badge
          variant="secondary"
          className="rounded-md bg-muted px-2 py-0.5 text-xs font-normal text-muted-foreground hover:bg-muted"
        >
          {stats?.total_users || 0} Total
        </Badge>
      </HStack>
      <Text muted as='p'>
        Manage your team members and their account permissions here.
      </Text>
    </Stack>
  )
}
