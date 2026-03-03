import { Suspense, useMemo } from 'react'
import { format } from 'date-fns'
import { Calendar as CalendarIcon, Download } from 'lucide-react'
import * as papaparse from 'papaparse'
import { useQuery, useSuspenseQuery } from '@tanstack/react-query'
import { getAttendanceReportQueryOptions } from '../api'
import { useAttendanceSearchParams } from '../search-params'
import type { ColumnDef } from '@tanstack/react-table'
import type { StudentAttendanceReportResponse } from '@/lib/api/types.gen'
import type { DateRange } from 'react-day-picker'
import { Box, HStack, Heading, Stack, Text } from '@/components/primitives'
import {
  Select,
  SelectContent,
  SelectItem,
  SelectTrigger,
  SelectValue,
} from '@/components/ui/select'
import { Button } from '@/components/ui/button'
import { Calendar } from '@/components/ui/calendar'
import {
  Popover,
  PopoverContent,
  PopoverTrigger,
} from '@/components/ui/popover'
import { cn } from '@/lib/utils'
import { DataTable } from '@/components/ui/data-table'
import { FullPageSpinner } from '@/components/ui/full-page-spinner'
import { Card, CardContent, CardHeader } from '@/components/ui/card'
import { getAllClassesQueryOptions } from '@/features/academics/classes/api'

function Filters({
  selectedClassId,
  onClassChange,
  dateRange,
  onDateChange,
  onExport,
  isExporting,
}: {
  selectedClassId?: string
  onClassChange: (classId: string | null) => void
  dateRange?: DateRange
  onDateChange: (range?: DateRange) => void
  onExport: () => void
  isExporting: boolean
}) {
  const { data: classes } = useSuspenseQuery(getAllClassesQueryOptions())
  return (
    <HStack justify="between" className="px-6 py-5 border-b bg-muted/20">
      <HStack gap={2}>
        <Select
          value={selectedClassId ?? ''}
          onValueChange={(value) => onClassChange(value || null)}
        >
          <SelectTrigger className="w-[280px] rounded-xl h-10 ring-1 ring-border">
            <SelectValue placeholder="Select a class" />
          </SelectTrigger>
          <SelectContent>
            {classes?.data?.map((c) => (
              <SelectItem key={c.id} value={c.id}>
                {c.section_name} - {c.id}
              </SelectItem>
            ))}
          </SelectContent>
        </Select>

        <Popover>
          <PopoverTrigger
            render={
              <Button
                id="date"
                variant={'outline'}
                className={cn(
                  'w-[300px] justify-start text-left font-normal rounded-xl h-10',
                  !dateRange && 'text-muted-foreground',
                )}
              >
                <CalendarIcon className="mr-2 h-4 w-4" />
                {dateRange?.from ? (
                  dateRange.to ? (
                    <>
                      {format(dateRange.from, 'LLL dd, y')} -{' '}
                      {format(dateRange.to, 'LLL dd, y')}
                    </>
                  ) : (
                    format(dateRange.from, 'LLL dd, y')
                  )
                ) : (
                  <span>Pick a date range</span>
                )}
              </Button>
            }
          />
          <PopoverContent className="w-auto p-0" align="start">
            <Calendar
              initialFocus
              mode="range"
              defaultMonth={dateRange?.from}
              selected={dateRange}
              onSelect={onDateChange}
              numberOfMonths={2}
            />
          </PopoverContent>
        </Popover>
      </HStack>
      <Button
        variant="outline"
        size="sm"
        onClick={onExport}
        disabled={isExporting || !selectedClassId}
        className="rounded-xl font-bold h-10 px-4"
      >
        <Download className="mr-2 h-4 w-4" />
        Export CSV
      </Button>
    </HStack>
  )
}

