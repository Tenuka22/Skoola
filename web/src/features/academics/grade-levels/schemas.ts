import { z } from 'zod'
import {
  zCreateGradeLevelRequest,
  zGradeLevelResponse,
  zEducationLevel,
} from '@/lib/api/zod.gen'

export const gradeLevelFormSchema = zCreateGradeLevelRequest.extend({
  id: z.string().min(1, 'ID is required'),
  grade_name: z.string().min(1, 'Grade name is required'),
  grade_number: z.number().min(0, 'Grade number cannot be negative'),
  education_level: zEducationLevel,
})

export type GradeLevelFormValues = z.infer<typeof gradeLevelFormSchema>

export const gradeLevelSchema = zGradeLevelResponse
