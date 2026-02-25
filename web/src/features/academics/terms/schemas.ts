import type { z } from 'zod'
import { zCreateTermRequest } from '@/lib/api/zod.gen'

export const termFormSchema = zCreateTermRequest

export type TermFormValues = z.infer<typeof termFormSchema>
