import { HugeiconsIcon } from '@hugeicons/react'
import { Layers02Icon } from '@hugeicons/core-free-icons'
import { useQuery } from '@tanstack/react-query'
import { Card, CardContent, CardHeader, CardTitle } from '@/components/ui/card'
import { getAllGradeLevelsOptions } from '@/lib/api/@tanstack/react-query.gen'
import { authClient } from '@/lib/clients'

export function GradeLevelsHeader() {
  const { data: gradeLevelsData } = useQuery(
    getAllGradeLevelsOptions({ client: authClient }),
  )

  const totalGradeLevels = gradeLevelsData?.total ?? 0

  return (
    <div className="flex flex-col gap-4 p-8">
      <div className="flex items-center justify-between">
        <div className="flex flex-col gap-1">
          <h1 className="text-3xl font-bold tracking-tight">Grade Levels</h1>
          <p className="text-muted-foreground">
            Manage academic grade levels and their settings.
          </p>
        </div>
      </div>
      <div className="grid auto-rows-min gap-4 md:grid-cols-3">
        <Card>
          <CardHeader className="flex flex-row items-center justify-between pb-2">
            <CardTitle className="text-sm font-medium">
              Total Grade Levels
            </CardTitle>
            <HugeiconsIcon
              icon={Layers02Icon}
              className="size-4 text-muted-foreground"
            />
          </CardHeader>
          <CardContent>
            <div className="text-2xl font-bold">{totalGradeLevels}</div>
            <p className="text-muted-foreground text-xs">
              Overview of all academic grade levels
            </p>
          </CardContent>
        </Card>
      </div>
    </div>
  )
}
