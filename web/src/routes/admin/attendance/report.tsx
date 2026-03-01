import { createFileRoute } from '@tanstack/react-router'
import { AttendanceReportPage } from '@/features/attendance/reports/attendance-report-page'

export const Route = createFileRoute('/admin/attendance/report')({
  component: AttendanceReportPage,
})
