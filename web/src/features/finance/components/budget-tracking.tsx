import { useQuery } from '@tanstack/react-query'
import { Bar, BarChart, CartesianGrid, XAxis, YAxis } from 'recharts'
import type { ChartConfig } from '@/components/ui/chart'
import {
  Card,
  CardContent,
  CardDescription,
  CardHeader,
  CardTitle,
} from '@/components/ui/card'
import {
  Table,
  TableBody,
  TableCell,
  TableHead,
  TableHeader,
  TableRow,
} from '@/components/ui/table'
import { getFinancialBudgetsComparison3F6316Da16993A5E1B80490503097A84Options } from '@/lib/api/@tanstack/react-query.gen'
import {
  ChartContainer,
  ChartLegend,
  ChartLegendContent,
  ChartTooltip,
  ChartTooltipContent,
} from '@/components/ui/chart'

interface BudgetTrackingProps {
  year_id?: string
}

const chartConfig = {
  allocated: {
    label: 'Allocated',
    color: 'hsl(var(--chart-1))',
  },
  actual_spent: {
    label: 'Actual Spent',
    color: 'hsl(var(--chart-2))',
  },
} satisfies ChartConfig

export const BudgetTracking = ({
  year_id = 'current',
}: BudgetTrackingProps) => {
  const {
    data: budgetComparison,
    isLoading,
    error,
  } = useQuery({
    ...getFinancialBudgetsComparison3F6316Da16993A5E1B80490503097A84Options({
      path: { year_id },
    }),
  })

  if (isLoading) {
    return <div className="p-4">Loading budget data...</div>
  }

  if (error) {
    return (
      <div className="p-4 text-red-500">
        Error loading budget data: {error.message}
      </div>
    )
  }

  const data = budgetComparison || []

  return (
    <div className="space-y-6">
      <h2 className="text-xl font-semibold">Budget vs Actuals Tracking</h2>

      <div className="grid grid-cols-1 lg:grid-cols-2 gap-6">
        <Card>
          <CardHeader>
            <CardTitle>Budget Overview</CardTitle>
            <CardDescription>
              Allocated vs Actual Spent per Category
            </CardDescription>
          </CardHeader>
          <CardContent>
            {data.length > 0 ? (
              <ChartContainer
                config={chartConfig}
                className="min-h-[300px] w-full"
              >
                <BarChart accessibilityLayer data={data}>
                  <CartesianGrid vertical={false} />
                  <XAxis
                    dataKey="category_name"
                    tickLine={false}
                    tickMargin={10}
                    axisLine={false}
                    tickFormatter={(value) => value.slice(0, 10)}
                  />
                  <YAxis />
                  <ChartTooltip content={<ChartTooltipContent />} />
                  <ChartLegend content={<ChartLegendContent />} />
                  <Bar
                    dataKey="allocated"
                    fill="var(--color-allocated)"
                    radius={4}
                  />
                  <Bar
                    dataKey="actual_spent"
                    fill="var(--color-actual_spent)"
                    radius={4}
                  />
                </BarChart>
              </ChartContainer>
            ) : (
              <div className="h-[300px] flex items-center justify-center text-muted-foreground">
                No budget data available for this year.
              </div>
            )}
          </CardContent>
        </Card>

        <Card>
          <CardHeader>
            <CardTitle>Detailed Breakdown</CardTitle>
            <CardDescription>Variance analysis</CardDescription>
          </CardHeader>
          <CardContent>
            <Table>
              <TableHeader>
                <TableRow>
                  <TableHead>Category</TableHead>
                  <TableHead className="text-right">Budgeted</TableHead>
                  <TableHead className="text-right">Actual</TableHead>
                  <TableHead className="text-right">Variance</TableHead>
                </TableRow>
              </TableHeader>
              <TableBody>
                {data.map((item, index) => (
                  <TableRow key={index}>
                    <TableCell className="font-medium">
                      {item.category_name}
                    </TableCell>
                    <TableCell className="text-right">
                      {item.allocated.toFixed(2)}
                    </TableCell>
                    <TableCell className="text-right">
                      {item.actual_spent.toFixed(2)}
                    </TableCell>
                    <TableCell
                      className={`text-right ${item.variance < 0 ? 'text-red-500' : 'text-green-500'}`}
                    >
                      {item.variance.toFixed(2)}
                    </TableCell>
                  </TableRow>
                ))}
              </TableBody>
            </Table>
          </CardContent>
        </Card>
      </div>
    </div>
  )
}
