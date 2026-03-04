import type { z } from 'zod'
import { zCreateExamRequest, zUpdateExamRequest } from '@/lib/api/zod.gen'

export const examSchema = zCreateExamRequest
export const updateExamSchema = zUpdateExamRequest

export type ExamFormValues = z.infer<typeof examSchema>
export type UpdateExamFormValues = z.infer<typeof updateExamSchema>
