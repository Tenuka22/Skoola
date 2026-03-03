import * as React from 'react'
import { format, isFuture, isToday } from 'date-fns'
import {
  Calendar01Icon,
  Download01Icon,
  Search01Icon,
} from '@hugeicons/core-free-icons'
import { HugeiconsIcon } from '@hugeicons/react'

import { useQuery } from '@tanstack/react-query'
import { useAttendanceSearchParams } from '../search-params'
import {
  getStaffAttendanceQueryOptions,
  useMarkStaffAttendanceBulk,
} from '../api'
import { StaffAttendanceCard } from './staff-attendance-card'
import type {
  AttendanceStatus,
  StaffAttendanceResponse,
  StaffResponse,
} from '@/lib/api/types.gen'
import {
  Box,
  Grid,
  HStack,
  Heading,
  Stack,
  Text,
} from '@/components/primitives'
import { Button } from '@/components/ui/button'
import { Input } from '@/components/ui/input'
import {
  Popover,
  PopoverContent,
  PopoverTrigger,
} from '@/components/ui/popover'
import { Calendar } from '@/components/ui/calendar'
import {
  Empty,
  EmptyDescription,
  EmptyHeader,
  EmptyMedia,
  EmptyTitle,
} from '@/components/ui/empty'
import { cn } from '@/lib/utils'
import { getAllStaffQueryOptions } from '@/features/staff/api'

