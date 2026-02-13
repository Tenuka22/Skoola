'use client'

import { useForm } from 'react-hook-form'
import { zodResolver } from '@hookform/resolvers/zod'
import { HugeiconsIcon } from '@hugeicons/react'
import { Loading03Icon } from '@hugeicons/core-free-icons'
import {
  createStudentSchema,
  ethnicitySchema,
  genderSchema,
  religionSchema,
  studentStatusSchema,
} from '../schemas'
import type {
  CreateStudentValues,
  Ethnicity,
  Gender,
  Religion,
  StudentStatus,
} from '../schemas'
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
    <form onSubmit={handleSubmit(onSubmit)} className="space-y-8">
      <FieldGroup className="space-y-6">
        <div className="grid grid-cols-1 md:grid-cols-2 gap-6">
          <Field>
            <FieldLabel className="text-xs font-black uppercase tracking-widest opacity-50">
              Admission Number
            </FieldLabel>
            <Input
              {...register('admission_number')}
              placeholder="ADM001"
              className="h-12 rounded-2xl border-none bg-muted/30 px-4 font-bold focus-visible:ring-2 focus-visible:ring-primary"
            />
            <FieldError errors={[errors.admission_number]} />
          </Field>

          <Field>
            <FieldLabel className="text-xs font-black uppercase tracking-widest opacity-50">
              English Name
            </FieldLabel>
            <Input
              {...register('name_english')}
              placeholder="John Doe"
              className="h-12 rounded-2xl border-none bg-muted/30 px-4 font-bold focus-visible:ring-2 focus-visible:ring-primary"
            />
            <FieldError errors={[errors.name_english]} />
          </Field>

          <Field>
            <FieldLabel className="text-xs font-black uppercase tracking-widest opacity-50">
              Sinhala Name (Optional)
            </FieldLabel>
            <Input
              {...register('name_sinhala')}
              placeholder="සිංහල නම"
              className="h-12 rounded-2xl border-none bg-muted/30 px-4 font-bold focus-visible:ring-2 focus-visible:ring-primary"
            />
            <FieldError errors={[errors.name_sinhala]} />
          </Field>

          <Field>
            <FieldLabel className="text-xs font-black uppercase tracking-widest opacity-50">
              Tamil Name (Optional)
            </FieldLabel>
            <Input
              {...register('name_tamil')}
              placeholder="தமிழ் பெயர்"
              className="h-12 rounded-2xl border-none bg-muted/30 px-4 font-bold focus-visible:ring-2 focus-visible:ring-primary"
            />
            <FieldError errors={[errors.name_tamil]} />
          </Field>

          <Field>
            <FieldLabel className="text-xs font-black uppercase tracking-widest opacity-50">
              Email (Optional)
            </FieldLabel>
            <Input
              {...register('email')}
              placeholder="john@example.com"
              type="email"
              className="h-12 rounded-2xl border-none bg-muted/30 px-4 font-bold focus-visible:ring-2 focus-visible:ring-primary"
            />
            <FieldError errors={[errors.email]} />
          </Field>

          <Field>
            <FieldLabel className="text-xs font-black uppercase tracking-widest opacity-50">
              Phone Number
            </FieldLabel>
            <Input
              {...register('phone')}
              placeholder="+94 77 123 4567"
              className="h-12 rounded-2xl border-none bg-muted/30 px-4 font-bold focus-visible:ring-2 focus-visible:ring-primary"
            />
            <FieldError errors={[errors.phone]} />
          </Field>

          <Field>
            <FieldLabel className="text-xs font-black uppercase tracking-widest opacity-50">
              NIC / Birth Certificate No.
            </FieldLabel>
            <Input
              {...register('nic_or_birth_certificate')}
              placeholder="123456789V"
              className="h-12 rounded-2xl border-none bg-muted/30 px-4 font-bold focus-visible:ring-2 focus-visible:ring-primary"
            />
            <FieldError errors={[errors.nic_or_birth_certificate]} />
          </Field>

          <Field>
            <FieldLabel className="text-xs font-black uppercase tracking-widest opacity-50">
              Date of Birth
            </FieldLabel>
            <Input
              {...register('dob')}
              type="date"
              className="h-12 rounded-2xl border-none bg-muted/30 px-4 font-bold focus-visible:ring-2 focus-visible:ring-primary"
            />
            <FieldError errors={[errors.dob]} />
          </Field>

          <Field>
            <FieldLabel className="text-xs font-black uppercase tracking-widest opacity-50">
              Gender
            </FieldLabel>
            <Select
              onValueChange={(val) => setValue('gender', val as Gender)}
              defaultValue={gender}
            >
              <SelectTrigger className="h-12 rounded-2xl border-none bg-muted/30 px-4 font-bold focus:ring-2 focus:ring-primary">
                <SelectValue placeholder="Select gender" />
              </SelectTrigger>
              <SelectContent className="rounded-xl">
                {genderSchema.options.map((g) => (
                  <SelectItem key={g} value={g}>
                    {g}
                  </SelectItem>
                ))}
              </SelectContent>
            </Select>
            <FieldError errors={[errors.gender]} />
          </Field>

          <Field>
            <FieldLabel className="text-xs font-black uppercase tracking-widest opacity-50">
              Ethnicity (Optional)
            </FieldLabel>
            <Select
              onValueChange={(val) => setValue('ethnicity', val as Ethnicity)}
              defaultValue={ethnicity || ''}
            >
              <SelectTrigger className="h-12 rounded-2xl border-none bg-muted/30 px-4 font-bold focus:ring-2 focus:ring-primary">
                <SelectValue placeholder="Select ethnicity" />
              </SelectTrigger>
              <SelectContent className="rounded-xl">
                {ethnicitySchema.options.map((e) => (
                  <SelectItem key={e} value={e}>
                    {e}
                  </SelectItem>
                ))}
              </SelectContent>
            </Select>
            <FieldError errors={[errors.ethnicity]} />
          </Field>

          <Field>
            <FieldLabel className="text-xs font-black uppercase tracking-widest opacity-50">
              Religion (Optional)
            </FieldLabel>
            <Select
              onValueChange={(val) => setValue('religion', val as Religion)}
              defaultValue={religion || ''}
            >
              <SelectTrigger className="h-12 rounded-2xl border-none bg-muted/30 px-4 font-bold focus:ring-2 focus:ring-primary">
                <SelectValue placeholder="Select religion" />
              </SelectTrigger>
              <SelectContent className="rounded-xl">
                {religionSchema.options.map((r) => (
                  <SelectItem key={r} value={r}>
                    {r}
                  </SelectItem>
                ))}
              </SelectContent>
            </Select>
            <FieldError errors={[errors.religion]} />
          </Field>

          <Field>
            <FieldLabel className="text-xs font-black uppercase tracking-widest opacity-50">
              Status
            </FieldLabel>
            <Select
              onValueChange={(val) => setValue('status', val as StudentStatus)}
              defaultValue={status || 'Active'}
            >
              <SelectTrigger className="h-12 rounded-2xl border-none bg-muted/30 px-4 font-bold focus:ring-2 focus:ring-primary">
                <SelectValue placeholder="Select status" />
              </SelectTrigger>
              <SelectContent className="rounded-xl">
                {studentStatusSchema.options.map((s) => (
                  <SelectItem key={s} value={s}>
                    {s}
                  </SelectItem>
                ))}
              </SelectContent>
            </Select>
            <FieldError errors={[errors.status]} />
          </Field>
        </div>

        <Field>
          <FieldLabel className="text-xs font-black uppercase tracking-widest opacity-50">
            Address
          </FieldLabel>
          <Input
            {...register('address')}
            placeholder="123, Main Street, City"
            className="h-12 rounded-2xl border-none bg-muted/30 px-4 font-bold focus-visible:ring-2 focus-visible:ring-primary"
          />
          <FieldError errors={[errors.address]} />
        </Field>
      </FieldGroup>

      <Button
        type="submit"
        className="w-full h-14 rounded-[1.25rem] font-black uppercase tracking-widest shadow-xl shadow-primary/20 transition-all active:scale-[0.98]"
        disabled={isSubmitting}
      >
        {isSubmitting ? (
          <HugeiconsIcon
            icon={Loading03Icon}
            className="mr-2 h-5 w-5 animate-spin"
          />
        ) : null}
        {submitLabel}
      </Button>
    </form>
  )
}
