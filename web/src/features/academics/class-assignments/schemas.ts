import type { z } from 'zod'
import {
  zClassSubjectTeacherResponse,
  zCreateClassSubjectTeacherRequest,
  zUpdateClassSubjectTeacherRequest,
} from '@/lib/api/zod.gen'

export const assignTeacherFormSchema = zCreateClassSubjectTeacherRequest

export type AssignTeacherFormValues = z.infer<typeof assignTeacherFormSchema>

export const updateTeacherAssignmentFormSchema =
  zUpdateClassSubjectTeacherRequest

export type UpdateTeacherAssignmentFormValues = z.infer<
  typeof updateTeacherAssignmentFormSchema
>

export const classSubjectTeacherAssignmentSchema = zClassSubjectTeacherResponse
