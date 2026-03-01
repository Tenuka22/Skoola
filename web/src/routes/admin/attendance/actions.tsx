import { createFileRoute } from '@tanstack/react-router'
import { AttendanceActionsPage } from '@/features/attendance/actions/attendance-actions-page'

export const Route = createFileRoute('/admin/attendance/actions')({
  component: AttendanceActionsPage,
})
