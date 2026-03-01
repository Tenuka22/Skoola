import { HugeiconsIcon } from '@hugeicons/react'
import { Calendar01Icon } from '@hugeicons/core-free-icons'
import { useQuery } from '@tanstack/react-query'
import { Card, CardContent, CardHeader, CardTitle } from '@/components/ui/card'
import { getAllAcademicYearsOptions } from '@/lib/api/@tanstack/react-query.gen'
import { authClient } from '@/lib/clients'
import { Badge } from '@/components/ui/badge'
import { Grid, HStack, Heading, Stack, Text } from '@/components/primitives'

export function AcademicYearsHeader() {
  const { data: yearsData } = useQuery({
    ...getAllAcademicYearsOptions({ client: authClient }),
  })

  const totalYears = yearsData?.total ?? 0
  const currentYear = yearsData?.data.find((y) => y.current)?.name ?? 'None'

  return (
    <Stack gap={4}>
      <HStack justify="between">
        <Stack gap={1}>
          <HStack align="center" gap={2}>
            <Heading size="h2">Academic Years</Heading>
            <Badge
              variant="secondary"
              className="rounded-md bg-muted px-2 py-0.5 text-xs font-normal text-muted-foreground hover:bg-muted"
            >
              {totalYears} Total
            </Badge>
          </HStack>
          <Text muted as="p">
            Manage academic years and their configuration.
          </Text>
        </Stack>
      </HStack>
      <Grid cols={3} gap={4}>
        <Card>
          <CardHeader>
            <HStack align="center" justify="between" className="pb-2">
              <CardTitle>
                <Text size="sm" className="font-medium">
                  Total Years
                </Text>
              </CardTitle>
              <HugeiconsIcon
                icon={Calendar01Icon}
                className="size-4 text-muted-foreground"
              />
            </HStack>
          </CardHeader>
          <CardContent>
            <Stack gap={1}>
              <Text size="2xl" className="font-bold">
                {totalYears}
              </Text>
              <Text size="xs" muted>
                Academic history overview
              </Text>
            </Stack>
          </CardContent>
        </Card>
        <Card>
          <CardHeader>
            <HStack align="center" justify="between" className="pb-2">
              <CardTitle>
                <Text size="sm" className="font-medium">
                  Current Year
                </Text>
              </CardTitle>
              <HugeiconsIcon
                icon={Calendar01Icon}
                className="size-4 text-muted-foreground"
              />
            </HStack>
          </CardHeader>
          <CardContent>
            <Stack gap={1}>
              <Text size="2xl" className="font-bold">
                {currentYear}
              </Text>
              <Text size="xs" muted>
                Active academic session
              </Text>
            </Stack>
          </CardContent>
        </Card>
      </Grid>
    </Stack>
  )
}
