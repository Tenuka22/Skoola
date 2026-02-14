'use client'

import { z } from 'zod'
import { useForm } from 'react-hook-form'
import { zodResolver } from '@hookform/resolvers/zod'
import {
  createStaffSchema,
  employmentStatusSchema,
  staffTypeSchema,
} from '../schemas'
import type { CreateStaffValues } from '../schemas'
import { Button } from '@/components/ui/button'
import { Input } from '@/components/ui/input'
import { Label } from '@/components/ui/label'
import {
  Select,
  SelectContent,
  SelectItem,
  SelectTrigger,
  SelectValue,
} from '@/components/ui/select'
import { Spinner } from '@/components/ui/spinner'

interface StaffFormProps {
  initialValues?: Partial<CreateStaffValues>
  onSubmit: (values: CreateStaffValues) => void
  isSubmitting?: boolean
  submitLabel?: string
}

export function StaffForm({
  initialValues,
  onSubmit,
  isSubmitting,
  submitLabel = 'Save Staff Member',
}: StaffFormProps) {
  const {
    register,
    handleSubmit,
    setValue,
    watch,
    formState: { errors },
  } = useForm<CreateStaffValues>({
    resolver: zodResolver(createStaffSchema),
    defaultValues: {
      employee_id: '',
      name: '',
      email: '',
      phone: '',
      nic: '',
      dob: '',
      gender: 'Male',
      address: '',
      staff_type: 'Teaching',
      employment_status: 'Permanent',
      ...initialValues,
    },
  })

  const gender = watch('gender')
  const staffType = watch('staff_type')
  const employmentStatus = watch('employment_status')

  return (
    <form onSubmit={handleSubmit(onSubmit)} className="space-y-6">
      <div className="grid gap-4 md:grid-cols-2">
        <div className="space-y-2">
          <Label>Employee ID</Label>
          <Input {...register('employee_id')} placeholder="EMP001" />
          {errors.employee_id && (
            <p className="text-sm text-destructive">
              {errors.employee_id.message}
            </p>
          )}
        </div>

        <div className="space-y-2">
          <Label>Full Name</Label>
          <Input {...register('name')} placeholder="John Doe" />
          {errors.name && (
            <p className="text-sm text-destructive">{errors.name.message}</p>
          )}
        </div>

        <div className="space-y-2">
          <Label>Email</Label>
          <Input
            {...register('email')}
            type="email"
            placeholder="john@example.com"
          />
          {errors.email && (
            <p className="text-sm text-destructive">{errors.email.message}</p>
          )}
        </div>

        <div className="space-y-2">
          <Label>Phone</Label>
          <Input {...register('phone')} placeholder="+94 77 123 4567" />
          {errors.phone && (
            <p className="text-sm text-destructive">{errors.phone.message}</p>
          )}
        </div>

        <div className="space-y-2">
          <Label>NIC</Label>
          <Input {...register('nic')} placeholder="123456789V" />
          {errors.nic && (
            <p className="text-sm text-destructive">{errors.nic.message}</p>
          )}
        </div>

        <div className="space-y-2">
          <Label>Date of Birth</Label>
          <Input {...register('dob')} type="date" />
          {errors.dob && (
            <p className="text-sm text-destructive">{errors.dob.message}</p>
          )}
        </div>

        <div className="space-y-2">
          <Label>Gender</Label>
          <Select
            value={gender}
            onValueChange={(val) => {
              const g = z.string().parse(val)
              setValue('gender', g)
            }}
          >
            <SelectTrigger className="w-full">
              <SelectValue placeholder="Select gender" />
            </SelectTrigger>
            <SelectContent>
              <SelectItem value="Male">Male</SelectItem>
              <SelectItem value="Female">Female</SelectItem>
              <SelectItem value="Other">Other</SelectItem>
            </SelectContent>
          </Select>
          {errors.gender && (
            <p className="text-sm text-destructive">{errors.gender.message}</p>
          )}
        </div>

        <div className="space-y-2">
          <Label>Staff Type</Label>
          <Select
            value={staffType}
            onValueChange={(val) => {
              const type = staffTypeSchema.parse(val)
              setValue('staff_type', type)
            }}
          >
            <SelectTrigger className="w-full">
              <SelectValue placeholder="Select staff type" />
            </SelectTrigger>
            <SelectContent>
              <SelectItem value="Teaching">Teaching</SelectItem>
              <SelectItem value="NonTeaching">Non-Teaching</SelectItem>
              <SelectItem value="Administrative">Administrative</SelectItem>
            </SelectContent>
          </Select>
          {errors.staff_type && (
            <p className="text-sm text-destructive">
              {errors.staff_type.message}
            </p>
          )}
        </div>

        <div className="space-y-2 col-span-2">
          <Label>Employment Status</Label>
          <Select
            value={employmentStatus}
            onValueChange={(val) => {
              const status = employmentStatusSchema.parse(val)
              setValue('employment_status', status)
            }}
          >
            <SelectTrigger className="w-full">
              <SelectValue placeholder="Select status" />
            </SelectTrigger>
            <SelectContent>
              <SelectItem value="Permanent">Permanent</SelectItem>
              <SelectItem value="Contract">Contract</SelectItem>
              <SelectItem value="Temporary">Temporary</SelectItem>
            </SelectContent>
          </Select>
          {errors.employment_status && (
            <p className="text-sm text-destructive">
              {errors.employment_status.message}
            </p>
          )}
        </div>

        <div className="space-y-2 col-span-2">
          <Label>Address</Label>
          <Input
            {...register('address')}
            placeholder="123, Main Street, City"
          />
          {errors.address && (
            <p className="text-sm text-destructive">{errors.address.message}</p>
          )}
        </div>
      </div>

      <div className="flex justify-end gap-2">
        <Button type="submit" disabled={isSubmitting}>
          {isSubmitting && <Spinner className="mr-2 h-4 w-4" />}
          {submitLabel}
        </Button>
      </div>
    </form>
  )
}
