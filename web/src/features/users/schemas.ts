import { z } from 'zod'
import { zRoleEnum } from '@/lib/api/zod.gen'

// Note: The API doesn't have a proper user bulk update schema. Using a custom schema.
export const bulkUpdateSchema = z
  .object({
    updates: z
      .array(
        z.object({
          id: z.string(),
          data: z.object({
            is_verified: z.boolean().optional(),
            lockout_until: z.string().nullable().optional(),
            role: zRoleEnum.optional(),
          }),
        }),
      )
      .optional(),
    roles: z.array(zRoleEnum, { message: 'Invalid role provided' }).optional(),
  })
  .partial()

export type BulkUpdateValues = z.infer<typeof bulkUpdateSchema>

// Note: zUpdateUserRequest doesn't exist in the API. Using a placeholder schema.
const zUpdateUserRequest = z.object({
  is_verified: z.boolean().optional(),
  lockout_until: z.string().nullable().optional(),
  role: zRoleEnum.optional(),
})

export const updateUserSchema = zUpdateUserRequest
  .extend({
    email: z.string().email('Please enter a valid email address').optional(),
    roles: z.array(zRoleEnum, { message: 'Invalid role provided' }).optional(),
  })
  .partial()

export type UpdateUserValues = z.infer<typeof updateUserSchema>
