
import { useQuery } from '@tanstack/react-query';
import { Card, CardContent, CardHeader, CardTitle, CardDescription } from '@/components/ui/card';
import { getFeesReportsCollection9Fa337D81Ca17817997Aa4A0217Fc5A4Options } from '@/lib/api/@tanstack/react-query.gen';
import { Bar, BarChart, CartesianGrid, XAxis, YAxis } from 'recharts';
import {
  ChartConfig,
  ChartContainer,
  ChartTooltip,
  ChartTooltipContent,
  ChartLegend,
  ChartLegendContent
} from '@/components/ui/chart';

interface FinancialReportsProps {
  // TODO: Define props for report data if needed
}

const chartConfig = {
  total_collected: {
    label: "Collected",
    color: "hsl(var(--chart-1))",
  },
  total_expected: {
    label: "Expected",
    color: "hsl(var(--chart-2))",
  },
} satisfies ChartConfig;

export const FinancialReports = ({}: FinancialReportsProps) => {
  const { data: collectionData, isLoading, error } = useQuery({
    ...getFeesReportsCollection9Fa337D81Ca17817997Aa4A0217Fc5A4Options(),
  });

  if (isLoading) {
    return <div className="p-4">Loading financial reports...</div>;
  }

  if (error) {
    return <div className="p-4 text-red-500">Error loading reports: {error.message}</div>;
  }

  // Transform data for chart if necessary, or use directly if it matches
  // collectionData is likely FeeCollectionReport[]
  const chartData = collectionData || [];

  return (
    <div className="space-y-6">
      <h2 className="text-xl font-semibold">Financial Reports</h2>

      <div className="grid grid-cols-1 lg:grid-cols-2 gap-6">
        <Card>
          <CardHeader>
            <CardTitle>Fee Collection by Category</CardTitle>
            <CardDescription>Comparing expected vs. actual collected fees</CardDescription>
          </CardHeader>
          <CardContent>
            {chartData.length > 0 ? (
              <ChartContainer config={chartConfig} className="min-h-[200px] w-full">
                <BarChart accessibilityLayer data={chartData}>
                  <CartesianGrid vertical={false} />
                  <XAxis
                    dataKey="category_name"
                    tickLine={false}
                    tickMargin={10}
                    axisLine={false}
                    tickFormatter={(value) => value.slice(0, 10)} // Truncate long names
                  />
                  <YAxis />
                  <ChartTooltip content={<ChartTooltipContent />} />
                  <ChartLegend content={<ChartLegendContent />} />
                  <Bar dataKey="total_collected" fill="var(--color-total_collected)" radius={4} />
                  <Bar dataKey="total_expected" fill="var(--color-total_expected)" radius={4} />
                </BarChart>
              </ChartContainer>
            ) : (
                <div className="h-[200px] flex items-center justify-center text-muted-foreground">
                    No data available
                </div>
            )}
          </CardContent>
        </Card>

        {/* Add more charts here as needed, e.g., daily collection if API available */}
      </div>
    </div>
  );
};
