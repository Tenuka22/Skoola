import * as React from 'react'
import {
  Card,
  CardContent,
  CardHeader,
  CardTitle,
} from '@/components/ui/card'
import {
  ChartContainer,
  ChartTooltip,
  ChartTooltipContent,
  ChartConfig,
} from '@/components/ui/chart'
import {
  Bar,
  BarChart,
  CartesianGrid,
  XAxis,
  Pie,
  PieChart,
  Tooltip,
} from 'recharts'
import { HugeiconsIcon } from '@hugeicons/react'
import { UserGroupIcon } from '@hugeicons/core-free-icons'
import type { UserStatsResponse } from '../types'

interface UserAnalyticsProps {
  stats?: UserStatsResponse
}

const chartConfig: ChartConfig = {
  google: { label: 'Google', color: '#4285F4' },
  github: { label: 'GitHub', color: '#333' },
  password: { label: 'Direct', color: 'hsl(var(--primary))' },
}

export function UserAnalytics({ stats }: UserAnalyticsProps) {
  const authData = React.useMemo(() => {
    if (!stats) return []
    return [
      { name: 'Google', value: stats.auth_methods.google, fill: '#4285F4' },
      { name: 'GitHub', value: stats.auth_methods.github, fill: '#333' },
      { name: 'Password', value: stats.auth_methods.password_only, fill: 'hsl(var(--primary))' },
    ]
  }, [stats])

  return (
    <div className="grid gap-4 md:grid-cols-2 lg:grid-cols-4">
      <Card className="overflow-hidden border-none bg-primary text-primary-foreground shadow-lg transition-transform hover:scale-[1.02]">
        <CardHeader className="pb-2 pt-4">
          <div className="flex items-center justify-between">
            <p className="text-xs font-bold uppercase tracking-widest opacity-70">Total Platform Users</p>
            <HugeiconsIcon icon={UserGroupIcon} className="size-5 opacity-50" />
          </div>
        </CardHeader>
        <CardContent>
          <h2 className="text-4xl font-black">{stats?.total_users || 0}</h2>
          <p className="mt-1 text-xs opacity-60">Global unique user identities</p>
        </CardContent>
      </Card>

      <Card className="col-span-1 lg:col-span-2">
        <CardContent className="pt-6">
          <ChartContainer config={chartConfig} className="h-[120px] w-full">
            <BarChart data={stats?.registration_trend || []}>
              <CartesianGrid vertical={false} strokeDasharray="3 3" opacity={0.2} />
              <XAxis dataKey="date" hide />
              <ChartTooltip content={<ChartTooltipContent />} />
              <Bar dataKey="count" fill="hsl(var(--primary))" radius={[4, 4, 0, 0]} />
            </BarChart>
          </ChartContainer>
          <p className="mt-2 text-center text-[10px] font-bold uppercase tracking-widest text-muted-foreground">30-Day Registration Velocity</p>
        </CardContent>
      </Card>

      <Card>
        <CardHeader className="pb-0 pt-4">
          <CardTitle className="text-xs font-bold uppercase tracking-widest text-muted-foreground">Identity Mix</CardTitle>
        </CardHeader>
        <CardContent>
          <div className="flex items-center justify-between gap-4">
            <ChartContainer config={chartConfig} className="h-[80px] w-[80px]">
              <PieChart>
                <Pie data={authData} dataKey="value" innerRadius={25} outerRadius={35} paddingAngle={5} />
                <Tooltip content={<ChartTooltipContent hideLabel />} />
              </PieChart>
            </ChartContainer>
            <div className="flex flex-col gap-1 text-[10px] font-bold uppercase tracking-tighter">
              <span className="flex items-center gap-1"><div className="size-1.5 rounded-full bg-[#4285F4]" /> Google</span>
              <span className="flex items-center gap-1"><div className="size-1.5 rounded-full bg-[#333]" /> GitHub</span>
              <span className="flex items-center gap-1"><div className="size-1.5 rounded-full bg-primary" /> Direct</span>
            </div>
          </div>
        </CardContent>
      </Card>
    </div>
  )
}