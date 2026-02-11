import { z } from 'zod';

export const studentStatusSchema = z.enum([
  'Active',
  'Suspended',
  'Graduated',
  'Transferred',
  'Withdrawn',
]);
export type StudentStatus = z.infer<typeof studentStatusSchema>;

export const genderSchema = z.enum(['Male', 'Female', 'Other']);
export type Gender = z.infer<typeof genderSchema>;

export const ethnicitySchema = z.enum([
  'Sinhala',
  'Tamil',
  'Muslim',
  'Burger',
  'Malay',
  'Other',
]);
export type Ethnicity = z.infer<typeof ethnicitySchema>;

export const religionSchema = z.enum([
  'Buddhism',
  'Hinduism',
  'Islam',
  'Christianity',
  'Other',
]);
export type Religion = z.infer<typeof religionSchema>;

export const createStudentSchema = z.object({
  admission_number: z.string().min(1, 'Admission number is required'),
  name_english: z.string().min(1, 'English name is required'),
  name_sinhala: z.string().optional().nullable(),
  name_tamil: z.string().optional().nullable(),
  email: z.string().email('Invalid email address').optional().nullable(),
  phone: z.string().min(1, 'Phone number is required'),
  nic_or_birth_certificate: z.string().min(1, 'NIC or Birth Certificate is required'),
  dob: z.string().min(1, 'Date of birth is required'),
  gender: genderSchema,
  address: z.string().min(1, 'Address is required'),
  ethnicity: ethnicitySchema.optional().nullable(),
  religion: religionSchema.optional().nullable(),
  photo_url: z.string().url('Invalid URL').optional().nullable(),
  status: studentStatusSchema.optional().nullable(),
})

export type CreateStudentValues = z.infer<typeof createStudentSchema>

export const updateStudentSchema = createStudentSchema.partial()

export type UpdateStudentValues = z.infer<typeof updateStudentSchema>
