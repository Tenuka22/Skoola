import { HugeiconsIcon } from '@hugeicons/react'
import { CalendarAdd01Icon } from '@hugeicons/core-free-icons'
import { useQuery } from '@tanstack/react-query'
import { Card, CardContent, CardHeader, CardTitle } from '@/components/ui/card'
import { getAllAcademicYearsOptions } from '@/lib/api/@tanstack/react-query.gen'
import { authClient } from '@/lib/clients'

export function TermsHeader() {
  const { data: academicYearsData } = useQuery(
    getAllAcademicYearsOptions({ client: authClient }),
  )

  const totalAcademicYears = academicYearsData?.total ?? 0

  return (
    <div className="flex flex-col gap-4 p-8">
      <div className="flex items-center justify-between">
        <div className="flex flex-col gap-1">
          <h1 className="text-3xl font-bold tracking-tight">Terms</h1>
          <p className="text-muted-foreground">
            Manage academic terms within academic years.
          </p>
        </div>
      </div>
      <div className="grid auto-rows-min gap-4 md:grid-cols-3">
        <Card>
          <CardHeader className="flex flex-row items-center justify-between pb-2">
            <CardTitle className="text-sm font-medium">
              Total Academic Years (for context)
            </CardTitle>
            <HugeiconsIcon
              icon={CalendarAdd01Icon}
              className="size-4 text-muted-foreground"
            />
          </CardHeader>
          <CardContent>
            <div className="text-2xl font-bold">{totalAcademicYears}</div>
            <p className="text-muted-foreground text-xs">
              Current count of all academic years
            </p>
          </CardContent>
        </Card>
      </div>
    </div>
  )
}
