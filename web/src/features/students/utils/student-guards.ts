import type { StudentStatus } from '@/lib/api/types.gen'
import { zStudentStatus } from '@/lib/api/zod.gen'

const STUDENT_STATUSES: Array<StudentStatus> = zStudentStatus.options

export const isStudentStatus = (value: string): value is StudentStatus =>
  STUDENT_STATUSES.some((status) => status === value)
