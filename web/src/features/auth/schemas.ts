import { z } from 'zod'
import { zLoginRequest, zRegisterRequest } from '@/lib/api/zod.gen'

export const loginSchema = zLoginRequest.extend({
  email: z.string().email('Please enter a valid email address'),
  password: z.string().min(6, 'Password must be at least 6 characters'),
})

export const signUpSchema = zRegisterRequest
  .extend({
    name: z.string().min(2, 'Name must be at least 2 characters'),
    email: z.string().email('Please enter a valid email address'),
    password: z.string().min(8, 'Password must be at least 8 characters'),
    confirmPassword: z.string(),
  })
  .refine((data) => data.password === data.confirmPassword, {
    message: "Passwords don't match",
    path: ['confirmPassword'],
  })

export type LoginFormValues = z.infer<typeof loginSchema>
export type SignUpFormValues = z.infer<typeof signUpSchema>
