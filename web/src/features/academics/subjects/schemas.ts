import { z } from 'zod'
import { zCreateSubjectRequest, zSubjectResponse } from '@/lib/api/zod.gen'

export const subjectFormSchema = zCreateSubjectRequest

export type SubjectFormValues = z.infer<typeof subjectFormSchema>

export const subjectSchema = zSubjectResponse

export const assignSubjectToGradeSchema = z.object({
  subject_id: z.string(),
  grade_level_id: z.string(),
})

export const assignSubjectToStreamSchema = z.object({
  subject_id: z.string(),
  stream_id: z.string(),
})

export const enrollStudentInSubjectSchema = z.object({
  subject_id: z.string(),
  student_id: z.string(),
  academic_year_id: z.string(),
})
