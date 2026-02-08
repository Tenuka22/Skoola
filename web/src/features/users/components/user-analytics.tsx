import * as React from 'react'
import {
  Card,
  CardContent,
  CardHeader,
  CardTitle,
  CardDescription,
} from '@/components/ui/card'
import {
  ChartContainer,
  ChartTooltipContent,
  ChartConfig,
} from '@/components/ui/chart'
import {
  Area,
  AreaChart,
  CartesianGrid,
  XAxis,
  Tooltip,
} from 'recharts'
import { HugeiconsIcon } from '@hugeicons/react'
import { UserGroupIcon, SecurityCheckIcon, UserBlock01Icon, ZapIcon } from '@hugeicons/core-free-icons'
import type { UserStatsResponse } from '../types'

interface UserAnalyticsProps {
  stats?: UserStatsResponse
}

const chartConfig: ChartConfig = {
  count: {
    label: "New Users",
    color: "hsl(var(--primary))",
  },
}

export function UserAnalytics({ stats }: UserAnalyticsProps) {
  const chartData = React.useMemo(() => {
    return stats?.registration_trend?.map(t => ({
      date: t.date,
      count: t.count
    })) || []
  }, [stats])

  return (
    <div className="grid gap-6 md:grid-cols-2 lg:grid-cols-4">
      {/* Total Users Hero Card */}
      <Card className="relative overflow-hidden border-none bg-primary text-primary-foreground shadow-2xl shadow-primary/20 transition-all hover:scale-[1.02]">
        <div className="absolute right-[-10%] top-[-10%] size-32 rounded-full bg-white/10 blur-3xl" />
        <CardHeader className="pb-2 pt-6">
          <div className="flex items-center justify-between">
            <p className="text-[10px] font-black uppercase tracking-[0.2em] opacity-80">Identity Pool</p>
            <div className="flex size-8 items-center justify-center rounded-xl bg-white/20 backdrop-blur-md">
               <HugeiconsIcon icon={UserGroupIcon} className="size-4" />
            </div>
          </div>
        </CardHeader>
        <CardContent>
          <div className="flex items-baseline gap-2">
            <h2 className="text-5xl font-black tracking-tighter">{stats?.total_users || 0}</h2>
            <div className="flex items-center text-[10px] font-bold text-white/70">
              <HugeiconsIcon icon={ZapIcon} className="mr-0.5 size-3" />
              Live
            </div>
          </div>
          <p className="mt-2 text-xs font-medium opacity-60">Verified & pending accounts</p>
        </CardContent>
      </Card>

      {/* Registration Velocity Area Chart */}
      <Card className="col-span-1 border-none shadow-xl lg:col-span-2">
        <CardHeader className="flex flex-row items-center justify-between pb-2">
          <div className="space-y-0.5">
            <CardTitle className="text-lg font-black tracking-tight">Growth Velocity</CardTitle>
            <CardDescription className="text-[10px] font-bold uppercase tracking-wider">30-Day Registration Trend</CardDescription>
          </div>
          <div className="rounded-lg bg-muted px-2 py-1 text-[10px] font-black uppercase tracking-widest text-muted-foreground">
            Server Sync
          </div>
        </CardHeader>
        <CardContent className="px-2 pt-4 sm:px-6">
          <ChartContainer config={chartConfig} className="aspect-auto h-[120px] w-full">
            <AreaChart data={chartData}>
              <defs>
                <linearGradient id="fillCount" x1="0" y1="0" x2="0" y2="1">
                  <stop offset="5%" stopColor="var(--color-count)" stopOpacity={0.3} />
                  <stop offset="95%" stopColor="var(--color-count)" stopOpacity={0.01} />
                </linearGradient>
              </defs>
              <CartesianGrid vertical={false} strokeDasharray="3 3" className="stroke-muted/50" />
              <XAxis 
                dataKey="date" 
                hide
              />
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
        <Card className="flex-1 border-none shadow-md transition-colors hover:bg-muted/10">
          <CardContent className="flex items-center gap-4 p-4">
            <div className="flex size-10 shrink-0 items-center justify-center rounded-2xl bg-emerald-500/10 text-emerald-600 dark:text-emerald-400">
               <HugeiconsIcon icon={SecurityCheckIcon} className="size-5" />
            </div>
            <div>
              <p className="text-[10px] font-black uppercase tracking-widest text-muted-foreground">Verified</p>
              <p className="text-xl font-black">{stats?.verified_users || 0}</p>
            </div>
          </CardContent>
        </Card>
        
        <Card className="flex-1 border-none shadow-md transition-colors hover:bg-muted/10">
          <CardContent className="flex items-center gap-4 p-4">
            <div className="flex size-10 shrink-0 items-center justify-center rounded-2xl bg-amber-500/10 text-amber-600 dark:text-amber-400">
               <HugeiconsIcon icon={UserBlock01Icon} className="size-5" />
            </div>
            <div>
              <p className="text-[10px] font-black uppercase tracking-widest text-muted-foreground">Locked</p>
              <p className="text-xl font-black">{stats?.locked_users || 0}</p>
            </div>
          </CardContent>
        </Card>
      </div>
    </div>
  )
}