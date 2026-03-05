import { HugeiconsIcon } from '@hugeicons/react'
import { Calendar01Icon } from '@hugeicons/core-free-icons'
import { useQuery } from '@tanstack/react-query'
import { Card, CardContent } from '@/components/ui/card'
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
          Define and manage your institution's academic timeline, terms, and
          active sessions.
        </Text>
      </Stack>

      <Grid cols={3} gap={4}>
        <Card className="bg-muted/30 border-none shadow-none">
          <CardContent className="p-4">
            <HStack gap={3}>
              <div className="p-2 bg-background rounded-lg">
                <HugeiconsIcon
                  icon={Calendar01Icon}
                  className="size-5 text-primary"
                />
              </div>
              <Stack gap={0}>
                <Text size="xs" muted>
                  Total Records
                </Text>
                <Text size="lg" className="font-bold">
                  {totalYears} Years
                </Text>
              </Stack>
            </HStack>
          </CardContent>
        </Card>

        <Card className="bg-primary/5 border-primary/10 border shadow-none">
          <CardContent className="p-4">
            <HStack gap={3}>
              <div className="p-2 bg-primary/10 rounded-lg">
                <HugeiconsIcon
                  icon={Calendar01Icon}
                  className="size-5 text-primary"
                />
              </div>
              <Stack gap={0}>
                <Text size="xs" className="text-primary/70">
                  Active Session
                </Text>
                <Text size="lg" className="font-bold text-primary">
                  {currentYear}
                </Text>
              </Stack>
            </HStack>
          </CardContent>
        </Card>
      </Grid>
    </Stack>
  )
}
