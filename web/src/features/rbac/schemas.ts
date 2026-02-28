import { z } from 'zod'
import {
  zCreatePermissionSetRequest,
  zCreateRoleSetRequest,
  zUpdatePermissionSetRequest,
  zUpdateRoleSetRequest,
} from '@/lib/api/zod.gen'

export const createPermissionSetSchema = zCreatePermissionSetRequest.extend({
  name: z.string().min(3, 'Name must be at least 3 characters'),
  description: z.string().min(5, 'Description must be at least 5 characters'),
})

export const updatePermissionSetSchema = zUpdatePermissionSetRequest.extend({
  name: z.string().min(3, 'Name must be at least 3 characters').optional(),
  description: z
    .string()
    .min(5, 'Description must be at least 5 characters')
    .optional(),
})

export type CreatePermissionSetInput = z.infer<typeof createPermissionSetSchema>
export type UpdatePermissionSetInput = z.infer<typeof updatePermissionSetSchema>

export const createRoleSetSchema = zCreateRoleSetRequest.extend({
  name: z.string().min(3, 'Name must be at least 3 characters'),
  description: z.string().min(5, 'Description must be at least 5 characters'),
})

export const updateRoleSetSchema = zUpdateRoleSetRequest.extend({
  name: z.string().min(3, 'Name must be at least 3 characters').optional(),
  description: z
    .string()
    .min(5, 'Description must be at least 5 characters')
    .optional(),
})

export type CreateRoleSetInput = z.infer<typeof createRoleSetSchema>
export type UpdateRoleSetInput = z.infer<typeof updateRoleSetSchema>
