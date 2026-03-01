import {
  useMutation,
  useQueryClient,
  useSuspenseQuery,
} from '@tanstack/react-query'
import { format } from 'date-fns'
import { useEffect, useMemo, useState } from 'react'
import { StaffAttendanceActions } from './staff-attendance-actions'
import type { ColumnDef } from '@tanstack/react-table'
import type {
  AttendanceStatus,
  StaffAttendanceResponse,
  StaffResponse,
} from '@/lib/api/types.gen'
import { isAttendanceStatus } from '@/features/attendance/types'
import {
  getAllStaffOptions,
  getStaffAttendanceByDateOptions,
  markStaffAttendanceBulkMutation,
  syncStaffLeavesMutation,
} from '@/lib/api/@tanstack/react-query.gen'
import { Avatar, AvatarFallback, AvatarImage } from '@/components/ui/avatar'
import { HStack, Stack, Text } from '@/components/primitives'
import {
  Select,
  SelectContent,
  SelectItem,
  SelectTrigger,
  SelectValue,
} from '@/components/ui/select'
import { DataTable } from '@/components/ui/data-table'
import { Card, CardContent, CardHeader } from '@/components/ui/card'

type AttendanceState = Record<string, AttendanceStatus>
type EnrichedStaffAttendance = StaffResponse & Partial<StaffAttendanceResponse>

export function StaffAttendanceTable({ date }: { date: Date }) {
  const queryClient = useQueryClient()
  const dateString = format(date, 'yyyy-MM-dd')
  const queryOptions = getStaffAttendanceByDateOptions({
    query: { date: dateString },
  })

  const { data: staffList } = useSuspenseQuery(getAllStaffOptions())
  const { data: attendanceList, isFetching: isAttendanceFetching } =
    useSuspenseQuery(queryOptions)

  const enrichedAttendanceList: Array<EnrichedStaffAttendance> | undefined =
    staffList?.data?.map((staffItem) => {
      const attendanceRecord = attendanceList?.find(
        (ar) => ar.staff_id === staffItem.id,
      )
      return { ...staffItem, ...attendanceRecord }
    })

  const [attendance, setAttendance] = useState<AttendanceState>({})
  const [isDirty, setIsDirty] = useState(false)

  const { mutate: bulkMark, isPending: isSaving } = useMutation({
    ...markStaffAttendanceBulkMutation(),
    onSuccess: () => {
      setIsDirty(false)
      void queryClient.invalidateQueries({ queryKey: queryOptions.queryKey })
    },
  })

  const { mutate: syncLeaves, isPending: isSyncing } = useMutation({
    ...syncStaffLeavesMutation(),
    onSuccess: () => {
      void queryClient.invalidateQueries({ queryKey: queryOptions.queryKey })
    },
  })

  useEffect(() => {
    const initialAttendance: AttendanceState = {}
    if (enrichedAttendanceList) {
      enrichedAttendanceList.forEach((record) => {
        initialAttendance[record.id] = record.status || 'Present'
      })
    }
    setAttendance(initialAttendance)
    setIsDirty(false)
  }, [enrichedAttendanceList])

  const handleStatusChange = (staffId: string, status: AttendanceStatus) => {
    setAttendance((prev: AttendanceState) => ({ ...prev, [staffId]: status }))
    setIsDirty(true)
  }

  const handleBulkMark = (status: AttendanceStatus) => {
    const newAttendance: AttendanceState = {}
    if (enrichedAttendanceList) {
      enrichedAttendanceList.forEach((record) => {
        newAttendance[record.id] = status
      })
    }
    setAttendance(newAttendance)
    setIsDirty(true)
  }

  const handleSave = () => {
    const records = Object.entries(attendance).map(([staff_id, status]) => ({
      staff_id,
      status: status,
      date: dateString,
    }))

    bulkMark({
      body: {
        attendance_records: records,
        date: dateString,
      },
    })
  }

  const handleSyncLeaves = () => {
    syncLeaves({ path: { date: dateString } })
  }

  const columns = useMemo<
    Array<ColumnDef<EnrichedStaffAttendance & { id: string | number }>>
  >(
    () => [
      {
        accessorKey: 'name',
        header: 'Staff Member',
        cell: ({ row }) => (
          <HStack gap={3}>
            <Avatar className="h-9 w-9 rounded-lg">
              <AvatarImage
                src={row.original.photo_url || undefined}
                alt={row.original.name}
              />
              <AvatarFallback className="rounded-lg">
                {row.original.name?.charAt(0)}
              </AvatarFallback>
            </Avatar>
            <Stack gap={0.5}>
              <Text className="font-medium">{row.original.name}</Text>
              <Text
                size="xs"
                muted
                className="font-medium uppercase tracking-tight"
              >
                {row.original.employee_id}
              </Text>
            </Stack>
          </HStack>
        ),
      },
      {
        accessorKey: 'status',
        header: 'Status',
        cell: ({ row }) => (
          <Select
            value={attendance[row.original.id]}
            onValueChange={(status) => {
              if (status && isAttendanceStatus(status)) {
                handleStatusChange(row.original.id, status)
              }
            }}
          >
            <SelectTrigger className="w-[180px] rounded-xl h-9 font-bold ring-1 ring-border">
              <SelectValue placeholder="Set status" />
            </SelectTrigger>
            <SelectContent>
              <SelectItem value="Present">Present</SelectItem>
              <SelectItem value="Absent">Absent</SelectItem>
              <SelectItem value="Excused">Excused</SelectItem>
              <SelectItem value="Late">Late</SelectItem>
              <SelectItem value="HalfDay">Half Day</SelectItem>
              <SelectItem value="SchoolBusiness">School Business</SelectItem>
            </SelectContent>
          </Select>
        ),
      },
    ],
    [attendance],
  )

  const dataForTable = useMemo(() => {
    return (enrichedAttendanceList || []).map((item) => ({
      ...item,
      id: item.id,
    }))
  }, [enrichedAttendanceList])

  return (
    <Card className="border-none shadow-xl overflow-hidden bg-card">
      <CardHeader className="p-0">
        <Box className="px-6 py-5 border-b bg-muted/20">
          <StaffAttendanceActions
            onBulkMark={handleBulkMark}
            onSave={handleSave}
            onSyncLeaves={handleSyncLeaves}
            isSaving={isSaving}
            isSyncing={isSyncing}
            isDirty={isDirty}
          />
        </Box>
      </CardHeader>
      <CardContent className="p-0">
        <DataTable
          columns={columns}
          data={dataForTable}
          isLoading={isAttendanceFetching}
          pageIndex={0}
          pageSize={dataForTable.length}
          pageCount={1}
          canNextPage={false}
          canPreviousPage={false}
          fetchNextPage={() => {}}
          fetchPreviousPage={() => {}}
        />
      </CardContent>
    </Card>
  )
}
