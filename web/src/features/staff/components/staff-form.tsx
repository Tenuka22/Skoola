'use client'

import { staffFormSchema } from '../schemas'
import type { StaffFormValues } from '../schemas'
import { Button } from '@/components/ui/button'
import { Spinner } from '@/components/ui/spinner'
import { zEmploymentStatus, zGender, zStaffType } from '@/lib/api/zod.gen'
import { FormBuilder, defineFormConfig } from '@/components/form-builder'

interface StaffFormProps {
  initialValues?: Partial<StaffFormValues>
  onSubmit: (values: StaffFormValues) => void
  isSubmitting?: boolean
  submitLabel?: string
}

export function StaffForm({
  initialValues,
  onSubmit,
  isSubmitting,
  submitLabel = 'Save Staff Member',
}: StaffFormProps) {
  const config = defineFormConfig(staffFormSchema, {
    structure: [
      [
        {
          field: 'employee_id',
          type: 'input',
          label: 'Employee ID',
          placeholder: 'EMP001',
        },
        {
          field: 'name',
          type: 'input',
          label: 'Full Name',
          placeholder: 'John Doe',
        },
      ],
      [
        {
          field: 'email',
          type: 'input',
          label: 'Email',
          inputType: 'email',
          placeholder: 'john@example.com',
        },
        {
          field: 'phone',
          type: 'input',
          label: 'Phone',
          placeholder: '+94 77 123 4567',
        },
      ],
      [
        {
          field: 'nic',
          type: 'input',
          label: 'NIC',
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
          items: zGender.options.map((value) => ({
            label: value,
            value,
          })),
          parse: (value) => zGender.parse(value),
        },
        {
          field: 'staff_type',
          type: 'select',
          label: 'Staff Type',
          placeholder: 'Select staff type',
          items: zStaffType.options.map((value) => ({
            label: value,
            value,
          })),
          parse: (value) => zStaffType.parse(value),
        },
      ],
      [
        {
          field: 'employment_status',
          type: 'select',
          label: 'Employment Status',
          placeholder: 'Select status',
          items: zEmploymentStatus.options.map((value) => ({
            label: value,
            value,
          })),
          parse: (value) => zEmploymentStatus.parse(value),
          className: 'sm:col-span-2',
        },
        {
          field: 'address',
          type: 'input',
          label: 'Address',
          placeholder: '123, Main Street, City',
          className: 'sm:col-span-2',
        },
      ],
    ],
    extras: {
      bottom: () => (
        <div className="flex justify-end gap-2">
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
      schema={staffFormSchema}
      config={config}
      defaultValues={{
        employee_id: '',
        name: '',
        email: '',
        phone: '',
        nic: '',
        dob: '',
        gender: zGender.enum.Male,
        address: '',
        staff_type: zStaffType.enum.Teaching,
        employment_status: zEmploymentStatus.enum.Permanent,
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
