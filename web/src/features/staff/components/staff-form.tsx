'use client'

import { useForm } from 'react-hook-form'
import { zodResolver } from '@hookform/resolvers/zod'
import { staffFormSchema } from '../schemas'
import type { StaffFormValues } from '../schemas'
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
import { zEmploymentStatus, zGender, zStaffType } from '@/lib/api/zod.gen'

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
  const {
    register,
    handleSubmit,
    setValue,
    watch,
    formState: { errors },
  } = useForm<StaffFormValues>({
    resolver: zodResolver(staffFormSchema),
    defaultValues: {
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
              setValue('gender', zGender.parse(val))
            }}
          >
            <SelectTrigger className="w-full">
              <SelectValue placeholder="Select gender" />
            </SelectTrigger>
            <SelectContent>
              {zGender.options.map((g) => (
                <SelectItem key={g} value={g}>
                  {g}
                </SelectItem>
              ))}
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
              setValue('staff_type', zStaffType.parse(val))
            }}
          >
            <SelectTrigger className="w-full">
              <SelectValue placeholder="Select staff type" />
            </SelectTrigger>
            <SelectContent>
              {zStaffType.options.map((type) => (
                <SelectItem key={type} value={type}>
                  {type}
                </SelectItem>
              ))}
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
              setValue('employment_status', zEmploymentStatus.parse(val))
            }}
          >
            <SelectTrigger className="w-full">
              <SelectValue placeholder="Select status" />
            </SelectTrigger>
            <SelectContent>
              {zEmploymentStatus.options.map((status) => (
                <SelectItem key={status} value={status}>
                  {status}
                </SelectItem>
              ))}
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
