import type { z } from 'zod'
import {
  zCreateSyllabusRequest,
  zUpdateSyllabusRequest,
} from '@/lib/api/zod.gen'

export const syllabusTopicSchema = zCreateSyllabusRequest
export const updateSyllabusTopicSchema = zUpdateSyllabusRequest

export type SyllabusTopicFormValues = z.infer<typeof syllabusTopicSchema>
export type UpdateSyllabusTopicFormValues = z.infer<
  typeof updateSyllabusTopicSchema
>
