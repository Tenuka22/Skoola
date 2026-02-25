import { z } from 'zod'
import { zCreateAcademicYearRequest } from '@/lib/api/zod.gen'

export const academicYearFormSchema = zCreateAcademicYearRequest.extend({
  start_date: z.string().min(1, 'Start date is required'),
  end_date: z.string().min(1, 'End date is required'),
})

export const bulkEditAcademicYearFormSchema = z.object({
  start_date: z.string().optional(),
  end_date: z.string().optional(),
})

export type AcademicYearFormValues = z.infer<typeof academicYearFormSchema>
export type BulkEditAcademicYearFormValues = z.infer<
  typeof bulkEditAcademicYearFormSchema
>
