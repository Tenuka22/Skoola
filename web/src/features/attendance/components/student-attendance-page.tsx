'use client'

import {
  ArrowLeft01Icon,
  ArrowRight01Icon,
  Book01Icon,
  Download02Icon,
  FilterIcon,
  RefreshIcon,
  Search01Icon,
} from '@hugeicons/core-free-icons'
import { HugeiconsIcon } from '@hugeicons/react'
import { addDays, format, subDays } from 'date-fns'
import * as React from 'react'
import {
  useClasses,
  useGenerateStudentAttendanceReport,
  useStudentAttendance,
  useStudentsInClass,
} from '../api'
import { AttendanceSummaryCards } from './attendance-summary-cards'
import { MarkStudentAttendanceDialog } from './mark-student-attendance-dialog'
import { studentAttendanceColumns } from './student-attendance-columns'
import type { StudentAttendanceWithMember } from '../types'
import { Spinner } from '@/components/ui/spinner'
import {
  Select,
  SelectContent,
  SelectItem,
  SelectTrigger,
  SelectValue,
} from '@/components/ui/select'
import { Input } from '@/components/ui/input'
import { DataTable } from '@/components/ui/data-table'
import { Card, CardContent, CardHeader } from '@/components/ui/card'
import { Button } from '@/components/ui/button'

