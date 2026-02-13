import { z } from 'zod'
import type { EmploymentStatus, StaffType } from '@/lib/api/types.gen'
import {
  zCreateStaffRequest,
  zEmploymentStatus,
  zStaffType,
  zUpdateStaffRequest,
} from '@/lib/api/zod.gen'

export const staffTypeSchema = zStaffType
export { StaffType }

export const employmentStatusSchema = zEmploymentStatus
export { EmploymentStatus }

export const createStaffSchema = zCreateStaffRequest.extend({
  employee_id: z.string().min(1, 'Employee ID is required'),
  name: z.string().min(1, 'Name is required'),
  email: z.string().email('Invalid email address'),
  phone: z.string().min(1, 'Phone number is required'),
  nic: z.string().min(1, 'NIC is required'),
  dob: z.string().min(1, 'Date of birth is required'),
  gender: z.string().min(1, 'Gender is required'),
  address: z.string().min(1, 'Address is required'),
})

export type CreateStaffValues = z.infer<typeof createStaffSchema>

export const updateStaffSchema = zUpdateStaffRequest.extend({
  employee_id: z.string().min(1, 'Employee ID is required').optional(),
  name: z.string().min(1, 'Name is required').optional(),
  email: z.string().email('Invalid email address').optional(),
  phone: z.string().min(1, 'Phone number is required').optional(),
  nic: z.string().min(1, 'NIC is required').optional(),
  dob: z.string().min(1, 'Date of birth is required').optional(),
  gender: z.string().min(1, 'Gender is required').optional(),
  address: z.string().min(1, 'Address is required').optional(),
})

export type UpdateStaffValues = z.infer<typeof updateStaffSchema>
