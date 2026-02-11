import { z } from 'zod'
import { zBulkUpdateRequest, zUpdateUserRequest, zRoleEnum } from '@/lib/api/zod.gen'

export const bulkUpdateSchema = zBulkUpdateRequest.extend({
  roles: z.array(zRoleEnum, { message: 'Invalid role provided' }).optional(),
}).partial();

export type BulkUpdateValues = z.infer<typeof bulkUpdateSchema>

export const updateUserSchema = zUpdateUserRequest.extend({
  email: z.string().email('Please enter a valid email address').optional(),
  roles: z.array(zRoleEnum, { message: 'Invalid role provided' }).optional(),
}).partial();

export type UpdateUserValues = z.infer<typeof updateUserSchema>
