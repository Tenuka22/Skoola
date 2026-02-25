import { HugeiconsIcon } from '@hugeicons/react'
import { UserStarIcon } from '@hugeicons/core-free-icons'
import { useQueries } from '@tanstack/react-query'
import { useClassAssignmentsStore } from '../store'
import { Card, CardContent, CardHeader, CardTitle } from '@/components/ui/card'
import { getSubjectsByClassOptions } from '@/lib/api/@tanstack/react-query.gen'
import { authClient } from '@/lib/clients'

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
    <div className="flex flex-col gap-4 p-8">
      <div className="flex items-center justify-between">
        <div className="flex flex-col gap-1">
          <h1 className="text-3xl font-bold tracking-tight">
            Class Assignments
          </h1>
          <p className="text-muted-foreground">
            Manage teacher assignments to subjects within classes.
          </p>
        </div>
      </div>
      <div className="grid auto-rows-min gap-4 md:grid-cols-3">
        <Card>
          <CardHeader className="flex flex-row items-center justify-between pb-2">
            <CardTitle className="text-sm font-medium">
              Total Assignments
            </CardTitle>
            <HugeiconsIcon
              icon={UserStarIcon}
              className="size-4 text-muted-foreground"
            />
          </CardHeader>
          <CardContent>
            <div className="text-2xl font-bold">{totalAssignments}</div>
            <p className="text-muted-foreground text-xs">
              Assignments for selected class and academic year
            </p>
          </CardContent>
        </Card>
      </div>
    </div>
  )
}
