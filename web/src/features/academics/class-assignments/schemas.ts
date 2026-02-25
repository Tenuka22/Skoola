import { z } from 'zod'
import { zClassSubjectTeacherResponse } from '@/lib/api/zod.gen'

export const assignTeacherFormSchema = z.object({
  class_id: z.string().min(1, 'Class is required'),
  subject_id: z.string().min(1, 'Subject is required'),
  teacher_id: z.string().min(1, 'Teacher is required'),
  academic_year_id: z.string().min(1, 'Academic Year is required'),
})

export type AssignTeacherFormValues = z.infer<typeof assignTeacherFormSchema>

export const updateTeacherAssignmentFormSchema = z.object({
  teacher_id: z.string().min(1, 'Teacher is required'),
})

export type UpdateTeacherAssignmentFormValues = z.infer<
  typeof updateTeacherAssignmentFormSchema
>

export const classSubjectTeacherAssignmentSchema = zClassSubjectTeacherResponse
