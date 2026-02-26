import type { EmploymentStatus, StaffType } from '@/lib/api/types.gen'
import { zEmploymentStatus, zStaffType } from '@/lib/api/zod.gen'

export const isStaffType = (value: string): value is StaffType => {
  return zStaffType.options.some((option) => option === value)
}

export const isEmploymentStatus = (
  value: string,
): value is EmploymentStatus => {
  return zEmploymentStatus.options.some((option) => option === value)
}
