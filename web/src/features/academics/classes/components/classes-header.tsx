import { HugeiconsIcon } from '@hugeicons/react'
import { SchoolIcon } from '@hugeicons/core-free-icons'
import { useQuery } from '@tanstack/react-query'
import { Card, CardContent, CardHeader, CardTitle } from '@/components/ui/card'
import { getAllClassesOptions } from '@/lib/api/@tanstack/react-query.gen'
import { authClient } from '@/lib/clients'

export function ClassesHeader() {
  const { data: classesData } = useQuery(
    getAllClassesOptions({ client: authClient }),
  )

  const totalClasses = classesData?.total ?? 0

  return (
    <div className="flex flex-col gap-4 p-8">
      <div className="flex items-center justify-between">
        <div className="flex flex-col gap-1">
          <h1 className="text-3xl font-bold tracking-tight">Classes</h1>
          <p className="text-muted-foreground">
            Manage academic classes and their assignments.
          </p>
        </div>
      </div>
      <div className="grid auto-rows-min gap-4 md:grid-cols-3">
        <Card>
          <CardHeader className="flex flex-row items-center justify-between pb-2">
            <CardTitle className="text-sm font-medium">Total Classes</CardTitle>
            <HugeiconsIcon
              icon={SchoolIcon}
              className="size-4 text-muted-foreground"
            />
          </CardHeader>
          <CardContent>
            <div className="text-2xl font-bold">{totalClasses}</div>
            <p className="text-muted-foreground text-xs">
              Overview of all academic classes
            </p>
          </CardContent>
        </Card>
      </div>
    </div>
  )
}
