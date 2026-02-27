import { z } from 'zod'
import { zClassResponse, zCreateClassRequest, zMedium } from '@/lib/api/zod.gen'

export const classFormSchema = zCreateClassRequest.extend({
  id: z.string().min(1, 'ID is required'),
  section_name: z.string().min(1, 'Section name is required'),
  grade_id: z.string().min(1, 'Grade is required'),
  academic_year_id: z.string().min(1, 'Academic year is required'),
  max_capacity: z.number().min(1, 'Capacity must be at least 1').max(100, 'Capacity cannot exceed 100'),
  medium: zMedium,
  room_number: z.string().optional().nullable(),
  class_teacher_id: z.string().optional().nullable(),
})

export type ClassFormValues = z.infer<typeof classFormSchema>

export const classSchema = zClassResponse
