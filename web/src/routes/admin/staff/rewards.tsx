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
import { Box, HStack, Stack, Text, Heading } from '@/components/primitives'
import { DataTable, DataTableColumnHeader } from '@/components/data-table'
import { Badge } from '@/components/ui/badge'
import { Empty } from '@/components/empty'

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
      <Box p={4}>
        <Text size="sm" muted>Loading history...</Text>
      </Box>
    )
    
  if (!history || history.length === 0)
    return (
      <Box p={4}>
        <Empty
          title="No History"
          description="This teacher has no reward point transactions yet."
          icon="empty"
          className="py-4"
        />
      </Box>
    )

  return (
    <Box p={6} bg="muted/20" rounded="lg" className="border border-border/40 shadow-inner">
      <Stack gap={3}>
        <Text size="sm" className="font-bold text-foreground/80 flex items-center gap-2">
          <HugeiconsIcon icon={StarIcon} className="size-4 text-yellow-500" />
          Reward Points History
        </Text>
        <Stack gap={1}>
          {history.map((item: TeacherRewardHistory) => (
            <HStack
              key={item.id}
              justify="between"
              align="center"
              p={3}
              bg="background"
              rounded="md"
              className="border border-border/40 shadow-sm"
            >
              <Stack gap={1}>
                <Text size="sm" className="font-semibold">{item.reason_type}</Text>
                <Text size="xs" muted>{format(new Date(item.created_at), 'PPP p')}</Text>
              </Stack>
              <HStack gap={2} align="center" px={2} py={1} rounded="full" bg={item.points >= 0 ? "emerald-500/10" : "rose-500/10"}>
                <HugeiconsIcon
                  icon={item.points >= 0 ? ArrowUp01Icon : ArrowDown01Icon}
                  className={`size-3.5 ${item.points >= 0 ? 'text-emerald-600' : 'text-rose-600'}`}
                />
                <Text
                  size="sm"
                  className={`font-bold ${item.points >= 0 ? 'text-emerald-700' : 'text-rose-700'}`}
                >
                  {item.points >= 0 ? '+' : ''}
                  {item.points}
                </Text>
              </HStack>
            </HStack>
          ))}
        </Stack>
      </Stack>
    </Box>
  )
}

function TeacherRewardsPage() {
  const [search, setSearch] = React.useState('')
  const staffQuery = useQuery(getAllStaffOptions({ client: authClient }))
  
  const teachers = React.useMemo(
    () =>
      staffQuery.data?.data.filter((s) => s.staff_type === 'Teaching') || [],
    [staffQuery.data],
  )

  const filteredTeachers = React.useMemo(() => {
    if (!search) return teachers
    const s = search.toLowerCase()
    return teachers.filter(t => t.name.toLowerCase().includes(s) || t.employee_id.toLowerCase().includes(s))
  }, [teachers, search])

  const columns = React.useMemo(() => [
    {
      accessorKey: 'name',
      header: ({ column }: any) => <DataTableColumnHeader column={column} title="Teacher Name" />,
      cell: ({ row }: { row: { original: StaffResponse } }) => (
        <Text className="font-semibold">{row.original.name}</Text>
      ),
    },
    {
      accessorKey: 'employee_id',
      header: ({ column }: any) => <DataTableColumnHeader column={column} title="Employee ID" />,
      cell: ({ row }: { row: { original: StaffResponse } }) => (
        <Text size="xs" className="font-mono text-muted-foreground">{row.original.employee_id}</Text>
      ),
    },
    {
      accessorKey: 'reward_points_balance',
      header: ({ column }: any) => <DataTableColumnHeader column={column} title="Current Balance" />,
      cell: ({ row }: { row: { original: StaffResponse } }) => (
        <HStack gap={2} align="center">
          <Box bg="yellow-500/10" p={2} rounded="full">
            <HugeiconsIcon icon={StarIcon} className="size-4 text-yellow-600 shadow-sm" />
          </Box>
          <Text size="lg" className="font-black tabular-nums">
            {row.original.reward_points_balance}
          </Text>
        </HStack>
      ),
    },
    {
      accessorKey: 'employment_status',
      header: ({ column }: any) => <DataTableColumnHeader column={column} title="Status" />,
      cell: ({ row }: { row: { original: StaffResponse } }) => (
        <Badge variant="secondary" className="font-medium">{row.original.employment_status}</Badge>
      ),
    },
  ], [])

  return (
    <Stack gap={6} p={8} className="h-full overflow-hidden">
      <Stack gap={1}>
        <Heading size="h2">Teacher Rewards</Heading>
        <Text muted>
          Monitor performance points and reward balances for teaching staff.
          Click rows to view transaction history.
        </Text>
      </Stack>

      <Box className="flex-1 flex flex-col overflow-hidden min-h-0">
        <DataTable
          columns={columns}
          data={filteredTeachers}
          isLoading={staffQuery.isLoading}
          search={search}
          onSearchChange={setSearch}
          searchPlaceholder="Search teachers..."
          pageIndex={0}
          pageSize={filteredTeachers.length || 10}
          pageCount={1}
          canNextPage={false}
          canPreviousPage={false}
          fetchNextPage={() => {}}
          fetchPreviousPage={() => {}}
          enableExpansion
          renderSubComponent={({ row }) => (
            <Box p={2}>
              <RewardHistoryView teacherId={row.original.id} />
            </Box>
          )}
          emptyState={
            <Empty
              title="No Teachers Found"
              description={search ? "Adjust your search to find teaching staff." : "No teaching staff found in the system."}
              icon="empty"
              className="py-12"
            />
          }
        />
      </Box>
    </Stack>
  )
}
