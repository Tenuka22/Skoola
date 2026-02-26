import type {
  AttendanceStatus,
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
