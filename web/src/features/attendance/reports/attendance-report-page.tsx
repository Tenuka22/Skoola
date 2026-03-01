import { Suspense, useMemo, useState } from 'react'
import { addDays, format } from 'date-fns'
import { Calendar as CalendarIcon, Download } from 'lucide-react'
import * as papaparse from 'papaparse'
import {
  useGenerateStudentAttendanceReport,
  useSuspenseClasses,
  useSuspenseGenerateStudentAttendanceReport,
} from '../api'
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

function Filters({
  selectedClassId,
  onClassChange,
  dateRange,
  onDateChange,
  onExport,
  isExporting,
}: {
  selectedClassId?: string
  onClassChange: (classId: string | undefined) => void
  dateRange?: DateRange
  onDateChange: (range?: DateRange) => void
  onExport: () => void
  isExporting: boolean
}) {
  const { data: classes } = useSuspenseClasses()
  return (
    <HStack justify="between" className="px-6 py-5 border-b bg-muted/20">
      <HStack gap={2}>
        <Select
          value={selectedClassId}
          onValueChange={(value) => onClassChange(value ?? undefined)}
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
          <PopoverTrigger>
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
          </PopoverTrigger>
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
  const { data: report } = useSuspenseGenerateStudentAttendanceReport(
    classId,
    fromDate,
    toDate,
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
  const [selectedClassId, setSelectedClassId] = useState<string | undefined>()
  const [dateRange, setDateRange] = useState<DateRange | undefined>({
    from: addDays(new Date(), -30),
    to: new Date(),
  })

  const fromDate = dateRange?.from
    ? format(dateRange.from, 'yyyy-MM-dd')
    : undefined
  const toDate = dateRange?.to ? format(dateRange.to, 'yyyy-MM-dd') : undefined

  const { data: reportData, isFetching: isExportDataFetching } =
    useGenerateStudentAttendanceReport(
      selectedClassId || '',
      fromDate || '',
      toDate || '',
      !!(selectedClassId && fromDate && toDate),
    )

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
              selectedClassId={selectedClassId}
              onClassChange={setSelectedClassId}
              dateRange={dateRange}
              onDateChange={setDateRange}
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
