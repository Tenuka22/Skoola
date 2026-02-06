import { useState } from 'react'
import { useForm } from 'react-hook-form'
import { zodResolver } from '@hookform/resolvers/zod'
import { useMutation } from '@tanstack/react-query'
import { toast } from 'sonner'
import type { CreateStudentRequest } from '@/lib/api/types.gen'
import { Button } from '@/components/ui/button'
import {
  Card,
  CardContent,
  CardDescription,
  CardFooter,
  CardHeader,
  CardTitle,
} from '@/components/ui/card'
import { Input } from '@/components/ui/input'
import { Label } from '@/components/ui/label'
import {
  Select,
  SelectContent,
  SelectItem,
  SelectTrigger,
  SelectValue,
} from '@/components/ui/select'
import { zCreateStudentRequest } from '@/lib/api/zod.gen'
import { postStudents9Cfb76Aa83C6A83D99Db1D6755C24Ee1Mutation } from '@/lib/api/@tanstack/react-query.gen'

const genders = ['Male', 'Female', 'Other'] as const
const ethnicities = ['Sinhala', 'Tamil', 'Muslim', 'Burger', 'Malay', 'Vedda', 'Other'] as const
const religions = ['Buddhism', 'Hinduism', 'Islam', 'Christianity', 'Other'] as const

interface NewStudentWizardProps {
  onStudentAdded?: () => void
}

