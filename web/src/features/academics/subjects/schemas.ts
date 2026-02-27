import { z } from 'zod'
import {
  zAssignSubjectToGradeRequest,
  zAssignSubjectToStreamRequest,
  zCreateSubjectRequest,
  zEnrollStudentInSubjectRequest,
  zSubjectResponse,
} from '@/lib/api/zod.gen'

export const subjectFormSchema = zCreateSubjectRequest.extend({
  id: z.string().min(1, 'ID is required'),
  subject_code: z.string().min(2, 'Subject code must be at least 2 characters'),
  subject_name_en: z
    .string()
    .min(2, 'English name must be at least 2 characters'),
  is_core: z.boolean(),
})

export type SubjectFormValues = z.infer<typeof subjectFormSchema>

export const subjectSchema = zSubjectResponse

export const assignSubjectToGradeSchema = zAssignSubjectToGradeRequest

export const assignSubjectToStreamSchema = zAssignSubjectToStreamRequest

export const enrollStudentInSubjectSchema = zEnrollStudentInSubjectRequest
