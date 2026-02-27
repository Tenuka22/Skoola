import { z } from 'zod'
import {
  zBulkUpdateStaffRequest,
  zCreateStaffRequest,
  zEmploymentStatus,
  zGender,
  zStaffType,
} from '@/lib/api/zod.gen'

export const staffFormSchema = zCreateStaffRequest.extend({
  id: z.string().min(1, 'ID is required'),
  name: z.string().min(2, 'Name must be at least 2 characters'),
  email: z.string().email('Invalid email address'),
  phone: z.string().min(10, 'Phone number must be at least 10 characters'),
  nic: z.string().min(10, 'NIC must be at least 10 characters'),
  address: z.string().min(5, 'Address must be at least 5 characters'),
  gender: zGender,
  staff_type: zStaffType,
  employment_status: zEmploymentStatus,
})

export const bulkEditStaffFormSchema = zBulkUpdateStaffRequest.pick({
  staff_type: true,
  employment_status: true,
})

export type StaffFormValues = z.infer<typeof staffFormSchema>
export type BulkEditStaffFormValues = z.infer<typeof bulkEditStaffFormSchema>
