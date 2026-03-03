import { useMemo, useState } from 'react'
import { addDays, format } from 'date-fns'
import { useMatch } from '@tanstack/react-router'
import { useSuspenseQuery } from '@tanstack/react-query'
import {
  getAttendanceByStudentQueryOptions,
  getStudentAttendancePercentageQueryOptions,
} from '../../api'
import { IssueExitPassDialog } from './issue-exit-pass-dialog'
import { SubmitExcuseDialog } from './submit-excuse-dialog'
import type { ColumnDef } from '@tanstack/react-table'
import type { DateRange, DayProps } from 'react-day-picker'
import type { StudentAttendanceResponse } from '@/lib/api/types.gen'
import { getStudentByIdQueryOptions } from '@/features/students/api'
import { HStack, Heading, Stack, Text } from '@/components/primitives'
import { Avatar, AvatarFallback, AvatarImage } from '@/components/ui/avatar'
import { Card, CardContent, CardHeader, CardTitle } from '@/components/ui/card'
import { Calendar } from '@/components/ui/calendar'
import { Badge } from '@/components/ui/badge'
import { Button } from '@/components/ui/button'
import { DataTable } from '@/components/ui/data-table'
import { cn } from '@/lib/utils'

function getStatusColor(status: string) {
  switch (status) {
    case 'Present':
      return 'text-green-500 bg-green-500/10 border-green-500/20'
    case 'Absent':
      return 'text-red-500 bg-red-500/10 border-red-500/20'
    case 'Late':
      return 'text-orange-500 bg-orange-500/10 border-orange-500/20'
    case 'Excused':
      return 'text-blue-500 bg-blue-500/10 border-blue-500/20'
    default:
      return 'text-muted-foreground bg-muted/10 border-muted/20'
  }
}

