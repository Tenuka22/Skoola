import type { z } from 'zod'
import { zCreateLessonProgressRequest } from '@/lib/api/zod.gen'

export const lessonProgressSchema = zCreateLessonProgressRequest

export type LessonProgressFormValues = z.infer<typeof lessonProgressSchema>
