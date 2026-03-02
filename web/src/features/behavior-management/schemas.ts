import { z } from 'zod'
import { zRecordBehaviorIncidentRequest } from '@/lib/api/zod.gen'

export const behaviorIncidentTypeSchema = z.object({
  type_name: z.string().min(1, 'Type name is required'),
  description: z.string().optional(),
  default_points: z.number().int(),
})

export type BehaviorIncidentTypeFormValues = z.infer<
  typeof behaviorIncidentTypeSchema
>

export const behaviorIncidentSchema = zRecordBehaviorIncidentRequest

export type BehaviorIncidentFormValues = z.infer<typeof behaviorIncidentSchema>
