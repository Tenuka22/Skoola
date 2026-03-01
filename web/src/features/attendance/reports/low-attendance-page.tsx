import { useQuery, useSuspenseQuery } from '@tanstack/react-query'
import { Suspense, useMemo, useState } from 'react'
import { addDays, format } from 'date-fns'
import { Calendar as CalendarIcon, Download } from 'lucide-react'
import * as papaparse from 'papaparse'
import type { ColumnDef } from '@tanstack/react-table'
import { useSuspenseClasses } from '../api'
import type { StudentAttendanceReportResponse } from '@/lib/api/types.gen'
import type { DateRange } from 'react-day-picker'
import { Box, HStack, Heading, Stack, Text } from '@/components/primitives'
import { getStudentsWithLowAttendanceOptions } from '@/lib/api/@tanstack/react-query.gen'
import { authClient } from '@/lib/clients'
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
import { Input } from '@/components/ui/input'
import { cn } from '@/lib/utils'
import { DataTable } from '@/components/ui/data-table'
import { FullPageSpinner } from '@/components/ui/full-page-spinner'
import { Card, CardContent, CardHeader } from '@/components/ui/card'

function Filters({
  selectedClassId,
  onClassChange,
  dateRange,
  onDateChange,
  threshold,
  onThresholdChange,
  onExport,
  isExporting,
}: {
  selectedClassId?: string
  onClassChange: (classId: string | undefined) => void
  dateRange?: DateRange
  onDateChange: (range?: DateRange) => void
  threshold: number
  onThresholdChange: (value: number) => void
  onExport: () => void
  isExporting: boolean
}) {
  const { data: classes } = useSuspenseClasses()
  return (
    <HStack justify="between" className="px-6 py-5 border-b bg-muted/20 flex-wrap gap-4">
      <HStack gap={2} className="flex-wrap">
        <Select
          value={selectedClassId}
          onValueChange={(value) => onClassChange(value ?? undefined)}
        >
          <SelectTrigger className="w-[240px] rounded-xl h-10 ring-1 ring-border">
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
                'w-[280px] justify-start text-left font-normal rounded-xl h-10',
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
        <div className="flex items-center gap-2 bg-background border rounded-xl px-3 h-10 shadow-sm">
          <Text size="xs" muted className="font-bold uppercase tracking-wider">Threshold %</Text>
          <Input
            type="number"
            placeholder="Threshold %"
            value={threshold}
            onChange={(e) => onThresholdChange(Number(e.target.value))}
            className="w-[60px] border-none bg-transparent p-0 h-auto focus-visible:ring-0 shadow-none font-bold"
          />
        </div>
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

function LowAttendanceTable({
  classId,
  fromDate,
  toDate,
  threshold,
}: {
  classId: string
  fromDate: string
  toDate: string
  threshold: number
}) {
  const { data: students, isFetching } = useSuspenseQuery(
    getStudentsWithLowAttendanceOptions({
      client: authClient,
      query: {
        class_id: classId,
        from_date: fromDate,
        to_date: toDate,
        threshold_percentage: threshold,
      },
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
          <Text size="sm" className="font-bold text-red-500">
            {row.original.percentage.toFixed(2)}%
          </Text>
        ),
      },
    ],
    [],
  )

  const dataWithId = useMemo(
    () =>
      students.map((row) => ({
        ...row,
        id: row.student_id,
      })),
    [students],
  )

  return (
    <div className="overflow-y-auto w-full flex-1">
      <DataTable
        columns={columns}
        data={dataWithId}
        isLoading={isFetching}
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

export function LowAttendancePage() {
  const [selectedClassId, setSelectedClassId] = useState<string | undefined>()
  const [dateRange, setDateRange] = useState<DateRange | undefined>({
    from: addDays(new Date(), -30),
    to: new Date(),
  })
  const [threshold, setThreshold] = useState<number>(80)

  const fromDate = dateRange?.from
    ? format(dateRange.from, 'yyyy-MM-dd')
    : undefined
  const toDate = dateRange?.to ? format(dateRange.to, 'yyyy-MM-dd') : undefined

  const lowAttendanceOptions = getStudentsWithLowAttendanceOptions({
    client: authClient,
    query: {
      class_id: selectedClassId || '',
      from_date: fromDate || '',
      to_date: toDate || '',
      threshold_percentage: threshold,
    },
  })

  const { data: reportData, isFetching: isExportDataFetching } = useQuery({
    ...lowAttendanceOptions,
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
      `low-attendance-report-${selectedClassId}-${fromDate}-to-${toDate}.csv`,
    )
    document.body.appendChild(link)
    link.click()
    document.body.removeChild(link)
  }

  return (
    <Stack gap={6} p={8} className="h-full">
      <Stack gap={1}>
        <Heading size="h2">Low Attendance Students</Heading>
        <Text muted as="p">
          View students with attendance below a specified threshold.
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
              threshold={threshold}
              onThresholdChange={setThreshold}
              onExport={handleExport}
              isExporting={isExportDataFetching}
            />
          </Suspense>
        </CardHeader>
        <CardContent className="p-0 flex flex-col min-h-[400px]">
          {selectedClassId && fromDate && toDate ? (
            <Suspense fallback={<FullPageSpinner />}>
              <LowAttendanceTable
                classId={selectedClassId}
                fromDate={fromDate}
                toDate={toDate}
                threshold={threshold}
              />
            </Suspense>
          ) : (
            <Box className="flex flex-1 items-center justify-center p-12">
              <Text muted>
                Select a class, date range, and threshold to generate a report.
              </Text>
            </Box>
          )}
        </CardContent>
      </Card>
    </Stack>
  )
}

