import type { z } from 'zod'
import { zClassResponse, zCreateClassRequest } from '@/lib/api/zod.gen'

export const classFormSchema = zCreateClassRequest

export type ClassFormValues = z.infer<typeof classFormSchema>

export const classSchema = zClassResponse
