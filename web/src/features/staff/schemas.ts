import { z } from 'zod'

export const staffTypeSchema = z.enum(['Teaching', 'NonTeaching', 'Administrative'])
export const employmentStatusSchema = z.enum(['Permanent', 'Contract', 'Temporary'])

export const createStaffSchema = z.object({
  employee_id: z.string().min(1, 'Employee ID is required'),
  name: z.string().min(1, 'Name is required'),
  email: z.string().email('Invalid email address'),
  phone: z.string().min(1, 'Phone number is required'),
  nic: z.string().min(1, 'NIC is required'),
  dob: z.string().min(1, 'Date of birth is required'),
  gender: z.string().min(1, 'Gender is required'),
  address: z.string().min(1, 'Address is required'),
  staff_type: staffTypeSchema,
  employment_status: employmentStatusSchema,
})

export type CreateStaffValues = z.infer<typeof createStaffSchema>

export const updateStaffSchema = createStaffSchema.partial()

export type UpdateStaffValues = z.infer<typeof updateStaffSchema>
