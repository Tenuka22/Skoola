import type {
  AttendanceStatus,
  ExitReason,
  StaffAttendanceResponse,
  StaffResponse,
  StudentAttendanceResponse,
} from '@/lib/api/types.gen'

export const ALL_ATTENDANCE_STATUSES: Array<AttendanceStatus> = [
  'Present',
  'Absent',
  'Late',
  'Excused',
  'HalfDay',
  'SchoolBusiness',
]

export function isAttendanceStatus(value: string): value is AttendanceStatus {
  return ALL_ATTENDANCE_STATUSES.some((v) => v === value)
}

export const ALL_EXIT_REASONS: Array<ExitReason> = [
  'Medical',
  'Personal',
  'Disciplinary',
  'Dismissal',
  'FamilyEvent',
  'Other',
]

export function isExitReason(value: string): value is ExitReason {
  return ALL_EXIT_REASONS.some((v) => v === value)
}

export interface StaffAttendanceWithMember extends StaffAttendanceResponse {
  staff: StaffResponse
}

export interface StudentAttendanceWithMember extends StudentAttendanceResponse {
  student?: {
    admission_number?: string
    id: string
    name_english?: string
  }
}
