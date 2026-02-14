'use client'

import { useForm } from 'react-hook-form'
import { zodResolver } from '@hookform/resolvers/zod'
import {
  createStudentSchema,
  ethnicitySchema,
  genderSchema,
  religionSchema,
  studentStatusSchema,
} from '../schemas'
import type { CreateStudentValues } from '../schemas'
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
  const {
    register,
    handleSubmit,
    setValue,
    watch,
    formState: { errors },
  } = useForm<CreateStudentValues>({
    resolver: zodResolver(createStudentSchema),
    defaultValues: {
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
    },
  })

  const gender = watch('gender')
  const ethnicity = watch('ethnicity')
  const religion = watch('religion')
  const status = watch('status')

  return (
    <form onSubmit={handleSubmit(onSubmit)} className="space-y-6">
      <div className="grid gap-4 md:grid-cols-2">
        <div className="space-y-2">
          <Label>Admission Number</Label>
          <Input {...register('admission_number')} placeholder="ADM001" />
          {errors.admission_number && (
            <p className="text-sm text-destructive">
              {errors.admission_number.message}
            </p>
          )}
        </div>

        <div className="space-y-2">
          <Label>English Name</Label>
          <Input {...register('name_english')} placeholder="John Doe" />
          {errors.name_english && (
            <p className="text-sm text-destructive">
              {errors.name_english.message}
            </p>
          )}
        </div>

        <div className="space-y-2">
          <Label>Sinhala Name (Optional)</Label>
          <Input {...register('name_sinhala')} placeholder="සිංහල නම" />
          {errors.name_sinhala && (
            <p className="text-sm text-destructive">
              {errors.name_sinhala.message}
            </p>
          )}
        </div>

        <div className="space-y-2">
          <Label>Tamil Name (Optional)</Label>
          <Input {...register('name_tamil')} placeholder="தமிழ் பெயர்" />
          {errors.name_tamil && (
            <p className="text-sm text-destructive">
              {errors.name_tamil.message}
            </p>
          )}
        </div>

        <div className="space-y-2">
          <Label>Email (Optional)</Label>
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
          <Label>Phone Number</Label>
          <Input {...register('phone')} placeholder="+94 77 123 4567" />
          {errors.phone && (
            <p className="text-sm text-destructive">{errors.phone.message}</p>
          )}
        </div>

        <div className="space-y-2">
          <Label>NIC / Birth Certificate No.</Label>
          <Input
            {...register('nic_or_birth_certificate')}
            placeholder="123456789V"
          />
          {errors.nic_or_birth_certificate && (
            <p className="text-sm text-destructive">
              {errors.nic_or_birth_certificate.message}
            </p>
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
              const g = genderSchema.parse(val)
              setValue('gender', g)
            }}
          >
            <SelectTrigger className="w-full">
              <SelectValue placeholder="Select gender" />
            </SelectTrigger>
            <SelectContent>
              {genderSchema.options.map((g) => (
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
          <Label>Ethnicity (Optional)</Label>
          <Select
            value={ethnicity ?? ''}
            onValueChange={(val) => {
              const e = ethnicitySchema.parse(val)
              setValue('ethnicity', e)
            }}
          >
            <SelectTrigger className="w-full">
              <SelectValue placeholder="Select ethnicity" />
            </SelectTrigger>
            <SelectContent>
              {ethnicitySchema.options.map((e) => (
                <SelectItem key={e} value={e}>
                  {e}
                </SelectItem>
              ))}
            </SelectContent>
          </Select>
          {errors.ethnicity && (
            <p className="text-sm text-destructive">
              {errors.ethnicity.message}
            </p>
          )}
        </div>

        <div className="space-y-2">
          <Label>Religion (Optional)</Label>
          <Select
            value={religion ?? ''}
            onValueChange={(val) => {
              const r = religionSchema.parse(val)
              setValue('religion', r)
            }}
          >
            <SelectTrigger className="w-full">
              <SelectValue placeholder="Select religion" />
            </SelectTrigger>
            <SelectContent>
              {religionSchema.options.map((r) => (
                <SelectItem key={r} value={r}>
                  {r}
                </SelectItem>
              ))}
            </SelectContent>
          </Select>
          {errors.religion && (
            <p className="text-sm text-destructive">
              {errors.religion.message}
            </p>
          )}
        </div>

        <div className="space-y-2">
          <Label>Status</Label>
          <Select
            value={status}
            onValueChange={(val) => {
              const s = studentStatusSchema.parse(val)
              setValue('status', s)
            }}
          >
            <SelectTrigger className="w-full">
              <SelectValue placeholder="Select status" />
            </SelectTrigger>
            <SelectContent>
              {studentStatusSchema.options.map((s) => (
                <SelectItem key={s} value={s}>
                  {s}
                </SelectItem>
              ))}
            </SelectContent>
          </Select>
          {errors.status && (
            <p className="text-sm text-destructive">{errors.status.message}</p>
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

      <div className="flex justify-end">
        <Button type="submit" disabled={isSubmitting}>
          {isSubmitting && <Spinner className="mr-2 h-4 w-4" />}
          {submitLabel}
        </Button>
      </div>
    </form>
  )
}
