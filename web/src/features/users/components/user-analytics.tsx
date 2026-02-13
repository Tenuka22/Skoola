import * as React from 'react'
import { Area, AreaChart, CartesianGrid, Tooltip, XAxis } from 'recharts'
import { HugeiconsIcon } from '@hugeicons/react'
import {
  SecurityCheckIcon,
  UserBlock01Icon,
  UserGroupIcon,
  ZapIcon,
} from '@hugeicons/core-free-icons'
import type {
  ChartConfig} from '@/components/ui/chart';
import type { UserStatsResponse } from '../types'
import {
  ChartContainer,
  ChartTooltipContent
} from '@/components/ui/chart'
import {
  Card,
  CardContent,
  CardDescription,
  CardHeader,
  CardTitle,
} from '@/components/ui/card'

interface UserAnalyticsProps {
  stats?: UserStatsResponse
}

const chartConfig: ChartConfig = {
  count: {
    label: 'New Users',
    color: 'hsl(var(--primary))',
  },
}

export function UserAnalytics({ stats }: UserAnalyticsProps) {
  const chartData = React.useMemo(() => {
    return (
      stats?.registration_trend?.map((t) => ({
        date: t.date,
        count: t.count,
      })) || []
    )
  }, [stats])

  return (
    <div className="grid gap-6 md:grid-cols-2 lg:grid-cols-4">
      {/* Total Users Card */}
      <Card>
        <CardHeader className="flex flex-row items-center justify-between pb-2 space-y-0">
          <CardTitle className="text-sm font-medium">Total Users</CardTitle>
          <HugeiconsIcon icon={UserGroupIcon} className="size-4 text-muted-foreground" />
        </CardHeader>
        <CardContent>
          <div className="text-2xl font-bold">{stats?.total_users || 0}</div>
          <p className="text-xs text-muted-foreground">
            Verified & pending accounts
          </p>
        </CardContent>
      </Card>

      {/* Registration Trend Chart */}
      <Card className="col-span-1 lg:col-span-2">
        <CardHeader>
          <CardTitle className="text-sm font-medium">Registration Trend</CardTitle>
          <CardDescription>
            User registrations over the last 30 days
          </CardDescription>
        </CardHeader>
        <CardContent className="px-2 pt-4 sm:px-6">
          <ChartContainer
            config={chartConfig}
            className="aspect-auto h-[120px] w-full"
          >
            <AreaChart data={chartData}>
              <defs>
                <linearGradient id="fillCount" x1="0" y1="0" x2="0" y2="1">
                  <stop
                    offset="5%"
                    stopColor="var(--color-count)"
                    stopOpacity={0.3}
                  />
                  <stop
                    offset="95%"
                    stopColor="var(--color-count)"
                    stopOpacity={0.01}
                  />
                </linearGradient>
              </defs>
              <CartesianGrid
                vertical={false}
                strokeDasharray="3 3"
                className="stroke-muted/50"
              />
              <XAxis dataKey="date" hide />
              <Tooltip
                content={<ChartTooltipContent hideLabel />}
                cursor={{ stroke: 'hsl(var(--primary))', strokeWidth: 2 }}
              />
              <Area
                dataKey="count"
                type="natural"
                fill="url(#fillCount)"
                fillOpacity={0.4}
                stroke="hsl(var(--primary))"
                strokeWidth={3}
                stackId="a"
              />
            </AreaChart>
          </ChartContainer>
        </CardContent>
      </Card>

      {/* Breakdown Stats */}
      <div className="flex flex-col gap-4">
        <Card>
          <CardContent className="flex items-center gap-4 p-4">
            <HugeiconsIcon icon={SecurityCheckIcon} className="size-5 text-green-500" />
            <div>
              <p className="text-xs font-medium text-muted-foreground">Verified</p>
              <p className="text-xl font-bold">{stats?.verified_users || 0}</p>
            </div>
          </CardContent>
        </Card>

        <Card>
          <CardContent className="flex items-center gap-4 p-4">
            <HugeiconsIcon icon={UserBlock01Icon} className="size-5 text-amber-500" />
            <div>
              <p className="text-xs font-medium text-muted-foreground">Locked</p>
              <p className="text-xl font-bold">{stats?.locked_users || 0}</p>
            </div>
          </CardContent>
        </Card>
      </div>
    </div>
  )
}
