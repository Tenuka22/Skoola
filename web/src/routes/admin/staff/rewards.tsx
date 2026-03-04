import { createFileRoute } from '@tanstack/react-router'
import { useQuery } from '@tanstack/react-query'
import * as React from 'react'
import { HugeiconsIcon } from '@hugeicons/react'
import {
  ArrowDown01Icon,
  ArrowUp01Icon,
  StarIcon,
} from '@hugeicons/core-free-icons'
import { format } from 'date-fns'
import type { StaffResponse, TeacherRewardHistory } from '@/lib/api/types.gen'
import { authClient } from '@/lib/clients'
import {
  getAllStaffOptions,
  getTeacherRewardHistoryOptions,
} from '@/lib/api/@tanstack/react-query.gen'
import { Box, HStack, Stack, Text } from '@/components/primitives'
import { DataTable } from '@/components/data-table'
import { Badge } from '@/components/ui/badge'

export const Route = createFileRoute('/admin/staff/rewards')({
  component: TeacherRewardsPage,
})

function RewardHistoryView({ teacherId }: { teacherId: string }) {
  const { data: history, isLoading } = useQuery(
    getTeacherRewardHistoryOptions({
      client: authClient,
      path: { staff_id: teacherId },
    }),
  )

  if (isLoading)
    return (
      <div className="p-4 text-sm text-muted-foreground">
        Loading history...
      </div>
    )
  if (!history || history.length === 0)
    return (
      <div className="p-4 text-sm text-muted-foreground">
        No reward history found.
      </div>
    )

  return (
    <Box className="p-4 bg-muted/30 rounded-md">
      <Text size="sm" className="font-semibold mb-3 block">
        Reward Points History
      </Text>
      <Stack gap={2}>
        {history.map((item: TeacherRewardHistory) => (
          <HStack
            key={item.id}
            justify="between"
            className="text-xs border-b border-border/50 pb-2 last:border-0"
          >
            <Stack gap={1}>
              <Text className="font-medium">{item.reason_type}</Text>
              <Text muted>{format(new Date(item.created_at), 'PPP p')}</Text>
            </Stack>
            <HStack gap={1} align="center">
              <HugeiconsIcon
                icon={item.points >= 0 ? ArrowUp01Icon : ArrowDown01Icon}
                className={`size-3 ${item.points >= 0 ? 'text-green-500' : 'text-red-500'}`}
              />
              <Text
                className={`font-bold ${item.points >= 0 ? 'text-green-500' : 'text-red-500'}`}
              >
                {item.points >= 0 ? '+' : ''}
                {item.points}
              </Text>
            </HStack>
          </HStack>
        ))}
      </Stack>
    </Box>
  )
}

function TeacherRewardsPage() {
  const staffQuery = useQuery(getAllStaffOptions({ client: authClient }))
  const teachers = React.useMemo(
    () =>
      staffQuery.data?.data.filter((s) => s.staff_type === 'Teaching') || [],
    [staffQuery.data],
  )

  const columns = [
    {
      accessorKey: 'name',
      header: 'Teacher Name',
      cell: ({ row }: { row: { original: StaffResponse } }) => (
        <div className="font-medium">{row.original.name}</div>
      ),
    },
    {
      accessorKey: 'employee_id',
      header: 'ID',
      cell: ({ row }: { row: { original: StaffResponse } }) => (
        <div className="font-mono text-xs">{row.original.employee_id}</div>
      ),
    },
    {
      accessorKey: 'reward_points_balance',
      header: 'Current Balance',
      cell: ({ row }: { row: { original: StaffResponse } }) => (
        <HStack gap={1} align="center">
          <HugeiconsIcon icon={StarIcon} className="size-4 text-yellow-500" />
          <span className="font-bold text-lg">
            {row.original.reward_points_balance}
          </span>
        </HStack>
      ),
    },
    {
      accessorKey: 'employment_status',
      header: 'Status',
      cell: ({ row }: { row: { original: StaffResponse } }) => (
        <Badge variant="outline">{row.original.employment_status}</Badge>
      ),
    },
  ]

  return (
    <Stack gap={4} p={8} className="h-full">
      <Stack gap={1}>
        <HStack align="center" gap={2}>
          <HugeiconsIcon icon={StarIcon} className="size-6 text-yellow-500" />
          <Text size="2xl" className="font-bold tracking-tight">
            Teacher Rewards
          </Text>
        </HStack>
        <Text muted>
          Monitor performance points and reward balances for teaching staff.
          Click rows to view history.
        </Text>
      </Stack>

      <Box className="flex-1 overflow-hidden border rounded-xl bg-card">
        <DataTable
          columns={columns}
          data={teachers}
          isLoading={staffQuery.isLoading}
          searchPlaceholder="Search teachers..."
          pageIndex={0}
          pageSize={teachers.length || 10}
          pageCount={1}
          canNextPage={false}
          canPreviousPage={false}
          fetchNextPage={() => {}}
          fetchPreviousPage={() => {}}
          enableExpansion
          renderSubComponent={({ row }) => (
            <RewardHistoryView teacherId={row.original.id} />
          )}
        />
      </Box>
    </Stack>
  )
}
