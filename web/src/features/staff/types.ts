import type { EmploymentStatus, Gender, StaffResponse, StaffType } from '@/lib/api'

export type Staff = StaffResponse

export type StaffFormData = {
  name: string
  email?: string
  phone?: string
  dob: string
  gender: Gender
  employee_id: string
  staff_type: StaffType
  employment_status?: EmploymentStatus | ''
  address?: string
  nic?: string
  photo_url?: string
}

export type StaffFilters = {
  search?: string
  staff_type?: StaffType
  employment_status?: EmploymentStatus
  gender?: Gender
  sort_by?: string
  sort_order?: 'asc' | 'desc'
  page?: number
  limit?: number
}
