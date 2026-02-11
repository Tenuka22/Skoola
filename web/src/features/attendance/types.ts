import type { StaffAttendanceResponse, StaffResponse, StudentAttendanceResponse, StudentResponse } from '@/lib/api/types.gen';

export type StaffAttendanceWithMember = StaffAttendanceResponse & {
  staff?: StaffResponse;
};

export type StudentAttendanceWithMember = StudentAttendanceResponse & {
  student?: StudentResponse;
};

export interface StaffAttendanceStats {
  present: number;
  late: number;
  early: number;
  absent: number;
  no_clock_in: number;
  no_clock_out: number;
  invalid: number;
  day_off: number;
  time_off: number;
}