function ReportTable({
  classId,
  fromDate,
  toDate,
}: {
  classId: string
  fromDate: string
  toDate: string
}) {
  const { data: report } = useSuspenseQuery(
    getAttendanceReportQueryOptions({
      query: { class_id: classId, from_date: fromDate, to_date: toDate },
    }),
  )

  const columns = useMemo<
    Array<ColumnDef<StudentAttendanceReportResponse & { id: string | number }>>
  >(
    () => [
      {
        accessorKey: 'student_name',
        header: 'Student',
        cell: ({ row }) => (
          <Text size="sm" className="font-bold">
            {row.original.student_name}
          </Text>
        ),
      },
      {
        accessorKey: 'total_days',
        header: 'Total Days',
        cell: ({ row }) => <Text size="sm">{row.original.total_days}</Text>,
      },
      {
        accessorKey: 'percentage',
        header: 'Percentage',
        cell: ({ row }) => (
          <Text size="sm" className="font-bold text-primary">
            {row.original.percentage.toFixed(2)}%
          </Text>
        ),
      },
    ],
    [],
  )

  const dataWithId = useMemo(
    () =>
      report.map((row) => ({
        ...row,
        id: row.student_id,
      })),
    [report],
  )

  return (
    <div className="overflow-y-auto w-full flex-1">
      <DataTable
        columns={columns}
        data={dataWithId}
        pageIndex={0}
        pageSize={dataWithId.length}
        pageCount={1}
        canNextPage={false}
        canPreviousPage={false}
        fetchNextPage={() => {}}
        fetchPreviousPage={() => {}}
      />
    </div>
  )
}

export function AttendanceReportPage() {
  const {
    classId: selectedClassId,
    setClassId: setSelectedClassId,
    fromDate,
    setFromDate,
    toDate,
    setToDate,
  } = useAttendanceSearchParams()

  const dateRange = useMemo<DateRange | undefined>(
    () => ({
      from: fromDate ? new Date(fromDate) : undefined,
      to: toDate ? new Date(toDate) : undefined,
    }),
    [fromDate, toDate],
  )

  const { data: reportData, isFetching: isExportDataFetching } = useQuery({
    ...getAttendanceReportQueryOptions({
      query: {
        class_id: selectedClassId || '',
        from_date: fromDate || '',
        to_date: toDate || '',
      },
    }),
    enabled: !!(selectedClassId && fromDate && toDate),
  })

  const handleExport = () => {
    if (!reportData || !selectedClassId || !fromDate || !toDate) {
      console.warn('Cannot export: Missing data or selections.')
      return
    }

    const currentReportData: Array<StudentAttendanceReportResponse> =
      reportData || []

    const dataToExport = currentReportData.map((row) => ({
      'Student ID': row.student_id,
      'Student Name': row.student_name,
      'Total Days': row.total_days,
      'Attendance Percentage': row.percentage.toFixed(2) + '%',
    }))
    const csv = papaparse.unparse(dataToExport)
    const blob = new Blob([csv], { type: 'text/csv;charset=utf-8;' })
    const link = document.createElement('a')
    const url = URL.createObjectURL(blob)
    link.setAttribute('href', url)
    link.setAttribute(
      'download',
      `attendance-report-${selectedClassId}-${fromDate}-to-${toDate}.csv`,
    )
    document.body.appendChild(link)
    link.click()
    document.body.removeChild(link)
  }

  const handleDateChange = (range?: DateRange) => {
    setFromDate(range?.from ? format(range.from, 'yyyy-MM-dd') : null)
    setToDate(range?.to ? format(range.to, 'yyyy-MM-dd') : null)
  }

  return (
    <Stack gap={6} p={8} className="h-full">
      <Stack gap={1}>
        <Heading size="h2">Attendance Report</Heading>
        <Text muted as="p">
          Generate attendance reports for classes over a date range.
        </Text>
      </Stack>

      <Card className="border-none shadow-xl overflow-hidden bg-card">
        <CardHeader className="p-0">
          <Suspense fallback={<FullPageSpinner />}>
            <Filters
              selectedClassId={selectedClassId ?? undefined}
              onClassChange={setSelectedClassId}
              dateRange={dateRange}
              onDateChange={handleDateChange}
              onExport={handleExport}
              isExporting={isExportDataFetching}
            />
          </Suspense>
        </CardHeader>
        <CardContent className="p-0 flex flex-col min-h-[400px]">
          {selectedClassId && fromDate && toDate ? (
            <Suspense fallback={<FullPageSpinner />}>
              <ReportTable
                classId={selectedClassId}
                fromDate={fromDate}
                toDate={toDate}
              />
            </Suspense>
          ) : (
            <Box className="flex flex-1 items-center justify-center p-12">
              <Text muted>
                Select a class and date range to generate a report.
              </Text>
            </Box>
          )}
        </CardContent>
      </Card>
    </Stack>
  )
}
