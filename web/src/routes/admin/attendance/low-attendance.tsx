import { createFileRoute } from '@tanstack/react-router'
import { LowAttendancePage } from '@/features/attendance/reports/low-attendance-page'

export const Route = createFileRoute('/admin/attendance/low-attendance')({
  component: LowAttendancePage,
})
