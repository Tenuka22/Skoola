import type { z } from 'zod'
import {
  zCreateGradeLevelRequest,
  zGradeLevelResponse,
} from '@/lib/api/zod.gen'

export const gradeLevelFormSchema = zCreateGradeLevelRequest

export type GradeLevelFormValues = z.infer<typeof gradeLevelFormSchema>

export const gradeLevelSchema = zGradeLevelResponse
