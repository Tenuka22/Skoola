import { z } from 'zod'
import type {
  Ethnicity,
  Gender,
  Religion,
  StudentStatus,
} from '@/lib/api/types.gen'
import {
  zCreateStudentRequest,
  zEthnicity,
  zGender,
  zReligion,
  zStudentStatus,
  zUpdateStudentRequest,
} from '@/lib/api/zod.gen'

export const studentStatusSchema = zStudentStatus
export { StudentStatus }

export const genderSchema = zGender
export { Gender }

export const ethnicitySchema = zEthnicity
export { Ethnicity }

export const religionSchema = zReligion
export { Religion }

export const createStudentSchema = zCreateStudentRequest.extend({
  admission_number: z.string().min(1, 'Admission number is required'),
  name_english: z.string().min(1, 'English name is required'),
  phone: z.string().min(1, 'Phone number is required'),
  nic_or_birth_certificate: z
    .string()
    .min(1, 'NIC or Birth Certificate is required'),
  dob: z.string().min(1, 'Date of birth is required'),
  address: z.string().min(1, 'Address is required'),
  email: z.string().email('Invalid email address').optional().nullable(),
  photo_url: z.string().url('Invalid URL').optional().nullable(),
})

export type CreateStudentValues = z.infer<typeof createStudentSchema>

export const updateStudentSchema = zUpdateStudentRequest.extend({
  admission_number: z
    .string()
    .min(1, 'Admission number is required')
    .optional(),
  name_english: z.string().min(1, 'English name is required').optional(),
  phone: z.string().min(1, 'Phone number is required').optional(),
  nic_or_birth_certificate: z
    .string()
    .min(1, 'NIC or Birth Certificate is required')
    .optional(),
  dob: z.string().min(1, 'Date of birth is required').optional(),
  address: z.string().min(1, 'Address is required').optional(),
  email: z.string().email('Invalid email address').optional().nullable(),
  photo_url: z.string().url('Invalid URL').optional().nullable(),
})

export type UpdateStudentValues = z.infer<typeof updateStudentSchema>
