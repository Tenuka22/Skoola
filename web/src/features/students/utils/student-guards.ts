import type { StudentStatus } from '@/lib/api/types.gen'

const STUDENT_STATUSES: Array<StudentStatus> = [
  'Active',
  'Transferred',
  'Graduated',
  'Withdrawn',
  'Suspended',
]

export const isStudentStatus = (value: string): value is StudentStatus =>
  STUDENT_STATUSES.includes(value)
