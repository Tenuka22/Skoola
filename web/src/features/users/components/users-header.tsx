import { Switch } from '@/components/ui/switch'
import { Label } from '@/components/ui/label'
import { HStack, Heading, Stack, Text } from '@/components/primitives'

interface UsersHeaderProps {
  showProfilePictures: boolean
  setShowProfilePictures: (show: boolean) => void
}

export function UsersHeader({
  showProfilePictures,
  setShowProfilePictures,
}: UsersHeaderProps) {
  return (
    <Stack gap={1}>
      <HStack className="justify-between items-start">
        <HStack>
          <Heading size="h2">User management</Heading>
        </HStack>
        <div className="flex items-center space-x-2">
          <Switch
            id="show-profile-pictures"
            checked={showProfilePictures}
            onCheckedChange={setShowProfilePictures}
          />
          <Label
            htmlFor="show-profile-pictures"
            className="text-sm font-medium text-muted-foreground cursor-pointer select-none"
          >
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
