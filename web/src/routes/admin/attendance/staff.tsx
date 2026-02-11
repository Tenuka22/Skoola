import { createFileRoute } from '@tanstack/react-router';
import { StaffAttendancePage } from '@/features/attendance';

export const Route = createFileRoute('/admin/attendance/staff')({
  component: StaffAttendancePage,
});
