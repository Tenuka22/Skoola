import type { StaffResponse } from '@/lib/api/types.gen'
import type { CreateStaffValues } from '../schemas'

export const mapStaffResponseToCreateStaffValues = (
  staff: StaffResponse,
): Partial<CreateStaffValues> => {
  return {
    employee_id: staff.employee_id,
    employment_status: staff.employment_status,
    staff_type: staff.staff_type,
    name: staff.name,
    email: staff.email,
    phone: staff.phone,
    nic: staff.nic,
    dob: staff.dob,
    gender: staff.gender,
    address: staff.address,
  }
}
