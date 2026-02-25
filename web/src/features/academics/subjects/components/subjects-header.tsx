import { HugeiconsIcon } from '@hugeicons/react'
import { Book01Icon } from '@hugeicons/core-free-icons'
import { useQuery } from '@tanstack/react-query'
import { Card, CardContent, CardHeader, CardTitle } from '@/components/ui/card'
import { getAllSubjectsOptions } from '@/lib/api/@tanstack/react-query.gen'
import { authClient } from '@/lib/clients'

export function SubjectsHeader() {
  const { data: subjectsData } = useQuery(
    getAllSubjectsOptions({ client: authClient }),
  )

  const totalSubjects = subjectsData?.total ?? 0

  return (
    <div className="flex flex-col gap-4 p-8">
      <div className="flex items-center justify-between">
        <div className="flex flex-col gap-1">
          <h1 className="text-3xl font-bold tracking-tight">Subjects</h1>
          <p className="text-muted-foreground">
            Manage academic subjects, their assignments to grades/streams, and student enrollments.
          </p>
        </div>
      </div>
      <div className="grid auto-rows-min gap-4 md:grid-cols-3">
        <Card>
          <CardHeader className="flex flex-row items-center justify-between pb-2">
            <CardTitle className="text-sm font-medium">
              Total Subjects
            </CardTitle>
            <HugeiconsIcon
              icon={Book01Icon}
              className="size-4 text-muted-foreground"
            />
          </CardHeader>
          <CardContent>
            <div className="text-2xl font-bold">{totalSubjects}</div>
            <p className="text-muted-foreground text-xs">
              Overview of all academic subjects
            </p>
          </CardContent>
        </Card>
      </div>
    </div>
  )
}
