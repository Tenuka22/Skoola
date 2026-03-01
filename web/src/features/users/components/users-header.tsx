import { useQuery } from '@tanstack/react-query'
import { useUsersStore } from '../store'

import { Badge } from '@/components/ui/badge'
import { Switch } from '@/components/ui/switch'
import { Label } from '@/components/ui/label'
import { getUserStatisticsOptions } from '@/lib/api/@tanstack/react-query.gen'
import { authClient } from '@/lib/clients'
import { HStack, Heading, Stack, Text } from '@/components/primitives'

export function UsersHeader() {
  const { data: stats } = useQuery(
    getUserStatisticsOptions({
      client: authClient,
    }),
  )
  const { showProfilePictures, setShowProfilePictures } = useUsersStore()

  return (
    <Stack gap={1}>
      <HStack className="justify-between items-start">
        <HStack>
        <Heading size="h2">User management</Heading>
        <Badge
          variant="secondary"
          className="rounded-md bg-muted px-2 py-0.5 text-xs font-normal text-muted-foreground hover:bg-muted"
        >
          {stats?.total_users || 0} Total
        </Badge>
        </HStack>
        <div className="flex items-center space-x-2">
          <Switch 
            id="show-profile-pictures" 
            checked={showProfilePictures} 
            onCheckedChange={setShowProfilePictures} 
          />
          <Label htmlFor="show-profile-pictures" className="text-sm font-medium text-muted-foreground cursor-pointer select-none">
            Show profile pictures
          </Label>
        </div>
      </HStack>
      <Text muted as="p">
        Manage your team members and their account permissions here.
      </Text>
    </Stack>
  )
}
