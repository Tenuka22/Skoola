import { z } from 'zod'
import {
  zCreateStaffRequest,
  zEmploymentStatus,
  zStaffType,
} from '@/lib/api/zod.gen'

export const staffFormSchema = zCreateStaffRequest
export const bulkEditStaffFormSchema = z.object({
  staff_type: zStaffType.optional(),
  employment_status: zEmploymentStatus.optional(),
})

export type StaffFormValues = z.infer<typeof staffFormSchema>
export type BulkEditStaffFormValues = z.infer<typeof bulkEditStaffFormSchema>
