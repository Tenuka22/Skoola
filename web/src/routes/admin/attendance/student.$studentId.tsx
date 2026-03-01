import { createFileRoute } from '@tanstack/react-router'
import { StudentAttendanceHistoryPage } from '@/features/attendance/student/components/student-attendance-history-page'

export const Route = createFileRoute('/admin/attendance/student/$studentId')({
  component: StudentAttendanceHistoryPage,
})