export const StudentAttendancePage = () => {
  const [date, setDate] = React.useState(new Date())
  const [search, setSearch] = React.useState('')
  const [selectedClassId, setSelectedClassId] = React.useState<string>('')
  const [selectedAttendance, setSelectedAttendance] =
    React.useState<StudentAttendanceWithMember | null>(null)
  const [isDialogOpen, setIsDialogOpen] = React.useState(false)
  const [shouldFetchReport, setShouldFetchReport] = React.useState(false)

  const formattedDateForApi = format(date, 'yyyy-MM-dd')
  const formattedDateForDisplay = format(date, 'EEEE, dd MMMM')

  const { data: classesData, isLoading: isClassesLoading } = useClasses()
  const { data: studentsData, isLoading: isStudentsLoading } =
    useStudentsInClass(selectedClassId, formattedDateForApi)
  const {
    isLoading: isAttendanceLoading,
    refetch: refetchAttendance,
  } = useStudentAttendance(selectedClassId, formattedDateForApi)
  const { data: reportData, isLoading: isReportLoading } =
    useGenerateStudentAttendanceReport(
      selectedClassId,
      formattedDateForApi,
      formattedDateForApi,
      shouldFetchReport,
    )

  const handlePrevDay = () => setDate((d) => subDays(d, 1))
  const handleNextDay = () => setDate((d) => addDays(d, 1))

  const handleMarkAttendance = (attendance: StudentAttendanceWithMember) => {
    setSelectedAttendance(attendance)
    setIsDialogOpen(true)
  }

  const handleExportReport = () => {
    if (selectedClassId) {
      setShouldFetchReport(true)
    } else {
      // Optionally, show a toast or alert if no class is selected
      console.warn('Please select a class to generate the report.')
    }
  }

  React.useEffect(() => {
    if (reportData && shouldFetchReport) {
      const headers = Object.keys(reportData[0]).join(',')
      const csv = reportData
        .map((row) => Object.values(row).join(','))
        .join('\n')
      const fullCsv = `${headers}\n${csv}`
      const blob = new Blob([fullCsv], { type: 'text/csv' })
      const url = URL.createObjectURL(blob)
      const a = document.createElement('a')
      a.href = url
      a.download = `student_attendance_report_${selectedClassId}_${formattedDateForApi}.csv`
      document.body.appendChild(a)
      a.click()
      document.body.removeChild(a)
      URL.revokeObjectURL(url)
      setShouldFetchReport(false)
    }
  }, [reportData, shouldFetchReport, selectedClassId, formattedDateForApi])

  const columns = React.useMemo(() => {
    return studentAttendanceColumns.map((col) => ({
      ...col,
      meta: {
        onMarkAttendance: handleMarkAttendance,
      },
    }))
  }, [])

  const mergedData: Array<StudentAttendanceWithMember> = React.useMemo(() => {
    if (!studentsData) return []
    return studentsData.map((attendance) => {
      return {
        ...attendance,
      }
    })
  }, [studentsData])

  const filteredData = React.useMemo(() => {
    return mergedData.filter(
      (item) =>
        item.student?.name_english
          .toLowerCase()
          .includes(search.toLowerCase()) ||
        item.student?.admission_number
          .toLowerCase()
          .includes(search.toLowerCase()),
    )
  }, [mergedData, search])

  // Set first class as default if not selected
  React.useEffect(() => {
    if (!selectedClassId && classesData?.data?.[0]) {
      setSelectedClassId(classesData.data[0].id)
    }
  }, [classesData, selectedClassId])

  if (isClassesLoading) {
    return (
      <div className="flex h-[400px] items-center justify-center">
        <Spinner className="size-8" />
      </div>
    )
  }

  return (
    <div className="space-y-6 p-6">
      <div className="flex flex-col space-y-4 md:flex-row md:items-center md:justify-between md:space-y-0">
        <div className="flex items-center gap-4">
          <h1 className="text-2xl font-black">Student Attendance</h1>
          <div className="flex items-center gap-1 rounded-xl bg-background border p-1 shadow-sm">
            <Button
              variant="ghost"
              size="icon"
              className="size-8 rounded-lg"
              onClick={handlePrevDay}
            >
              <HugeiconsIcon icon={ArrowLeft01Icon} className="size-4" />
            </Button>
            <span className="px-2 text-sm font-bold min-w-[140px] text-center">
              {formattedDateForDisplay}
            </span>
            <Button
              variant="ghost"
              size="icon"
              className="size-8 rounded-lg"
              onClick={handleNextDay}
            >
              <HugeiconsIcon icon={ArrowRight01Icon} className="size-4" />
            </Button>
          </div>
        </div>
        <div className="flex items-center gap-3">
          <Button
            variant="outline"
            className="rounded-xl border-2 font-bold h-10"
            onClick={handleExportReport}
            disabled={isReportLoading || !selectedClassId}
          >
            {isReportLoading ? (
              <Spinner className="mr-2 size-4" />
            ) : (
              <HugeiconsIcon icon={Download02Icon} className="mr-2 size-4" />
            )}
            Export Class Report
          </Button>
        </div>
      </div>

      <div className="flex items-center gap-4">
        <div className="flex items-center gap-2 rounded-xl bg-background border p-1 shadow-sm">
          <HugeiconsIcon
            icon={Book01Icon}
            className="ml-2 size-4 text-muted-foreground"
          />
          <Select
            value={selectedClassId}
            onValueChange={(value) => setSelectedClassId(value ?? '')}
          >
            <SelectTrigger className="h-8 border-none bg-transparent text-[11px] font-black uppercase tracking-wider focus:ring-0 w-[180px]">
              <SelectValue placeholder="Select Class" />
            </SelectTrigger>
            <SelectContent>
              {classesData?.data?.map((cls) => (
                <SelectItem key={cls.id} value={cls.id}>
                  {cls.section_name} ({cls.medium})
                </SelectItem>
              ))}
            </SelectContent>
          </Select>
        </div>
      </div>

      <AttendanceSummaryCards attendanceRecords={mergedData} />

      <Card className="border-none shadow-xl overflow-hidden bg-card">
        <CardHeader className="flex flex-col space-y-4 border-b bg-muted/20 px-6 py-5 lg:flex-row lg:items-center lg:justify-between lg:space-y-0">
          <div className="flex flex-wrap items-center gap-3">
            <div className="relative group">
              <HugeiconsIcon
                icon={Search01Icon}
                className="absolute left-3 top-1/2 size-4 -translate-y-1/2 text-muted-foreground transition-colors group-focus-within:text-primary"
              />
              <Input
                placeholder="Search student..."
                className="w-72 border-none bg-background/50 pl-10 ring-1 ring-border focus-visible:ring-2 focus-visible:ring-primary shadow-sm rounded-xl h-10"
                value={search}
                onChange={(e) => setSearch(e.target.value)}
              />
            </div>

            <Button
              variant="outline"
              size="sm"
              className="h-10 rounded-xl font-bold"
            >
              <HugeiconsIcon icon={FilterIcon} className="mr-2 size-4" />
              Advance Filter
            </Button>
          </div>
          <Button
            variant="outline"
            size="icon"
            className="h-10 w-10 rounded-xl shadow-sm transition-transform active:scale-95"
            onClick={() => refetchAttendance()}
          >
            <HugeiconsIcon icon={RefreshIcon} className="size-4" />
          </Button>
        </CardHeader>
        <CardContent className="p-0">
          {isStudentsLoading || isAttendanceLoading ? (
            <div className="flex h-32 items-center justify-center">
              <Spinner className="size-6" />
            </div>
          ) : (
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
          )}
        </CardContent>
      </Card>

      <MarkStudentAttendanceDialog
        open={isDialogOpen}
        onOpenChange={setIsDialogOpen}
        attendance={selectedAttendance}
        date={formattedDateForApi}
        classId={selectedClassId}
      />
    </div>
  )
}
