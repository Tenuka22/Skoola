'use client'

import { useForm } from 'react-hook-form'
import { zodResolver } from '@hookform/resolvers/zod'
import { HugeiconsIcon } from '@hugeicons/react'
import { Loading03Icon } from '@hugeicons/core-free-icons'
import {  createStaffSchema } from '../schemas'
import type {CreateStaffValues} from '../schemas';
import {
  Field,
  FieldError,
  FieldGroup,
  FieldLabel,
} from '@/components/ui/field'
import { Input } from '@/components/ui/input'
import {
  Select,
  SelectContent,
  SelectItem,
  SelectTrigger,
  SelectValue,
} from '@/components/ui/select'
import { Button } from '@/components/ui/button'

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

  const staffType = watch('staff_type')
  const employmentStatus = watch('employment_status')
  const gender = watch('gender')

  return (
    <form onSubmit={handleSubmit(onSubmit)} className="space-y-8">
      <FieldGroup className="space-y-6">
        <div className="grid grid-cols-1 md:grid-cols-2 gap-6">
          <Field>
            <FieldLabel className="text-xs font-black uppercase tracking-widest opacity-50">Employee ID</FieldLabel>
            <Input 
              {...register('employee_id')} 
              placeholder="EMP001" 
              className="h-12 rounded-2xl border-none bg-muted/30 px-4 font-bold focus-visible:ring-2 focus-visible:ring-primary"
            />
            <FieldError errors={[errors.employee_id]} />
          </Field>

          <Field>
            <FieldLabel className="text-xs font-black uppercase tracking-widest opacity-50">Full Name</FieldLabel>
            <Input 
              {...register('name')} 
              placeholder="John Doe" 
              className="h-12 rounded-2xl border-none bg-muted/30 px-4 font-bold focus-visible:ring-2 focus-visible:ring-primary"
            />
            <FieldError errors={[errors.name]} />
          </Field>

          <Field>
            <FieldLabel className="text-xs font-black uppercase tracking-widest opacity-50">Email</FieldLabel>
            <Input 
              {...register('email')} 
              placeholder="john@example.com" 
              type="email" 
              className="h-12 rounded-2xl border-none bg-muted/30 px-4 font-bold focus-visible:ring-2 focus-visible:ring-primary"
            />
            <FieldError errors={[errors.email]} />
          </Field>

          <Field>
            <FieldLabel className="text-xs font-black uppercase tracking-widest opacity-50">Phone Number</FieldLabel>
            <Input 
              {...register('phone')} 
              placeholder="+94 77 123 4567" 
              className="h-12 rounded-2xl border-none bg-muted/30 px-4 font-bold focus-visible:ring-2 focus-visible:ring-primary"
            />
            <FieldError errors={[errors.phone]} />
          </Field>

          <Field>
            <FieldLabel className="text-xs font-black uppercase tracking-widest opacity-50">NIC</FieldLabel>
            <Input 
              {...register('nic')} 
              placeholder="123456789V" 
              className="h-12 rounded-2xl border-none bg-muted/30 px-4 font-bold focus-visible:ring-2 focus-visible:ring-primary"
            />
            <FieldError errors={[errors.nic]} />
          </Field>

          <Field>
            <FieldLabel className="text-xs font-black uppercase tracking-widest opacity-50">Date of Birth</FieldLabel>
            <Input 
              {...register('dob')} 
              type="date" 
              className="h-12 rounded-2xl border-none bg-muted/30 px-4 font-bold focus-visible:ring-2 focus-visible:ring-primary"
            />
            <FieldError errors={[errors.dob]} />
          </Field>

          <Field>
            <FieldLabel className="text-xs font-black uppercase tracking-widest opacity-50">Gender</FieldLabel>
            <Select onValueChange={(val) => setValue('gender', val as any)} defaultValue={gender}>
              <SelectTrigger className="h-12 rounded-2xl border-none bg-muted/30 px-4 font-bold focus:ring-2 focus:ring-primary">
                <SelectValue placeholder="Select gender" />
              </SelectTrigger>
              <SelectContent className="rounded-xl">
                <SelectItem value="Male">Male</SelectItem>
                <SelectItem value="Female">Female</SelectItem>
                <SelectItem value="Other">Other</SelectItem>
              </SelectContent>
            </Select>
            <FieldError errors={[errors.gender]} />
          </Field>

          <Field>
            <FieldLabel className="text-xs font-black uppercase tracking-widest opacity-50">Staff Type</FieldLabel>
            <Select onValueChange={(val: any) => setValue('staff_type', val)} defaultValue={staffType}>
              <SelectTrigger className="h-12 rounded-2xl border-none bg-muted/30 px-4 font-bold focus:ring-2 focus:ring-primary">
                <SelectValue placeholder="Select type" />
              </SelectTrigger>
              <SelectContent className="rounded-xl">
                <SelectItem value="Teaching">Teaching</SelectItem>
                <SelectItem value="NonTeaching">Non-Teaching</SelectItem>
                <SelectItem value="Administrative">Administrative</SelectItem>
              </SelectContent>
            </Select>
            <FieldError errors={[errors.staff_type]} />
          </Field>

          <Field>
            <FieldLabel className="text-xs font-black uppercase tracking-widest opacity-50">Employment Status</FieldLabel>
            <Select onValueChange={(val: any) => setValue('employment_status', val)} defaultValue={employmentStatus}>
              <SelectTrigger className="h-12 rounded-2xl border-none bg-muted/30 px-4 font-bold focus:ring-2 focus:ring-primary">
                <SelectValue placeholder="Select status" />
              </SelectTrigger>
              <SelectContent className="rounded-xl">
                <SelectItem value="Permanent">Permanent</SelectItem>
                <SelectItem value="Contract">Contract</SelectItem>
                <SelectItem value="Temporary">Temporary</SelectItem>
              </SelectContent>
            </Select>
            <FieldError errors={[errors.employment_status]} />
          </Field>
        </div>

        <Field>
          <FieldLabel className="text-xs font-black uppercase tracking-widest opacity-50">Address</FieldLabel>
          <Input 
            {...register('address')} 
            placeholder="123, Main Street, City" 
            className="h-12 rounded-2xl border-none bg-muted/30 px-4 font-bold focus-visible:ring-2 focus-visible:ring-primary"
          />
          <FieldError errors={[errors.address]} />
        </Field>
      </FieldGroup>

      <Button type="submit" className="w-full h-14 rounded-[1.25rem] font-black uppercase tracking-widest shadow-xl shadow-primary/20 transition-all active:scale-[0.98]" disabled={isSubmitting}>
        {isSubmitting ? (
          <HugeiconsIcon icon={Loading03Icon} className="mr-2 h-5 w-5 animate-spin" />
        ) : null}
        {submitLabel}
      </Button>
    </form>
  )
}
