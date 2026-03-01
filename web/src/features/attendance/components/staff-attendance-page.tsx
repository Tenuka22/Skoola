'use client'

import * as React from 'react'
import { addDays, format, isFuture, isToday, subDays } from 'date-fns'
import { HugeiconsIcon } from '@hugeicons/react'
import {
  ArrowLeft01Icon,
  ArrowRight01Icon,
  Calendar01Icon,
  Download02Icon,
  FilterIcon,
  PlusSignIcon,
  RefreshIcon,
  Search01Icon,
} from '@hugeicons/core-free-icons'
import { useStaffAttendance, useStaffList } from '../api'
import { staffAttendanceColumns } from './staff-attendance-columns'
import { AttendanceSummaryCards } from './attendance-summary-cards'
import { MarkStaffAttendanceDialog } from './mark-staff-attendance-dialog'
import type { StaffAttendanceWithMember } from '../types'
import type { StaffAttendanceResponse } from '@/lib/api/types.gen'
import { DataTable } from '@/components/ui/data-table'
import {
  Select,
  SelectContent,
  SelectItem,
  SelectTrigger,
  SelectValue,
} from '@/components/ui/select'
import { Input } from '@/components/ui/input'
import { Button } from '@/components/ui/button'
import { Card, CardContent, CardHeader } from '@/components/ui/card'
import { Spinner } from '@/components/ui/spinner'
import { Box, HStack, Heading, Stack, Text } from '@/components/primitives'

