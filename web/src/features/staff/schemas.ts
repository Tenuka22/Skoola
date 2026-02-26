import type { z } from 'zod'
import { zBulkUpdateStaffRequest, zCreateStaffRequest } from '@/lib/api/zod.gen'

export const staffFormSchema = zCreateStaffRequest
export const bulkEditStaffFormSchema = zBulkUpdateStaffRequest.pick({
  staff_type: true,
  employment_status: true,
})

export type StaffFormValues = z.infer<typeof staffFormSchema>
export type BulkEditStaffFormValues = z.infer<typeof bulkEditStaffFormSchema>
