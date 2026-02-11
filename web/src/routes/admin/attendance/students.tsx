import { createFileRoute } from '@tanstack/react-router';
import { StudentAttendancePage } from '@/features/attendance';

export const Route = createFileRoute('/admin/attendance/students')({
  component: StudentAttendancePage,
});
