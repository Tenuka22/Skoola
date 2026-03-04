import { Suspense, useState } from 'react'
import { useQuery, useSuspenseQuery } from '@tanstack/react-query'
import { format } from 'date-fns'
import { stringify as serializeCsv } from 'csv-stringify/browser/esm/sync'
import { StaffAttendanceFilters } from './components/staff-attendance-filters'
import { StaffAttendanceTable } from './components/staff-attendance-table'
import type {
  StaffAttendanceResponse,
  StaffResponse,
} from '@/lib/api/types.gen'
import { Spinner } from '@/components/ui/spinner'
import {
  getAllStaffOptions,
  getStaffAttendanceByDateOptions,
} from '@/lib/api/@tanstack/react-query.gen'
import { Heading, Stack, Text } from '@/components/primitives'

type EnrichedStaffAttendance = StaffAttendanceResponse & {
  staff?: StaffResponse
}

export function StaffAttendancePage() {
  const [selectedDate, setSelectedDate] = useState<Date>(new Date())

  const { data: attendanceData, isFetching: isExportDataFetching } = useQuery({
    ...getStaffAttendanceByDateOptions({
      query: { date: format(selectedDate, 'yyyy-MM-dd') },
    }),
  })
  const { data: staffList } = useSuspenseQuery(getAllStaffOptions())

  const enrichedAttendanceList: Array<EnrichedStaffAttendance> | undefined =
    attendanceData?.map((attendanceRecord) => {
      const staff = staffList?.data?.find(
        (s) => s.id === attendanceRecord.staff_id,
      )
      return { ...attendanceRecord, staff }
    })

  const handleExport = () => {
    if (!enrichedAttendanceList) return

    const dataToExport = enrichedAttendanceList.map((staff) => ({
      'Employee ID': staff.staff?.employee_id,
      'Staff Name': staff.staff?.name,
      Status: staff.status,
    }))

    const csv = serializeCsv(dataToExport, { header: true })
    const blob = new Blob([csv], { type: 'text/csv;charset=utf-8;' })
    const link = document.createElement('a')
    const url = URL.createObjectURL(blob)
    link.setAttribute('href', url)
    link.setAttribute(
      'download',
      `staff-attendance-${format(selectedDate, 'yyyy-MM-dd')}.csv`,
    )
    document.body.appendChild(link)
    link.click()
    document.body.removeChild(link)
  }

  return (
    <Stack gap={4} p={8} className="h-full">
      <Stack gap={1}>
        <Heading size="h2">Staff Attendance</Heading>
        <Text muted as="p">
          Mark and manage daily attendance for staff members.
        </Text>
      </Stack>

      <Suspense fallback={<Spinner />}>
        <StaffAttendanceFilters
          selectedDate={selectedDate}
          onDateChange={setSelectedDate}
          onExport={handleExport}
          isExporting={isExportDataFetching}
        />
      </Suspense>

      <Suspense fallback={<Spinner />}>
        <StaffAttendanceTable date={selectedDate} />
      </Suspense>
    </Stack>
  )
}
