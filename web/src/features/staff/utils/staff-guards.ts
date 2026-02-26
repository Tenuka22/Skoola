import type { EmploymentStatus, StaffType } from '@/lib/api/types.gen'

export const isStaffType = (value: string): value is StaffType => {
  return ['Teaching', 'NonTeaching', 'Administrative'].includes(value)
}

export const isEmploymentStatus = (
  value: string,
): value is EmploymentStatus => {
  return ['Permanent', 'Contract', 'OnLeave', 'Terminated'].includes(value)
}
