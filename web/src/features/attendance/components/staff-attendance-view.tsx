import * as React from 'react'
import { format } from 'date-fns'
import {
  Calendar01Icon,
  Download01Icon,
  Search01Icon,
} from '@hugeicons/core-free-icons'
import { HugeiconsIcon } from '@hugeicons/react'

import { useAttendanceStore } from '../store'
import {
  useMarkStaffAttendanceBulk,
  useStaffAttendance,
  useStaffList,
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

export function StaffAttendanceView() {
  const { staffDate, setStaffDate, staffSearch, setStaffSearch } =
    useAttendanceStore()

  // Fetch all staff members to list them
  const { data: staffData, isLoading: isLoadingStaff } = useStaffList()
  const staffList = React.useMemo(() => staffData?.data || [], [staffData])

  // Fetch existing attendance records for the date
  const { data: attendanceData, isLoading: isLoadingAttendance } =
    useStaffAttendance(staffDate)

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
        date: staffDate,
      }),
    )

    if (records.length === 0) return

    markBulkMutation.mutate({
      body: { attendance_records: records, date: staffDate },
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
        format(new Date(staffDate), 'yyyy-MM-dd'),
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

  return (
    <Stack gap={4} p={8} className="h-full w-full">
      {/* Header & Toolbar */}
      <HStack className="justify-between wrap gap-4 w-full">
        <Stack gap={1}>
          <Heading size="h2">Staff Attendance</Heading>
          <Text muted as="p">
            Manage daily attendance for teachers and administrative staff.
          </Text>
        </Stack>
        <HStack gap={2}>
          <Button
            variant="outline"
            onClick={handleExportCSV}
            disabled={!staffList.length}
          >
            <HugeiconsIcon icon={Download01Icon} className="mr-2 size-4" />
            Export CSV
          </Button>
          <Button
            onClick={handleSave}
            disabled={markBulkMutation.isPending || !staffList.length}
          >
            {markBulkMutation.isPending ? 'Saving...' : 'Save Attendance'}
          </Button>
        </HStack>
      </HStack>

      <HStack className="justify-between p-0">
        <div className="relative max-w-sm">
          <HugeiconsIcon
            icon={Search01Icon}
            className="absolute left-2.5 top-2.5 size-4 text-muted-foreground"
          />
          <Input
            type="search"
            placeholder="Search staff..."
            className="pl-8 ring-0 focus-visible:ring-0 focus-visible:ring-offset-0 bg-transparent w-[300px]"
            value={staffSearch}
            onChange={(e) => setStaffSearch(e.target.value)}
          />
        </div>

        <Popover>
          <PopoverTrigger
            render={
              <Button
                variant={'outline'}
                className={cn(
                  'w-[240px] justify-start text-left font-normal',
                  !staffDate && 'text-muted-foreground',
                )}
              >
                <HugeiconsIcon icon={Calendar01Icon} className="mr-2 size-4" />
                {staffDate ? (
                  format(new Date(staffDate), 'PPP')
                ) : (
                  <span>Pick a date</span>
                )}
              </Button>
            }
          />
          <PopoverContent className="w-auto p-0" align="end">
            <Calendar
              mode="single"
              selected={new Date(staffDate)}
              onSelect={(date) =>
                date && setStaffDate(format(date, 'yyyy-MM-dd'))
              }
              initialFocus
            />
          </PopoverContent>
        </Popover>
      </HStack>

      {/* Grid Content */}
      <Box className="flex-1 overflow-y-auto">
        {isLoadingStaff || isLoadingAttendance ? (
          <Text className="text-center mt-8 text-muted-foreground">
            Loading staff...
          </Text>
        ) : filteredStaff.length === 0 ? (
          <Empty className="border border-dashed w-auto mt-8 h-64 flex flex-col justify-center">
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
