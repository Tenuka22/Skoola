import { HugeiconsIcon } from '@hugeicons/react'
import { Book01Icon } from '@hugeicons/core-free-icons'
import { useQuery } from '@tanstack/react-query'
import { Card, CardContent } from '@/components/ui/card'
import { getAllSubjectsOptions } from '@/lib/api/@tanstack/react-query.gen'
import { authClient } from '@/lib/clients'
import { Badge } from '@/components/ui/badge'
import { Grid, HStack, Heading, Stack, Text } from '@/components/primitives'

export function SubjectsHeader() {
  const { data: subjectsData } = useQuery(
    getAllSubjectsOptions({ client: authClient }),
  )

  const totalSubjects = subjectsData?.total ?? 0

  return (
    <Stack gap={4}>
      <Stack gap={1}>
        <HStack align="center" gap={2}>
          <Heading size="h2">Subjects</Heading>
          <Badge
            variant="secondary"
            className="rounded-md bg-muted px-2 py-0.5 text-xs font-normal text-muted-foreground hover:bg-muted"
          >
            {totalSubjects} Total
          </Badge>
        </HStack>
        <Text muted as="p">
          Manage academic subjects, their assignments to grades/streams, and
          student enrollments.
        </Text>
      </Stack>

      <Grid cols={3} gap={4}>
        <Card className="bg-muted/30 border-none shadow-none">
          <CardContent className="p-4">
            <HStack gap={3}>
              <div className="p-2 bg-background rounded-lg">
                <HugeiconsIcon
                  icon={Book01Icon}
                  className="size-5 text-primary"
                />
              </div>
              <Stack gap={0}>
                <Text size="xs" muted>
                  Total Subjects
                </Text>
                <Text size="lg" className="font-bold">
                  {totalSubjects}
                </Text>
              </Stack>
            </HStack>
          </CardContent>
        </Card>
      </Grid>
    </Stack>
  )
}
