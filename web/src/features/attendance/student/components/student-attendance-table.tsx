import { useSuspenseQuery } from '@tanstack/react-query'
import { format } from 'date-fns'
import { useEffect } from 'react'
import { getStudentAttendanceColumns } from './student-attendance-columns'
import type { LocalEnrichedStudentAttendance } from './student-attendance-columns'
import type { AttendanceStatus } from '@/lib/api/types.gen'
import { DataTable } from '@/components/ui/data-table'
import {
  getAllStudentsOptions,
  getEnrichedStudentListOptions,
} from '@/lib/api/@tanstack/react-query.gen'

type AttendanceState = Record<string, AttendanceStatus>

interface StudentAttendanceTableProps {
  classId: string
  date: Date
  attendance: AttendanceState
  setAttendance: React.Dispatch<React.SetStateAction<AttendanceState>>
  onStatusChange: (studentId: string, status: AttendanceStatus) => void
  isLoading: boolean
}

export function StudentAttendanceTable({
  classId,
  date,
  attendance,
  setAttendance,
  onStatusChange,
  isLoading,
}: StudentAttendanceTableProps) {
  const { data: enrichedList, isFetching: isEnrichedListFetching } =
    useSuspenseQuery(
      getEnrichedStudentListOptions({
        path: { class_id: classId, date: format(date, 'yyyy-MM-dd') },
      }),
    )
  const { data: allStudents } = useSuspenseQuery(getAllStudentsOptions())

  const displayList: Array<LocalEnrichedStudentAttendance> = enrichedList.map(
    (attendanceRecord) => {
      const studentDetails = allStudents.data?.find(
        (student) => student.id === attendanceRecord.student_id,
      )
      return {
        ...attendanceRecord,
        admission_number: studentDetails?.admission_number || '',
        profile_photo_url: studentDetails?.profile_photo_url,
      }
    },
  )

  useEffect(() => {
    const initialAttendance: AttendanceState = {}
    displayList.forEach((student) => {
      initialAttendance[student.student_id] = student.status || 'Present' // Use student.status
    })
    setAttendance(initialAttendance)
  }, [displayList, setAttendance])

  const columns = getStudentAttendanceColumns({ attendance, onStatusChange })

  const dataForTable = displayList.map((student) => ({
    ...student,
    id: student.student_id, // Map student_id to id for DataTable
    status: attendance[student.student_id] || student.status || 'Present', // Ensure status reflects local state
  }))

  return (
    <DataTable<
      LocalEnrichedStudentAttendance & { id: string | number },
      unknown
    >
      columns={columns}
      data={dataForTable}
      isLoading={isLoading || isEnrichedListFetching}
      pageIndex={0} // For now, client-side pagination
      pageSize={dataForTable.length} // Show all for now
      pageCount={1} // Only one page for now
      canPreviousPage={false}
      canNextPage={false}
      fetchNextPage={() => {}}
      fetchPreviousPage={() => {}}
    />
  )
}