export function StudentAttendanceHistoryPage() {
  const { studentId } = useMatch({
    from: '/admin/attendance/student/$studentId',
  }).params
  const [isIssuePassOpen, setIsIssuePassOpen] = useState(false)
  const [isExcuseOpen, setIsExcuseOpen] = useState(false)
  const [selectedRecordId, setSelectedRecordId] = useState('')

  const [dateRange, setDateRange] = useState<DateRange | undefined>({
    from: addDays(new Date(), -30),
    to: new Date(),
  })

  const { data: student } = useSuspenseQuery(
    getStudentByIdQueryOptions({
      path: { student_id: studentId },
    }),
  )

  const { data: attendanceHistory } = useSuspenseQuery(
    getAttendanceByStudentQueryOptions({
      path: { student_id: studentId },
      query: {
        from_date: dateRange?.from
          ? format(dateRange.from, 'yyyy-MM-dd')
          : undefined,
        to_date: dateRange?.to ? format(dateRange.to, 'yyyy-MM-dd') : undefined,
      },
    }),
  )

  const { data: attendancePercentage } = useSuspenseQuery(
    getStudentAttendancePercentageQueryOptions({
      path: { student_id: studentId },
      query: {
        from_date: format(dateRange?.from || new Date(), 'yyyy-MM-dd'),
        to_date: format(dateRange?.to || new Date(), 'yyyy-MM-dd'),
      },
    }),
  )

  const attendanceByDate = useMemo(
    () =>
      new Map(
        attendanceHistory.map((rec) => [
          format(new Date(rec.date), 'yyyy-MM-dd'),
          rec.status,
        ]),
      ),
    [attendanceHistory],
  )

  const openExcuseDialog = (recordId: string) => {
    setSelectedRecordId(recordId)
    setIsExcuseOpen(true)
  }

  const columns = useMemo<
    Array<ColumnDef<StudentAttendanceResponse & { id: string | number }>>
  >(
    () => [
      {
        accessorKey: 'date',
        header: 'Date',
        cell: ({ row }) => (
          <Text size="sm" className="font-bold">
            {format(new Date(row.original.date), 'PPP')}
          </Text>
        ),
      },
      {
        accessorKey: 'status',
        header: 'Status',
        cell: ({ row }) => (
          <Badge
            variant="outline"
            className={cn(
              'rounded-lg font-bold border',
              getStatusColor(row.original.status),
            )}
          >
            {row.original.status}
          </Badge>
        ),
      },
      {
        accessorKey: 'remarks',
        header: 'Remarks',
        cell: ({ row }) => (
          <Text size="xs" muted className="max-w-[200px] truncate">
            {row.original.remarks || 'No remarks'}
          </Text>
        ),
      },
      {
        id: 'actions',
        header: '',
        cell: ({ row }) =>
          row.original.status === 'Absent' ? (
            <div className="flex justify-end">
              <Button
                variant="outline"
                size="sm"
                className="h-8 font-bold text-[10px] uppercase tracking-wider hover:bg-primary hover:text-primary-foreground"
                onClick={() => openExcuseDialog(row.original.id)}
              >
                Submit Excuse
              </Button>
            </div>
          ) : null,
      },
    ],
    [],
  )

  const dataForTable = useMemo(() => {
    return attendanceHistory.map((rec) => ({
      ...rec,
      id: rec.id,
    }))
  }, [attendanceHistory])

  return (
    <>
      <IssueExitPassDialog
        open={isIssuePassOpen}
        onOpenChange={setIsIssuePassOpen}
        studentId={studentId}
      />
      <SubmitExcuseDialog
        open={isExcuseOpen}
        onOpenChange={setIsExcuseOpen}
        attendanceRecordId={selectedRecordId}
      />
      <Stack gap={6} p={8} className="h-full">
        <HStack gap={4} align="center" justify="between">
          <HStack gap={4} align="center">
            <Avatar className="h-16 w-16 border-2 border-primary/20">
              <AvatarImage src={student.profile_photo_url || ''} />
              <AvatarFallback className="text-xl font-black">
                {student.name_english.charAt(0)}
              </AvatarFallback>
            </Avatar>
            <Stack gap={1}>
              <Heading size="h2" className="font-black">
                {student.name_english}
              </Heading>
              <Text
                muted
                className="font-bold tracking-widest uppercase text-xs"
              >
                {student.admission_number}
              </Text>
            </Stack>
          </HStack>
          <Button
            className="rounded-xl font-bold h-10 px-6 shadow-lg shadow-primary/20"
            onClick={() => setIsIssuePassOpen(true)}
          >
            Issue Exit Pass
          </Button>
        </HStack>

        <div className="grid gap-4 md:grid-cols-2 lg:grid-cols-4">
          <Card className="border-none shadow-xl bg-card">
            <CardHeader className="pb-2">
              <Text
                muted
                className="text-[10px] font-black uppercase tracking-widest"
              >
                Attendance Percentage
              </Text>
            </CardHeader>
            <CardContent>
              <Text size="2xl" className="font-black text-primary">
                {(Number(attendancePercentage) || 0).toFixed(2)}%
              </Text>
            </CardContent>
          </Card>
        </div>

        <Card className="border-none shadow-xl bg-card overflow-hidden">
          <CardHeader className="bg-muted/20 border-b px-6 py-4">
            <CardTitle className="text-sm font-bold">
              Attendance History Map
            </CardTitle>
          </CardHeader>
          <CardContent className="flex gap-8 p-6">
            <Calendar
              mode="range"
              selected={dateRange}
              onSelect={setDateRange}
              numberOfMonths={2}
              className="p-0"
              components={{
                Day: (props: DayProps) => {
                  const date = format(props.day.date, 'yyyy-MM-dd')
                  const status = attendanceByDate.get(date)
                  return (
                    <div className="relative h-full w-full">
                      <span className="relative z-10">
                        {props.day.date.getDate()}
                      </span>
                      {status && (
                        <Badge
                          className={cn(
                            'absolute bottom-0 left-1/2 -translate-x-1/2 w-1.5 h-1.5 p-0 rounded-full border-none',
                            status === 'Present'
                              ? 'bg-green-500'
                              : status === 'Absent'
                                ? 'bg-red-500'
                                : status === 'Late'
                                  ? 'bg-orange-500'
                                  : 'bg-blue-500',
                          )}
                        />
                      )}
                    </div>
                  )
                },
              }}
            />
            <Stack gap={4} className="border-l pl-8 border-border/40">
              <Text
                size="xs"
                className="font-black uppercase tracking-widest text-muted-foreground"
              >
                Legend
              </Text>
              <Stack gap={2}>
                <HStack gap={3} align="center">
                  <div className="size-2 rounded-full bg-green-500 shadow-sm shadow-green-500/40" />{' '}
                  <Text size="xs" className="font-bold">
                    Present
                  </Text>
                </HStack>
                <HStack gap={3} align="center">
                  <div className="size-2 rounded-full bg-red-500 shadow-sm shadow-red-500/40" />{' '}
                  <Text size="xs" className="font-bold">
                    Absent
                  </Text>
                </HStack>
                <HStack gap={3} align="center">
                  <div className="size-2 rounded-full bg-orange-500 shadow-sm shadow-orange-500/40" />{' '}
                  <Text size="xs" className="font-bold">
                    Late
                  </Text>
                </HStack>
                <HStack gap={3} align="center">
                  <div className="size-2 rounded-full bg-blue-500 shadow-sm shadow-blue-500/40" />{' '}
                  <Text size="xs" className="font-bold">
                    Excused
                  </Text>
                </HStack>
              </Stack>
            </Stack>
          </CardContent>
        </Card>

        <Card className="border-none shadow-xl bg-card overflow-hidden">
          <CardHeader className="bg-muted/20 border-b px-6 py-4">
            <CardTitle className="text-sm font-bold">
              Attendance Records
            </CardTitle>
          </CardHeader>
          <CardContent className="p-0">
            <DataTable
              columns={columns}
              data={dataForTable}
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
      </Stack>
    </>
  )
}
