import { HugeiconsIcon } from '@hugeicons/react'
import { UserStarIcon } from '@hugeicons/core-free-icons'
import { useQueries } from '@tanstack/react-query'
import { useClassAssignmentsStore } from '../store'
import { Card, CardContent, CardHeader, CardTitle } from '@/components/ui/card'
import { getSubjectsByClassOptions } from '@/lib/api/@tanstack/react-query.gen'
import { authClient } from '@/lib/clients'
import { Grid, HStack, Heading, Stack, Text } from '@/components/primitives'

export function ClassAssignmentsHeader() {
  const { selectedClassId, selectedAcademicYearId } = useClassAssignmentsStore()

  const [assignmentsQuery] = useQueries({
    queries: [
      {
        ...getSubjectsByClassOptions({
          client: authClient,
          path: {
            class_id: selectedClassId ?? '',
            academic_year_id: selectedAcademicYearId ?? '',
          },
        }),
        enabled: !!selectedClassId && !!selectedAcademicYearId,
        staleTime: 5 * 60 * 1000, // 5 minutes
      },
    ],
  })

  const totalAssignments = assignmentsQuery.data?.length ?? 0

  return (
    <Stack gap={4} className="p-8">
      <HStack align="center" className="justify-between">
        <Stack gap={1}>
          <Heading size="h1" className="text-3xl font-bold tracking-tight">
            Class Assignments
          </Heading>
          <Text muted>
            Manage teacher assignments to subjects within classes.
          </Text>
        </Stack>
      </HStack>
      <Grid gap={4} className="auto-rows-min md:grid-cols-3">
        <Card>
          <CardHeader>
            <HStack align="center" className="justify-between pb-2">
              <CardTitle>
                <Text size="sm" className="font-medium">
                  Total Assignments
                </Text>
              </CardTitle>
              <HugeiconsIcon
                icon={UserStarIcon}
                className="size-4 text-muted-foreground"
              />
            </HStack>
          </CardHeader>
          <CardContent>
            <Text size="2xl" className="font-bold">
              {totalAssignments}
            </Text>
            <Text size="xs" muted className="block">
              Assignments for selected class and academic year
            </Text>
          </CardContent>
        </Card>
      </Grid>
    </Stack>
  )
}
