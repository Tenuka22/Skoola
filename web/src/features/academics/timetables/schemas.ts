import type { z } from 'zod'
import { zCreateTimetableRequest, zTimetableResponse } from '@/lib/api/zod.gen'

export const timetableEntryFormSchema = zCreateTimetableRequest

export type TimetableEntryFormValues = z.infer<typeof timetableEntryFormSchema>

export const timetableEntrySchema = zTimetableResponse
