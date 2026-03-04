import type { z } from 'zod'
import {
  zCreateCurriculumStandardRequest,
  zUpdateCurriculumStandardRequest,
} from '@/lib/api/zod.gen'

export const curriculumStandardSchema = zCreateCurriculumStandardRequest
export const updateCurriculumStandardSchema = zUpdateCurriculumStandardRequest

export type CurriculumStandardFormValues = z.infer<
  typeof curriculumStandardSchema
>
export type UpdateCurriculumStandardFormValues = z.infer<
  typeof updateCurriculumStandardSchema
>
