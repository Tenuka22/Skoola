import { z } from 'zod'

export const bulkUpdateSchema = z.object({
  is_verified: z.boolean().optional(),
  is_locked: z.boolean().optional(),
  roles: z.array(z.string()).optional(),
})

export type BulkUpdateValues = z.infer<typeof bulkUpdateSchema>

export const updateUserSchema = z.object({
  email: z.string().email().optional(),
  is_verified: z.boolean().optional(),
  is_locked: z.boolean().optional(),
  roles: z.array(z.string()).optional(),
})

export type UpdateUserValues = z.infer<typeof updateUserSchema>
