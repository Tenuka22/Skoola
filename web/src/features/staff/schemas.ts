import { z } from 'zod'
import { zEmploymentStatus, zGender, zStaffType } from '@/lib/api/zod.gen'

export const createStaffSchema = z.object({
  name: z.string().min(2, 'Name must be at least 2 characters'),
  email: z.string().email('Please enter a valid email address').optional().or(z.literal('')),
  phone: z.string().optional().or(z.literal('')),
  dob: z.string().refine((val) => val && !Number.isNaN(new Date(val).getTime()), {
    message: 'Please select a valid date',
  }),
  gender: zGender,
  employee_id: z.string().min(1, 'Employee ID is required'),
  staff_type: zStaffType,
  employment_status: zEmploymentStatus.optional().or(z.literal('')),
  address: z.string().optional().or(z.literal('')),
  nic: z.string().optional().or(z.literal('')),
})

export const updateStaffSchema = createStaffSchema

export type CreateStaffValues = z.infer<typeof createStaffSchema>
export type UpdateStaffValues = z.infer<typeof updateStaffSchema>
