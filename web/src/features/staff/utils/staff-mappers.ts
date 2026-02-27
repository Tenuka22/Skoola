import type { StaffFormValues } from '../schemas'
import type { StaffResponse } from '@/lib/api/types.gen'

export const mapStaffResponseToCreateStaffValues = (
  staff: StaffResponse,
): Partial<StaffFormValues> => {
  return {
    employee_id: staff.employee_id,
    employment_status: staff.employment_status,
    staff_type: staff.staff_type,
    name: staff.name,
    email: staff.email,
    phone: staff.phone,
    nic: staff.nic,
    dob: staff.dob,
    gender: staff.gender as StaffFormValues['gender'],
    address: staff.address,
  }
}