export const StaffAttendancePage = () => {
  const [date, setDate] = React.useState(new Date())
  const [search, setSearch] = React.useState('')
  const [selectedAttendance, setSelectedAttendance] =
    React.useState<StaffAttendanceWithMember | null>(null)
  const [isDialogOpen, setIsDialogOpen] = React.useState(false)
  const formattedDateForApi = format(date, 'yyyy-MM-dd')
  const formattedDateForDisplay = format(date, 'EEEE, dd MMMM')

  const { data: attendanceData, refetch: refetchAttendance } =
    useStaffAttendance(formattedDateForApi)
  const { data: staffData, isLoading: isStaffLoading } = useStaffList()

  const handlePrevDay = () => {
    setDate((d) => subDays(d, 1))
  }

  const handleNextDay = () => {
    if (!isToday(date)) {
      setDate((d) => addDays(d, 1))
    }
  }

  const handleMarkAttendance = (attendance: StaffAttendanceWithMember) => {
    setSelectedAttendance(attendance)
    setIsDialogOpen(true)
  }

  const columns = React.useMemo(() => {
    return staffAttendanceColumns.map((col) => ({
      ...col,
      meta: {
        onMarkAttendance: handleMarkAttendance,
      },
    }))
  }, [])

  const mergedData = React.useMemo(() => {
    if (!staffData?.data) return []
    return staffData.data.map((staff) => {
      const attendance: StaffAttendanceResponse | undefined =
        attendanceData?.find((a) => a.staff_id === staff.id)

      const defaultAttendance: StaffAttendanceResponse = {
        id: `temp-${staff.id}`,
        staff_id: staff.id,
        date: formattedDateForApi,
        status: 'Absent',
        created_at: new Date().toISOString(),
        updated_at: new Date().toISOString(),
        time_in: null,
        time_out: null,
        remarks: null,
      }

      const finalAttendance: StaffAttendanceResponse =
        attendance ?? defaultAttendance

      const mergedItem: StaffAttendanceWithMember = {
        id: finalAttendance.id,
        staff_id: finalAttendance.staff_id,
        date: finalAttendance.date,
        status: finalAttendance.status,
        created_at: finalAttendance.created_at,
        updated_at: finalAttendance.updated_at,
        time_in: finalAttendance.time_in,
        time_out: finalAttendance.time_out,
        remarks: finalAttendance.remarks,
        staff: staff,
      }
      return mergedItem
    })
  }, [staffData, attendanceData, formattedDateForApi])

  const filteredData = React.useMemo(() => {
    return mergedData.filter(
      (item) =>
        item.staff?.name.toLowerCase().includes(search.toLowerCase()) ||
        item.staff?.employee_id.toLowerCase().includes(search.toLowerCase()),
    )
  }, [mergedData, search])

  const isFutureDate = isFuture(date) && !isToday(date)

  if (isStaffLoading && !staffData) {
    return (
      <Box className="flex h-[400px] items-center justify-center">
        <Spinner className="size-8" />
      </Box>
    )
  }

  return (
    <Stack gap={6} p={8} className="h-full">
      <HStack
        align="center"
        className="flex-col md:flex-row md:justify-between space-y-4 md:space-y-0"
      >
        <HStack gap={4}>
          <Heading size="h3" className="font-black">
            Attendance
          </Heading>
          <HStack
            gap={1}
            rounded="xl"
            className="bg-background border p-1 shadow-sm"
          >
            <Button
              variant="ghost"
              size="icon"
              className="size-8 rounded-lg"
              onClick={handlePrevDay}
            >
              <HugeiconsIcon icon={ArrowLeft01Icon} className="size-4" />
            </Button>
            <Text
              size="sm"
              className="px-2 font-bold min-w-[140px] text-center"
            >
              {formattedDateForDisplay}
            </Text>
            <Button
              variant="ghost"
              size="icon"
              className="size-8 rounded-lg"
              onClick={handleNextDay}
              disabled={isToday(date)}
            >
              <HugeiconsIcon icon={ArrowRight01Icon} className="size-4" />
            </Button>
          </HStack>
        </HStack>
        <HStack gap={3}>
          <Button
            variant="outline"
            className="rounded-xl border-2 font-bold h-10 px-4"
          >
            <HStack gap={2} p={0}>
              <HugeiconsIcon icon={Download02Icon} className="size-4" />
              <span>Attendance Report</span>
            </HStack>
          </Button>
          <Button
            className="rounded-xl font-bold h-10 px-4"
            disabled={isFutureDate}
          >
            <HStack gap={2} p={0}>
              <HugeiconsIcon icon={PlusSignIcon} className="size-4" />
              <span>Add Attendance</span>
            </HStack>
          </Button>
        </HStack>
      </HStack>

      <AttendanceSummaryCards attendanceRecords={mergedData} />

      <Card className="border-none shadow-xl overflow-hidden bg-card">
        <CardHeader className="p-0">
          <HStack
            align="center"
            className="flex-col lg:flex-row lg:justify-between space-y-4 lg:space-y-0 border-b bg-muted/20 px-6 py-5"
          >
            <HStack gap={3} className="flex-wrap">
              <Box className="relative group">
                <HugeiconsIcon
                  icon={Search01Icon}
                  className="absolute left-3 top-1/2 size-4 -translate-y-1/2 text-muted-foreground transition-colors group-focus-within:text-primary"
                />
                <Input
                  placeholder="Search employee..."
                  className="w-72 border-none bg-background/50 pl-10 ring-1 ring-border focus-visible:ring-2 focus-visible:ring-primary shadow-sm rounded-xl h-10"
                  value={search}
                  onChange={(e) => setSearch(e.target.value)}
                />
              </Box>

              <HStack
                gap={1}
                rounded="xl"
                className="bg-background border p-1 shadow-sm"
              >
                <Box p={0} className="pl-2">
                  <HugeiconsIcon
                    icon={Calendar01Icon}
                    className="size-4 text-muted-foreground"
                  />
                </Box>
                <Select defaultValue="today">
                  <SelectTrigger className="h-8 border-none bg-transparent text-[11px] font-black uppercase tracking-wider focus:ring-0 w-[120px]">
                    <SelectValue placeholder="Date Range" />
                  </SelectTrigger>
                  <SelectContent>
                    <SelectItem value="today">Today</SelectItem>
                    <SelectItem value="yesterday">Yesterday</SelectItem>
                    <SelectItem value="last-7-days">Last 7 Days</SelectItem>
                  </SelectContent>
                </Select>
              </HStack>

              <Button
                variant="outline"
                size="sm"
                className="h-10 rounded-xl font-bold px-4"
              >
                <HStack gap={2} p={0}>
                  <HugeiconsIcon icon={FilterIcon} className="size-4" />
                  <span>Advance Filter</span>
                </HStack>
              </Button>
            </HStack>
            <Button
              variant="outline"
              size="icon"
              className="h-10 w-10 rounded-xl shadow-sm transition-transform active:scale-95"
              onClick={() => refetchAttendance()}
            >
              <HugeiconsIcon icon={RefreshIcon} className="size-4" />
            </Button>
          </HStack>
        </CardHeader>
        <CardContent className="p-0">
          <DataTable
            columns={columns}
            data={filteredData}
            pageIndex={0}
            pageSize={filteredData.length}
            pageCount={1}
            canNextPage={false}
            canPreviousPage={false}
            fetchNextPage={() => {}}
            fetchPreviousPage={() => {}}
          />
        </CardContent>
      </Card>

      <MarkStaffAttendanceDialog
        open={isDialogOpen}
        onOpenChange={setIsDialogOpen}
        attendance={selectedAttendance}
        date={formattedDateForApi}
      />
    </Stack>
  )
}
