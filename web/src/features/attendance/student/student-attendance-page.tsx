import { Suspense, useMemo, useState } from 'react'
import { useMutation, useQuery, useQueryClient } from '@tanstack/react-query'
import { format } from 'date-fns'
import { stringify as serializeCsv } from 'csv-stringify/browser/esm/sync'
import { StudentAttendanceFilters } from './components/student-attendance-filters'
import { StudentAttendanceTable } from './components/student-attendance-table'
import { StudentAttendanceActions } from './components/student-attendance-actions'
import type { LocalEnrichedStudentAttendance } from './components/student-attendance-columns'
import type {
  AttendanceStatus,
  EnrichedStudentAttendance,
  GetEnrichedStudentListData,
  MarkStudentAttendanceRequest, // Import this for type safety
} from '@/lib/api/types.gen'
import type { Options } from '@/lib/api/sdk.gen' // Import Options type directly from sdk.gen
import { FullPageSpinner } from '@/components/ui/full-page-spinner'
import {
  bulkMarkStudentAttendanceMutation,
  getAllStudentsOptions,
  getEnrichedStudentListOptions,
  getEnrichedStudentListQueryKey, // Import this
} from '@/lib/api/@tanstack/react-query.gen'
import { Heading, Stack, Text } from '@/components/primitives'

type AttendanceState = Record<string, AttendanceStatus>
type EnrichedStudentListQueryKeyType = ReturnType<
  typeof getEnrichedStudentListQueryKey
>

export function StudentAttendancePage() {
  const queryClient = useQueryClient()
  const [selectedClassId, setSelectedClassId] = useState<string | undefined>()
  const [selectedDate, setSelectedDate] = useState<Date>(new Date())

  const [attendance, setAttendance] = useState<AttendanceState>({})
  const [isDirty, setIsDirty] = useState(false)

  const queryKey = useMemo<EnrichedStudentListQueryKeyType>(() => {
    const options: Options<GetEnrichedStudentListData> = {
      path: {
        class_id: selectedClassId || '',
        date: format(selectedDate, 'yyyy-MM-dd'),
      },
    }
    return getEnrichedStudentListQueryKey(options)
  }, [selectedClassId, selectedDate])

  const queryFn = useMemo(() => {
    if (!selectedClassId) {
      return () => Promise.resolve([])
    }
    const options: Options<GetEnrichedStudentListData> = {
      path: {
        class_id: selectedClassId,
        date: format(selectedDate, 'yyyy-MM-dd'),
      },
    }
    return getEnrichedStudentListOptions(options).queryFn
  }, [selectedClassId, selectedDate])

  const enrichedListQueryResult = useQuery<
    Array<EnrichedStudentAttendance>, // TQueryFnData
    Error, // TError
    Array<EnrichedStudentAttendance>, // TData
    EnrichedStudentListQueryKeyType // TQueryKey
  >({
    queryKey: queryKey,
    queryFn: queryFn,
    enabled: !!selectedClassId,
  })

  const enrichedListForExport = enrichedListQueryResult.data || []
  const isExportDataFetching = enrichedListQueryResult.isFetching

  const { data: allStudents } = useQuery(getAllStudentsOptions())

  const displayListForExport: Array<LocalEnrichedStudentAttendance> =
    enrichedListForExport.map((attendanceRecord) => {
      const studentDetails = allStudents?.data?.find(
        (student) => student.id === attendanceRecord.student_id,
      )
      return {
        ...attendanceRecord,
        admission_number: studentDetails
          ? studentDetails.admission_number
          : 'N/A',
        profile_photo_url: studentDetails?.profile_photo_url,
      }
    })

  const { mutate: bulkMark, isPending: isSaving } = useMutation({
    ...bulkMarkStudentAttendanceMutation(),
    onSuccess: () => {
      setIsDirty(false)
      // Invalidate the query to refetch the latest attendance data
      if (selectedClassId) {
        void queryClient.invalidateQueries({
          queryKey: queryKey,
        })
      }
    },
  })

  const handleStatusChange = (studentId: string, status: AttendanceStatus) => {
    setAttendance((prev) => ({ ...prev, [studentId]: status }))
    setIsDirty(true)
  }

  const handleBulkMark = (status: AttendanceStatus) => {
    const newAttendance: AttendanceState = {}
    if (enrichedListForExport) {
      // Use enrichedListForExport which is always fetched
      enrichedListForExport.forEach((student) => {
        newAttendance[student.student_id] = status
      })
    }
    setAttendance(newAttendance)
    setIsDirty(true)
  }

  const handleSave = () => {
    if (!selectedClassId) return // Ensure selectedClassId is defined

    const records: Array<MarkStudentAttendanceRequest> = Object.entries(
      attendance,
    ).map(([student_id, status]) => ({
      student_id,
      status,
      date: format(selectedDate, 'yyyy-MM-dd'),
      class_id: selectedClassId, // Add class_id
      marked_by: 'admin', // Placeholder for marked_by
    }))

    bulkMark({
      body: {
        attendance_records: records,
      },
    })
  }

  const handleExport = () => {
    if (!displayListForExport || !selectedClassId) return // Ensure selectedClassId is defined

    const dataToExport = displayListForExport.map(
      (student: LocalEnrichedStudentAttendance) => ({
        'Student ID': student.student_id,
        'Student Name': student.student_name,
        'Admission Number': student.admission_number,
        Status: attendance[student.student_id] || student.status || 'N/A', // Use current attendance state
        'Medical Alerts': student.medical_alerts || 'None',
      }),
    )

    const csv = serializeCsv(dataToExport, { header: true })
    const blob = new Blob([csv], { type: 'text/csv;charset=utf-8;' })
    const link = document.createElement('a')
    const url = URL.createObjectURL(blob)
    link.setAttribute('href', url)
    link.setAttribute(
      'download',
      `attendance-${selectedClassId}-${format(selectedDate, 'yyyy-MM-dd')}.csv`,
    )
    document.body.appendChild(link)
    link.click()
    document.body.removeChild(link)
  }

  return (
    <Stack gap={4} p={8} className="h-full">
      <Stack gap={1}>
        <Heading size="h2">Student Attendance</Heading>
        <Text muted as="p">
          Mark and manage attendance for students by class.
        </Text>
      </Stack>

      <Suspense fallback={<FullPageSpinner />}>
        <StudentAttendanceFilters
          selectedClassId={selectedClassId}
          onClassChange={setSelectedClassId}
          selectedDate={selectedDate}
          onDateChange={setSelectedDate}
          onExport={handleExport}
          isExporting={isExportDataFetching}
        />
      </Suspense>

      <StudentAttendanceActions
        onBulkMark={handleBulkMark}
        onSave={handleSave}
        isSaving={isSaving}
        isDirty={isDirty}
      />

      {selectedClassId ? (
        <Suspense fallback={<FullPageSpinner />}>
          <StudentAttendanceTable
            classId={selectedClassId}
            date={selectedDate}
            attendance={attendance}
            setAttendance={setAttendance}
            onStatusChange={handleStatusChange}
            isLoading={isExportDataFetching} // Pass loading state from query
          />
        </Suspense>
      ) : (
        <div className="flex flex-1 items-center justify-center rounded-md border border-dashed">
          <Text muted>Select a class and date to view attendance.</Text>
        </div>
      )}
    </Stack>
  )
}