export const NewStudentWizard = ({ onStudentAdded }: NewStudentWizardProps) => {
  const [currentStep, setCurrentStep] = useState(1)
  const totalSteps = 1 // Starting with 1 for now, expand later

  const form = useForm<any>({
    resolver: zodResolver(zCreateStudentRequest) as any,
    defaultValues: {
      name_english: '',
      admission_number: '',
      dob: new Date().toISOString().split('T')[0],
      gender: 'Male', // Default value
      nic_or_birth_certificate: '',
      phone: '',
      address: '',
      email: '',
      ethnicity: 'Sinhala', // Default value
      religion: 'Buddhism', // Default value
      status: 'Active', // Default value
    },
  })

  const createStudentMutation = useMutation({
    ...postStudents9Cfb76Aa83C6A83D99Db1D6755C24Ee1Mutation(),
    onSuccess: () => {
      toast.success('Student added successfully!')
      form.reset()
      setCurrentStep(1)
      onStudentAdded?.()
    },
    onError: (error) => {
      toast.error('Failed to add student.', { description: error.message })
    },
  })

  const onSubmit = (data: CreateStudentRequest) => {
    createStudentMutation.mutate({ body: data })
  }

  const handleNext = () => {
    // For now, directly submit if it's the only step.
    if (currentStep === totalSteps) {
      form.handleSubmit(onSubmit)()
    } else {
      setCurrentStep((prev) => Math.min(prev + 1, totalSteps))
    }
  }

  const handleBack = () => {
    setCurrentStep((prev) => Math.max(prev - 1, 1))
  }

  return (
    <Card className="w-full max-w-lg mx-auto">
      <CardHeader>
        <CardTitle>
          New Student Registration - Step {currentStep} of {totalSteps}
        </CardTitle>
        <CardDescription>Enter the student's details.</CardDescription>
      </CardHeader>
      <CardContent>
        {currentStep === 1 && (
          <div className="grid gap-4">
            <div className="grid grid-cols-4 items-center gap-4">
              <Label htmlFor="name_english">Full Name (English)</Label>
              <Input
                id="name_english"
                {...form.register('name_english')}
                className="col-span-3"
              />
              {form.formState.errors.name_english && (
                <p className="col-span-4 text-right text-red-500 text-sm">
                  {form.formState.errors.name_english?.message as string}
                </p>
              )}
            </div>
            <div className="grid grid-cols-4 items-center gap-4">
              <Label htmlFor="admission_number">Admission Number</Label>
              <Input
                id="admission_number"
                {...form.register('admission_number')}
                className="col-span-3"
              />
              {form.formState.errors.admission_number && (
                <p className="col-span-4 text-right text-red-500 text-sm">
                  {form.formState.errors.admission_number?.message as string}
                </p>
              )}
            </div>
            <div className="grid grid-cols-4 items-center gap-4">
              <Label htmlFor="dob">Date of Birth</Label>
              <Input
                id="dob"
                type="date"
                {...form.register('dob')}
                className="col-span-3"
              />
              {form.formState.errors.dob && (
                <p className="col-span-4 text-right text-red-500 text-sm">
                  {form.formState.errors.dob?.message as string}
                </p>
              )}
            </div>
            <div className="grid grid-cols-4 items-center gap-4">
              <Label htmlFor="gender">Gender</Label>
              <Select
                onValueChange={(value) =>
                  value && form.setValue('gender', value)
                }
                defaultValue={form.watch('gender')}
              >
                <SelectTrigger className="col-span-3" id="gender">
                  <SelectValue placeholder="Select Gender" />
                </SelectTrigger>
                <SelectContent>
                  {genders.map((gender) => (
                    <SelectItem key={gender} value={gender}>
                      {gender}
                    </SelectItem>
                  ))}
                </SelectContent>
              </Select>
              {form.formState.errors.gender && (
                <p className="col-span-4 text-right text-red-500 text-sm">
                  {form.formState.errors.gender?.message as string}
                </p>
              )}
            </div>
            <div className="grid grid-cols-4 items-center gap-4">
              <Label htmlFor="nic_or_birth_certificate">
                NIC/Birth Certificate
              </Label>
              <Input
                id="nic_or_birth_certificate"
                {...form.register('nic_or_birth_certificate')}
                className="col-span-3"
              />
              {form.formState.errors.nic_or_birth_certificate && (
                <p className="col-span-4 text-right text-red-500 text-sm">
                  {form.formState.errors.nic_or_birth_certificate?.message as string}
                </p>
              )}
            </div>
            <div className="grid grid-cols-4 items-center gap-4">
              <Label htmlFor="phone">Phone</Label>
              <Input
                id="phone"
                {...form.register('phone')}
                className="col-span-3"
              />
              {form.formState.errors.phone && (
                <p className="col-span-4 text-right text-red-500 text-sm">
                  {form.formState.errors.phone?.message as string}
                </p>
              )}
            </div>
            <div className="grid grid-cols-4 items-center gap-4">
              <Label htmlFor="address">Address</Label>
              <Input
                id="address"
                {...form.register('address')}
                className="col-span-3"
              />
              {form.formState.errors.address && (
                <p className="col-span-4 text-right text-red-500 text-sm">
                  {form.formState.errors.address?.message as string}
                </p>
              )}
            </div>
            <div className="grid grid-cols-4 items-center gap-4">
              <Label htmlFor="email">Email (Optional)</Label>
              <Input
                id="email"
                {...form.register('email')}
                className="col-span-3"
                type="email"
              />
              {form.formState.errors.email && (
                <p className="col-span-4 text-right text-red-500 text-sm">
                  {form.formState.errors.email?.message as string}
                </p>
              )}
            </div>
            <div className="grid grid-cols-4 items-center gap-4">
              <Label htmlFor="ethnicity">Ethnicity</Label>
              <Select
                onValueChange={(value) =>
                  value && form.setValue('ethnicity', value)
                }
                defaultValue={form.watch('ethnicity')}
              >
                <SelectTrigger className="col-span-3" id="ethnicity">
                  <SelectValue placeholder="Select Ethnicity" />
                </SelectTrigger>
                <SelectContent>
                  {ethnicities.map((ethnicity) => (
                    <SelectItem key={ethnicity} value={ethnicity}>
                      {ethnicity}
                    </SelectItem>
                  ))}
                </SelectContent>
              </Select>
              {form.formState.errors.ethnicity && (
                <p className="col-span-4 text-right text-red-500 text-sm">
                  {form.formState.errors.ethnicity?.message as string}
                </p>
              )}
            </div>
            <div className="grid grid-cols-4 items-center gap-4">
              <Label htmlFor="religion">Religion</Label>
              <Select
                onValueChange={(value) =>
                  value && form.setValue('religion', value)
                }
                defaultValue={form.watch('religion')}
              >
                <SelectTrigger className="col-span-3" id="religion">
                  <SelectValue placeholder="Select Religion" />
                </SelectTrigger>
                <SelectContent>
                  {religions.map((religion) => (
                    <SelectItem key={religion} value={religion}>
                      {religion}
                    </SelectItem>
                  ))}
                </SelectContent>
              </Select>
              {form.formState.errors.religion && (
                <p className="col-span-4 text-right text-red-500 text-sm">
                  {form.formState.errors.religion?.message as string}
                </p>
              )}
            </div>
          </div>
        )}
        {/* Future steps can be added here */}
      </CardContent>
      <CardFooter className="flex justify-between">
        <Button
          variant="outline"
          onClick={handleBack}
          disabled={currentStep === 1}
        >
          Back
        </Button>
        <Button onClick={handleNext} disabled={createStudentMutation.isPending}>
          {currentStep < totalSteps
            ? 'Next'
            : createStudentMutation.isPending
              ? 'Submitting...'
              : 'Submit'}
        </Button>
      </CardFooter>
    </Card>
  )
}
