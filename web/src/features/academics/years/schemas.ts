import { z } from 'zod'
import { zCreateAcademicYearRequest } from '@/lib/api/zod.gen'

export const academicYearFormSchema = zCreateAcademicYearRequest.extend({
  id: z.string().min(1, 'ID is required'),
  name: z
    .string()
    .min(4, 'Name must be at least 4 characters (e.g. 2024-2025)'),
  year_start: z.number().optional(),
  year_end: z.number().optional(),
  start_date: z.string().min(1, 'Start date is required'),
  end_date: z.string().min(1, 'End date is required'),
  current: z.boolean().optional().nullable(),
})

export const bulkEditAcademicYearFormSchema = z.object({
  start_date: z.string().optional(),
  end_date: z.string().optional(),
})

export type AcademicYearFormValues = z.infer<typeof academicYearFormSchema>
export type BulkEditAcademicYearFormValues = z.infer<
  typeof bulkEditAcademicYearFormSchema
>
