import * as React from 'react'
import { format } from 'date-fns'
import {
  Calendar01Icon,
  Search01Icon,
  Download01Icon,
} from '@hugeicons/core-free-icons'
import { HugeiconsIcon } from '@hugeicons/react'

import { Stack, HStack, Grid, Heading, Text, Box } from '@/components/primitives'
import { Button } from '@/components/ui/button'
import { Input } from '@/components/ui/input'
import {
  Select,
  SelectContent,
  SelectItem,
  SelectTrigger,
  SelectValue,
} from '@/components/ui/select'
import {
  Popover,
  PopoverContent,
  PopoverTrigger,
} from '@/components/ui/popover'
import { Calendar } from '@/components/ui/calendar'
import {
  Empty,
  EmptyHeader,
  EmptyMedia,
  EmptyTitle,
  EmptyDescription,
} from '@/components/ui/empty'
import { cn } from '@/lib/utils'
import { useAuth } from '@/hooks/use-auth'

import { useAttendanceStore } from '../store'
import {
  useClasses,
  useStudentAttendance,
  useMarkStudentAttendanceBulk,
} from '../api'
import { StudentAttendanceCard } from './student-attendance-card'
import type { AttendanceStatus } from '@/lib/api/types.gen'

export function StudentAttendanceView() {
  const { user } = useAuth()
  const {
    studentDate,
    setStudentDate,
    studentClassId,
    setStudentClassId,
    studentSearch,
    setStudentSearch,
  } = useAttendanceStore()

  const { data: classesData } = useClasses()
  const classes = classesData?.data || []

  const { data: attendanceData, isLoading: isLoadingAttendance } =
    useStudentAttendance(studentClassId || '', studentDate)

  // Map to hold local edits before saving
  const [localAttendance, setLocalAttendance] = React.useState<
    Record<string, AttendanceStatus>
  >({})

  // Initialize local state when fetched data changes
  React.useEffect(() => {
    if (attendanceData) {
      const initial: Record<string, AttendanceStatus> = {}
      attendanceData.forEach((record: any) => {
        if (record.student_id && record.status) {
          initial[record.student_id] = record.status as AttendanceStatus
        }
      })
      setLocalAttendance(initial)
    }
  }, [attendanceData])

  const markBulkMutation = useMarkStudentAttendanceBulk()

  const handleSave = () => {
    if (!studentClassId || !user?.id) return

    const records = Object.entries(localAttendance).map(
      ([student_id, status]) => ({
        student_id,
        status,
        date: studentDate,
        class_id: studentClassId,
        marked_by: user.id,
      }),
    )

    markBulkMutation.mutate({
      body: { attendance_records: records },
    })
  }

  const handleExportCSV = () => {
    if (!attendanceData) return
    const csvRows = [
      ['Student Name', 'Admission No', 'Status', 'Date'],
      ...attendanceData.map((r: any) => [
        r.student?.name_english || 'Unknown',
        r.student?.admission_number || 'N/A',
        localAttendance[r.student_id] || 'Not Marked',
        format(new Date(studentDate), 'yyyy-MM-dd'),
      ]),
    ]

    const csvContent = csvRows.map((e) => e.join(',')).join('\n')
    const blob = new Blob([csvContent], { type: 'text/csv;charset=utf-8;' })
    const link = document.createElement('a')
    const url = URL.createObjectURL(blob)
    link.setAttribute('href', url)
    link.setAttribute('download', `attendance_export_${studentDate}.csv`)
    link.style.visibility = 'hidden'
    document.body.appendChild(link)
    link.click()
    document.body.removeChild(link)
  }

  const handleStatusChange = (studentId: string, status: AttendanceStatus) => {
    setLocalAttendance((prev) => ({ ...prev, [studentId]: status }))
  }

  // Filter out records where student is undefined or doesn't match search
  const filteredRecords =
    attendanceData?.filter((record: any) => {
      if (!record.student) return false
      if (!studentSearch) return true
      const searchLower = studentSearch.toLowerCase()
      return (
        record.student.name_english?.toLowerCase().includes(searchLower) ||
        record.student.admission_number?.toLowerCase().includes(searchLower)
      )
    }) || []

  return (
    <Stack gap={4} p={8} className="h-full w-full">
      {/* Header & Toolbar */}
      <HStack className="justify-between wrap gap-4 w-full">
        <Stack gap={1}>
          <Heading size="h2">Student Attendance</Heading>
          <Text muted as="p">
            Manage daily roll call and evaluate student attendance records.
          </Text>
        </Stack>
        <HStack gap={2}>
          <Button
            variant="outline"
            onClick={handleExportCSV}
            disabled={!attendanceData?.length}
          >
            <HugeiconsIcon icon={Download01Icon} className="mr-2 size-4" />
            Export CSV
          </Button>
          <Button
            onClick={handleSave}
            disabled={
              markBulkMutation.isPending || !studentClassId || !attendanceData
            }
          >
            {markBulkMutation.isPending ? 'Saving...' : 'Save Attendance'}
          </Button>
        </HStack>
      </HStack>

      <HStack className="justify-between p-0">
        <HStack gap={2}>
          <div className="relative max-w-sm">
            <HugeiconsIcon
              icon={Search01Icon}
              className="absolute left-2.5 top-2.5 size-4 text-muted-foreground"
            />
            <Input
              type="search"
              placeholder="Search students..."
              className="pl-8 ring-0 focus-visible:ring-0 focus-visible:ring-offset-0 bg-transparent"
              value={studentSearch}
              onChange={(e) => setStudentSearch(e.target.value)}
            />
          </div>

          <Select value={studentClassId || ''} onValueChange={setStudentClassId}>
            <SelectTrigger className="w-[180px]">
              <SelectValue placeholder="Select Class" />
            </SelectTrigger>
            <SelectContent>
              {classes.map((c: any) => (
                <SelectItem key={c.id} value={c.id}>
                  {c.name}
                </SelectItem>
              ))}
            </SelectContent>
          </Select>
        </HStack>

        <Popover>
          <PopoverTrigger
            render={
              <Button
                variant={'outline'}
                className={cn(
                  'w-[240px] justify-start text-left font-normal',
                  !studentDate && 'text-muted-foreground',
                )}
              >
                <HugeiconsIcon icon={Calendar01Icon} className="mr-2 size-4" />
                {studentDate ? (
                  format(new Date(studentDate), 'PPP')
                ) : (
                  <span>Pick a date</span>
                )}
              </Button>
            }
          />
          <PopoverContent className="w-auto p-0" align="end">
            <Calendar
              mode="single"
              selected={new Date(studentDate)}
              onSelect={(date) =>
                date && setStudentDate(format(date, 'yyyy-MM-dd'))
              }
              initialFocus
            />
          </PopoverContent>
        </Popover>
      </HStack>

      {/* Grid Content */}
      <Box className="flex-1 overflow-y-auto">
        {!studentClassId ? (
          <Empty className="border border-dashed w-auto mt-8 h-64 flex flex-col justify-center">
            <EmptyHeader>
              <EmptyMedia variant="icon">
                <HugeiconsIcon icon={Calendar01Icon} />
              </EmptyMedia>
              <EmptyTitle>Select a Class</EmptyTitle>
              <EmptyDescription>
                Please select a class from the dropdown above to mark
                attendance.
              </EmptyDescription>
            </EmptyHeader>
          </Empty>
        ) : isLoadingAttendance ? (
          <Text className="text-center mt-8 text-muted-foreground">
            Loading students...
          </Text>
        ) : filteredRecords.length === 0 ? (
          <Empty className="border border-dashed w-auto mt-8 h-64 flex flex-col justify-center">
            <EmptyHeader>
              <EmptyMedia variant="icon">
                <HugeiconsIcon icon={Search01Icon} />
              </EmptyMedia>
              <EmptyTitle>No Students Found</EmptyTitle>
              <EmptyDescription>
                No students match your search criteria, or this class is empty.
              </EmptyDescription>
            </EmptyHeader>
          </Empty>
        ) : (
          <Grid
            cols={1}
            className="sm:grid-cols-2 md:grid-cols-3 xl:grid-cols-4"
            gap={4}
          >
            {filteredRecords.map((record: any) => (
              <StudentAttendanceCard
                key={record.student_id}
                student={record.student}
                status={localAttendance[record.student_id]}
                onStatusChange={(status) =>
                  handleStatusChange(record.student_id, status)
                }
              />
            ))}
          </Grid>
        )}
      </Box>
    </Stack>
  )
}

