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
  id: z.string().min(1, 'ID is required'),
  admission_number: z.string().min(1, 'Admission number is required'),
  name_english: z.string().min(2, 'English name must be at least 2 characters'),
  phone: z.string().min(10, 'Phone number must be at least 10 characters'),
  nic_or_birth_certificate: z
    .string()
    .min(5, 'NIC or Birth Certificate must be at least 5 characters'),
  dob: z.string().min(1, 'Date of birth is required'),
  address: z.string().min(5, 'Address must be at least 5 characters'),
  email: z.string().email('Invalid email address').optional().nullable(),
  photo_url: z.string().url('Invalid URL').optional().nullable(),
})

export type CreateStudentValues = z.infer<typeof createStudentSchema>

export const updateStudentSchema = zUpdateStudentRequest.extend({
  id: z.string().min(1, 'ID is required'),
  admission_number: z
    .string()
    .min(1, 'Admission number is required')
    .optional(),
  name_english: z
    .string()
    .min(2, 'English name must be at least 2 characters')
    .optional(),
  phone: z
    .string()
    .min(10, 'Phone number must be at least 10 characters')
    .optional(),
  nic_or_birth_certificate: z
    .string()
    .min(5, 'NIC or Birth Certificate must be at least 5 characters')
    .optional(),
  dob: z.string().min(1, 'Date of birth is required').optional(),
  address: z.string().min(5, 'Address must be at least 5 characters').optional(),
  email: z.string().email('Invalid email address').optional().nullable(),
  photo_url: z.string().url('Invalid URL').optional().nullable(),
})

export type UpdateStudentValues = z.infer<typeof updateStudentSchema>
