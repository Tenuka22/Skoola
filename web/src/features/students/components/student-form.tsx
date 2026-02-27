'use client'

import {
  createStudentSchema,
  ethnicitySchema,
  genderSchema,
  religionSchema,
  studentStatusSchema,
} from '../schemas'
import type { CreateStudentValues } from '../schemas'
import { Button } from '@/components/ui/button'
import { Spinner } from '@/components/ui/spinner'
import { FormBuilder, defineFormConfig } from '@/components/form-builder'

interface StudentFormProps {
  initialValues?: Partial<CreateStudentValues>
  onSubmit: (values: CreateStudentValues) => void
  isSubmitting?: boolean
  submitLabel?: string
}

export function StudentForm({
  initialValues,
  onSubmit,
  isSubmitting,
  submitLabel = 'Save Student',
}: StudentFormProps) {
  const config = defineFormConfig(createStudentSchema, {
    structure: [
      [
        {
          field: 'admission_number',
          type: 'input',
          label: 'Admission Number',
          placeholder: 'ADM001',
        },
        {
          field: 'name_english',
          type: 'input',
          label: 'English Name',
          placeholder: 'John Doe',
        },
      ],
      [
        {
          field: 'name_sinhala',
          type: 'input',
          label: 'Sinhala Name (Optional)',
          placeholder: 'සිංහල නම',
        },
        {
          field: 'name_tamil',
          type: 'input',
          label: 'Tamil Name (Optional)',
          placeholder: 'தமிழ் பெயர்',
        },
      ],
      [
        {
          field: 'email',
          type: 'input',
          label: 'Email (Optional)',
          inputType: 'email',
          placeholder: 'john@example.com',
        },
        {
          field: 'phone',
          type: 'input',
          label: 'Phone Number',
          placeholder: '+94 77 123 4567',
        },
      ],
      [
        {
          field: 'nic_or_birth_certificate',
          type: 'input',
          label: 'NIC / Birth Certificate No.',
          placeholder: '123456789V',
        },
        {
          field: 'dob',
          type: 'input',
          label: 'Date of Birth',
          inputType: 'date',
        },
      ],
      [
        {
          field: 'gender',
          type: 'select',
          label: 'Gender',
          placeholder: 'Select gender',
          items: genderSchema.options.map((value) => ({
            label: value,
            value,
          })),
          parse: (value) => genderSchema.parse(value),
        },
        {
          field: 'ethnicity',
          type: 'select',
          label: 'Ethnicity (Optional)',
          placeholder: 'Select ethnicity',
          items: ethnicitySchema.options.map((value) => ({
            label: value,
            value,
          })),
          parse: (value) => ethnicitySchema.parse(value),
        },
      ],
      [
        {
          field: 'religion',
          type: 'select',
          label: 'Religion (Optional)',
          placeholder: 'Select religion',
          items: religionSchema.options.map((value) => ({
            label: value,
            value,
          })),
          parse: (value) => religionSchema.parse(value),
        },
        {
          field: 'status',
          type: 'select',
          label: 'Status',
          placeholder: 'Select status',
          items: studentStatusSchema.options.map((value) => ({
            label: value,
            value,
          })),
          parse: (value) => studentStatusSchema.parse(value),
        },
      ],
      [
        {
          field: 'address',
          type: 'input',
          label: 'Address',
          placeholder: '123, Main Street, City',
        },
      ],
    ],
    extras: {
      bottom: () => (
        <div className="flex justify-end">
          <Button type="submit" disabled={isSubmitting}>
            {isSubmitting && <Spinner className="mr-2 h-4 w-4" />}
            {submitLabel}
          </Button>
        </div>
      ),
    },
  })

  return (
    <FormBuilder
      schema={createStudentSchema}
      config={config}
      defaultValues={{
        admission_number: '',
        name_english: '',
        name_sinhala: undefined,
        name_tamil: undefined,
        email: undefined,
        phone: '',
        nic_or_birth_certificate: '',
        dob: '',
        gender: 'Male',
        address: '',
        ethnicity: undefined,
        religion: undefined,
        photo_url: undefined,
        status: 'Active',
        ...initialValues,
      }}
      onSubmit={(values) => onSubmit(values)}
      isLoading={isSubmitting}
      showErrorSummary={false}
      toastErrors={false}
      showSuccessAlert={false}
      actions={[]}
      className="space-y-6"
    />
  )
}
