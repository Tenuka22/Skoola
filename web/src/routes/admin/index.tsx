import { createFileRoute } from '@tanstack/react-router'
import { HugeiconsIcon } from '@hugeicons/react'
import {
  Analytics01Icon,
  Money03Icon,
  UserGroupIcon,
} from '@hugeicons/core-free-icons'
import { Card, CardContent, CardHeader, CardTitle } from '@/components/ui/card'
import { Grid, HStack, Stack, Text } from '@/components/primitives'

export const Route = createFileRoute('/admin/')({
  component: Dashboard,
})

function Dashboard() {
  return (
    <Stack gap={4}>
      <Grid gap={4} className="auto-rows-min md:grid-cols-3">
        <Card>
          <CardHeader>
            <HStack align="center" className="justify-between pb-2">
              <CardTitle>
                <Text size="sm" className="font-medium">
                  Total Revenue
                </Text>
              </CardTitle>
              <HugeiconsIcon
                icon={Money03Icon}
                className="text-muted-foreground size-4"
              />
            </HStack>
          </CardHeader>
          <CardContent>
            <Text size="2xl" className="font-bold">
              $45,231.89
            </Text>
            <Text size="xs" muted className="block">
              +20.1% from last month
            </Text>
          </CardContent>
        </Card>
        <Card>
          <CardHeader>
            <HStack align="center" className="justify-between pb-2">
              <CardTitle>
                <Text size="sm" className="font-medium">
                  Subscriptions
                </Text>
              </CardTitle>
              <HugeiconsIcon
                icon={UserGroupIcon}
                className="text-muted-foreground size-4"
              />
            </HStack>
          </CardHeader>
          <CardContent>
            <Text size="2xl" className="font-bold">
              +2350
            </Text>
            <Text size="xs" muted className="block">
              +180.1% from last month
            </Text>
          </CardContent>
        </Card>
        <Card>
          <CardHeader>
            <HStack align="center" className="justify-between pb-2">
              <CardTitle>
                <Text size="sm" className="font-medium">
                  Active Now
                </Text>
              </CardTitle>
              <HugeiconsIcon
                icon={Analytics01Icon}
                className="text-muted-foreground size-4"
              />
            </HStack>
          </CardHeader>
          <CardContent>
            <Text size="2xl" className="font-bold">
              +573
            </Text>
            <Text size="xs" muted className="block">
              +201 since last hour
            </Text>
          </CardContent>
        </Card>
      </Grid>
      <Stack className="min-h-[100vh] flex-1 rounded-xl bg-muted/50 md:min-h-min" />
    </Stack>
  )
}
