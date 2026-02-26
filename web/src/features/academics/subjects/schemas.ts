import type { z } from 'zod'
import {
  zAssignSubjectToGradeRequest,
  zAssignSubjectToStreamRequest,
  zCreateSubjectRequest,
  zEnrollStudentInSubjectRequest,
  zSubjectResponse,
} from '@/lib/api/zod.gen'

export const subjectFormSchema = zCreateSubjectRequest

export type SubjectFormValues = z.infer<typeof subjectFormSchema>

export const subjectSchema = zSubjectResponse

export const assignSubjectToGradeSchema = zAssignSubjectToGradeRequest

export const assignSubjectToStreamSchema = zAssignSubjectToStreamRequest

export const enrollStudentInSubjectSchema = zEnrollStudentInSubjectRequest
