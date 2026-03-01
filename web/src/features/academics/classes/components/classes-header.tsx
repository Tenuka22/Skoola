import { HugeiconsIcon } from '@hugeicons/react'
import { SchoolIcon } from '@hugeicons/core-free-icons'
import { useQuery } from '@tanstack/react-query'
import { Card, CardContent, CardHeader, CardTitle } from '@/components/ui/card'
import { getAllClassesOptions } from '@/lib/api/@tanstack/react-query.gen'
import { authClient } from '@/lib/clients'
import { Badge } from '@/components/ui/badge'
import { Grid, HStack, Heading, Stack, Text } from '@/components/primitives'

export function ClassesHeader() {
  const { data: classesData } = useQuery(
    getAllClassesOptions({ client: authClient }),
  )

  const totalClasses = classesData?.total ?? 0

  return (
    <Stack gap={4}>
      <HStack justify="between">
        <Stack gap={1}>
          <HStack align="center" gap={2}>
            <Heading size="h2">Classes</Heading>
            <Badge
              variant="secondary"
              className="rounded-md bg-muted px-2 py-0.5 text-xs font-normal text-muted-foreground hover:bg-muted"
            >
              {totalClasses} Total
            </Badge>
          </HStack>
          <Text muted as="p">
            Manage academic classes and their assignments.
          </Text>
        </Stack>
      </HStack>
      <Grid cols={3} gap={4}>
        <Card>
          <CardHeader>
            <HStack align="center" justify="between" className="pb-2">
              <CardTitle>
                <Text size="sm" className="font-medium">
                  Total Classes
                </Text>
              </CardTitle>
              <HugeiconsIcon
                icon={SchoolIcon}
                className="size-4 text-muted-foreground"
              />
            </HStack>
          </CardHeader>
          <CardContent>
            <Stack gap={1}>
              <Text size="2xl" className="font-bold">
                {totalClasses}
              </Text>
              <Text size="xs" muted>
                Overview of all academic classes
              </Text>
            </Stack>
          </CardContent>
        </Card>
      </Grid>
    </Stack>
  )
}