export function StaffAttendanceView() {
  const { date: staffDate, setDate: setStaffDate } = useAttendanceSearchParams()
  const [staffSearch, setStaffSearch] = React.useState('')

  // Fetch all staff members to list them
  const { data: staffData, isLoading: isLoadingStaff } = useQuery(
    getAllStaffQueryOptions({
      query: { limit: 100 },
    }),
  )
  const staffList = React.useMemo(() => staffData?.data || [], [staffData])

  // Fetch existing attendance records for the date
  const { data: attendanceData, isLoading: isLoadingAttendance } = useQuery(
    getStaffAttendanceQueryOptions({
      query: { date: staffDate ?? format(new Date(), 'yyyy-MM-dd') },
    }),
  )

  const markBulkMutation = useMarkStaffAttendanceBulk()

  const [localAttendance, setLocalAttendance] = React.useState<
    Record<string, AttendanceStatus>
  >({})

  // Initialize local attendance state combining staff list and existing records
  React.useEffect(() => {
    if (staffList.length && !isLoadingAttendance) {
      const initial: Record<string, AttendanceStatus> = {}

      // If attendance data exists, map it
      if (attendanceData) {
        attendanceData.forEach((record: StaffAttendanceResponse) => {
          if (record.staff_id && record.status) {
            initial[record.staff_id] = record.status
          }
        })
      }
      setLocalAttendance(initial)
    }
  }, [staffList, attendanceData, isLoadingAttendance])

  const handleSave = () => {
    const records = Object.entries(localAttendance).map(
      ([staff_id, status]) => ({
        staff_id,
        status,
        date: staffDate ?? format(new Date(), 'yyyy-MM-dd'),
      }),
    )

    if (records.length === 0) return

    markBulkMutation.mutate({
      body: {
        attendance_records: records,
        date: staffDate ?? format(new Date(), 'yyyy-MM-dd'),
      },
    })
  }

  const handleExportCSV = () => {
    if (!staffList.length) return
    const csvRows = [
      ['Staff Name', 'Email', 'Status', 'Date'],
      ...staffList.map((staff: StaffResponse) => [
        staff.name || 'Unknown',
        staff.email || 'N/A',
        localAttendance[staff.id] || 'Not Marked',
        format(new Date(staffDate ?? new Date()), 'yyyy-MM-dd'),
      ]),
    ]

    const csvContent = csvRows.map((e) => e.join(',')).join('\n')
    const blob = new Blob([csvContent], { type: 'text/csv;charset=utf-8;' })
    const link = document.createElement('a')
    const url = URL.createObjectURL(blob)
    link.setAttribute('href', url)
    link.setAttribute('download', `staff_attendance_export_${staffDate}.csv`)
    link.style.visibility = 'hidden'
    document.body.appendChild(link)
    link.click()
    document.body.removeChild(link)
  }

  const handleStatusChange = (staffId: string, status: AttendanceStatus) => {
    setLocalAttendance((prev) => ({ ...prev, [staffId]: status }))
  }

  const filteredStaff = staffList.filter((staff: StaffResponse) => {
    if (!staffSearch) return true
    const searchLower = staffSearch.toLowerCase()
    return (
      staff.name?.toLowerCase().includes(searchLower) ||
      staff.email?.toLowerCase().includes(searchLower)
    )
  })

  const isFutureDate = staffDate
    ? isFuture(new Date(staffDate)) && !isToday(new Date(staffDate))
    : false

  return (
    <Stack gap={6} p={8} className="h-full w-full">
      {/* Header & Toolbar */}
      <HStack className="justify-between wrap gap-4 w-full">
        <Stack gap={1}>
          <Heading size="h2" className="font-black">
            Staff Attendance
          </Heading>
          <Text muted as="p">
            Manage daily attendance for teachers and administrative staff.
          </Text>
        </Stack>
        <HStack gap={3}>
          <Button
            variant="outline"
            onClick={handleExportCSV}
            disabled={!staffList.length}
            className="rounded-xl font-bold h-10 px-4"
          >
            <HStack gap={2} p={0}>
              <HugeiconsIcon icon={Download01Icon} className="size-4" />
              <span>Export CSV</span>
            </HStack>
          </Button>
          <Button
            className="rounded-xl font-bold h-10 px-6"
            onClick={handleSave}
            disabled={
              markBulkMutation.isPending || !staffList.length || isFutureDate
            }
          >
            {markBulkMutation.isPending ? 'Saving...' : 'Save Attendance'}
          </Button>
        </HStack>
      </HStack>

      <HStack className="justify-between p-0" gap={4}>
        <Box className="relative group flex-1 max-w-sm">
          <HugeiconsIcon
            icon={Search01Icon}
            className="absolute left-3 top-1/2 size-4 -translate-y-1/2 text-muted-foreground transition-colors group-focus-within:text-primary"
          />
          <Input
            type="search"
            placeholder="Search staff..."
            className="border-none bg-background/50 pl-10 ring-1 ring-border focus-visible:ring-2 focus-visible:ring-primary shadow-sm rounded-xl h-10 w-full"
            value={staffSearch}
            onChange={(e) => setStaffSearch(e.target.value)}
          />
        </Box>

        <Popover>
          <PopoverTrigger
            render={
              <Button
                variant={'outline'}
                className={cn(
                  'w-[240px] justify-start text-left font-bold rounded-xl h-10 shadow-sm ring-1 ring-border',
                  !staffDate && 'text-muted-foreground',
                )}
              >
                <HStack gap={2} p={0}>
                  <HugeiconsIcon icon={Calendar01Icon} className="size-4" />
                  <span>
                    {staffDate
                      ? format(new Date(staffDate), 'PPP')
                      : 'Pick a date'}
                  </span>
                </HStack>
              </Button>
            }
          />
          <PopoverContent className="w-auto p-0" align="end">
            <Calendar
              mode="single"
              selected={staffDate ? new Date(staffDate) : new Date()}
              onSelect={(date) =>
                date && setStaffDate(format(date, 'yyyy-MM-dd'))
              }
              initialFocus
              disabled={(date) => isFuture(date) && !isToday(date)}
            />
          </PopoverContent>
        </Popover>
      </HStack>

      {/* Grid Content */}
      <Box className="flex-1 overflow-y-auto">
        {isLoadingStaff || isLoadingAttendance ? (
          <Box className="flex h-[400px] items-center justify-center">
            <Text muted className="font-bold">
              Loading staff...
            </Text>
          </Box>
        ) : filteredStaff.length === 0 ? (
          <Empty className="border border-dashed w-auto mt-8 h-64 flex flex-col justify-center rounded-xl bg-muted/5">
            <EmptyHeader>
              <EmptyMedia variant="icon">
                <HugeiconsIcon icon={Search01Icon} />
              </EmptyMedia>
              <EmptyTitle>No Staff Found</EmptyTitle>
              <EmptyDescription>
                No staff members match your search criteria.
              </EmptyDescription>
            </EmptyHeader>
          </Empty>
        ) : (
          <Grid
            cols={1}
            className="sm:grid-cols-2 lg:grid-cols-3 xl:grid-cols-4"
            gap={4}
          >
            {filteredStaff.map((staff: StaffResponse) => (
              <StaffAttendanceCard
                key={staff.id}
                staff={staff}
                status={localAttendance[staff.id]}
                onStatusChange={(status) =>
                  handleStatusChange(staff.id, status)
                }
              />
            ))}
          </Grid>
        )}
      </Box>
    </Stack>
  )
}
