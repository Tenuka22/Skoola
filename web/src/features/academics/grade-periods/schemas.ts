import { z } from 'zod'
import {
  zCreateGradePeriodRequest,
  zUpdateGradePeriodRequest,
} from '@/lib/api/zod.gen'

export const gradePeriodFormSchema = zCreateGradePeriodRequest.extend({
  grade_id: z.string().min(1, 'Grade is required'),
  period_number: z.number().min(1, 'Period number must be at least 1'),
  start_time: z.string().regex(/^([01]\d|2[0-3]):([0-5]\d):([0-5]\d)$/, 'Invalid time format (HH:MM:SS)'),
  end_time: z.string().regex(/^([01]\d|2[0-3]):([0-5]\d):([0-5]\d)$/, 'Invalid time format (HH:MM:SS)'),
  is_break: z.boolean().default(false),
})

export type GradePeriodFormValues = z.infer<typeof gradePeriodFormSchema>

export const updateGradePeriodFormSchema = zUpdateGradePeriodRequest.extend({
  period_number: z.number().min(1, 'Period number must be at least 1').optional(),
  start_time: z.string().regex(/^([01]\d|2[0-3]):([0-5]\d):([0-5]\d)$/, 'Invalid time format (HH:MM:SS)').optional(),
  end_time: z.string().regex(/^([01]\d|2[0-3]):([0-5]\d):([0-5]\d)$/, 'Invalid time format (HH:MM:SS)').optional(),
  is_break: z.boolean().optional(),
})

export type UpdateGradePeriodFormValues = z.infer<typeof updateGradePeriodFormSchema>
