import { Controller } from 'react-hook-form'
import { useQuery } from '@tanstack/react-query'
import type {
  ControllerFieldState,
  ControllerRenderProps,
  Path,
} from 'react-hook-form'
import type {
  AcademicYearResponse,
  ClassResponse,
  CreateClassSubjectTeacherRequest,
  StaffResponse,
  SubjectResponse,
} from '@/lib/api/types.gen'
import { Button } from '@/components/ui/button'
import {
  Dialog,
  DialogContent,
  DialogFooter,
  DialogHeader,
  DialogTitle,
} from '@/components/ui/dialog'
import { Field, FieldError, FieldLabel } from '@/components/ui/field'
import {
  Select,
  SelectContent,
  SelectItem,
  SelectTrigger,
  SelectValue,
} from '@/components/ui/select'
import { Spinner } from '@/components/ui/spinner'
import { authClient } from '@/lib/clients'
import {
  getAllStaffOptions,
  getAllSubjectsOptions,
} from '@/lib/api/@tanstack/react-query.gen'
import { zCreateClassSubjectTeacherRequest } from '@/lib/api/zod.gen'
import { FormBuilder, defineFormConfig } from '@/components/form-builder'

interface AssignTeacherDialogProps {
  open: boolean
  onOpenChange: (open: boolean) => void
  onConfirm: (data: CreateClassSubjectTeacherRequest) => void
  isSubmitting: boolean
  academicYears: Array<AcademicYearResponse>
  classes: Array<ClassResponse>
}

export function AssignTeacherDialog({
  open,
  onOpenChange,
  onConfirm,
  isSubmitting,
  academicYears,
  classes,
}: AssignTeacherDialogProps) {
  const { data: staffData } = useQuery({
    ...getAllStaffOptions({
      client: authClient,
      query: { limit: 1000, page: 1 },
    }),
  })
  const teachers = staffData?.data || []

  const { data: subjectsData } = useQuery({
    ...getAllSubjectsOptions({
      client: authClient,
      query: { limit: 1000, page: 1 },
    }),
  })
  const subjects = subjectsData?.data || []

  const config = defineFormConfig(zCreateClassSubjectTeacherRequest, {
    structure: [],
    extras: {
      top: (form) => (
        <>
          <Controller
            name="class_id"
            control={form.control}
            render={({
              field,
              fieldState,
            }: {
              field: ControllerRenderProps<
                CreateClassSubjectTeacherRequest,
                Path<CreateClassSubjectTeacherRequest>
              >
              fieldState: ControllerFieldState
            }) => (
              <Field data-invalid={fieldState.invalid}>
                <FieldLabel htmlFor="class_id">Class</FieldLabel>
                <Select
                  {...field}
                  value={typeof field.value === 'string' ? field.value : ''}
                  onValueChange={(value) => {
                    if (value === null) return
                    field.onChange(value)
                  }}
                >
                  <SelectTrigger
                    id="class_id"
                    aria-invalid={fieldState.invalid}
                  >
                    <SelectValue placeholder="Select a class" />
                  </SelectTrigger>
                  <SelectContent>
                    {classes.map((cls) => (
                      <SelectItem key={cls.id} value={cls.id}>
                        {cls.section_name}
                      </SelectItem>
                    ))}
                  </SelectContent>
                </Select>
                {fieldState.invalid && (
                  <FieldError errors={[fieldState.error]} />
                )}
              </Field>
            )}
          />

          <Controller
            name="subject_id"
            control={form.control}
            render={({
              field,
              fieldState,
            }: {
              field: ControllerRenderProps<
                CreateClassSubjectTeacherRequest,
                Path<CreateClassSubjectTeacherRequest>
              >
              fieldState: ControllerFieldState
            }) => (
              <Field data-invalid={fieldState.invalid}>
                <FieldLabel htmlFor="subject_id">Subject</FieldLabel>
                <Select
                  {...field}
                  value={typeof field.value === 'string' ? field.value : ''}
                  onValueChange={(value) => {
                    if (value === null) return
                    field.onChange(value)
                  }}
                >
                  <SelectTrigger
                    id="subject_id"
                    aria-invalid={fieldState.invalid}
                  >
                    <SelectValue placeholder="Select a subject" />
                  </SelectTrigger>
                  <SelectContent>
                    {subjects.map((subject: SubjectResponse) => (
                      <SelectItem key={subject.id} value={subject.id}>
                        {subject.subject_name_en}
                      </SelectItem>
                    ))}
                  </SelectContent>
                </Select>
                {fieldState.invalid && (
                  <FieldError errors={[fieldState.error]} />
                )}
              </Field>
            )}
          />

          <Controller
            name="teacher_id"
            control={form.control}
            render={({
              field,
              fieldState,
            }: {
              field: ControllerRenderProps<
                CreateClassSubjectTeacherRequest,
                Path<CreateClassSubjectTeacherRequest>
              >
              fieldState: ControllerFieldState
            }) => (
              <Field data-invalid={fieldState.invalid}>
                <FieldLabel htmlFor="teacher_id">Teacher</FieldLabel>
                <Select
                  {...field}
                  value={typeof field.value === 'string' ? field.value : ''}
                  onValueChange={(value) => {
                    if (value === null) return
                    field.onChange(value)
                  }}
                >
                  <SelectTrigger
                    id="teacher_id"
                    aria-invalid={fieldState.invalid}
                  >
                    <SelectValue placeholder="Select a teacher" />
                  </SelectTrigger>
                  <SelectContent>
                    {teachers.map((teacher: StaffResponse) => (
                      <SelectItem key={teacher.id} value={teacher.id}>
                        {teacher.name}
                      </SelectItem>
                    ))}
                  </SelectContent>
                </Select>
                {fieldState.invalid && (
                  <FieldError errors={[fieldState.error]} />
                )}
              </Field>
            )}
          />

          <Controller
            name="academic_year_id"
            control={form.control}
            render={({
              field,
              fieldState,
            }: {
              field: ControllerRenderProps<
                CreateClassSubjectTeacherRequest,
                Path<CreateClassSubjectTeacherRequest>
              >
              fieldState: ControllerFieldState
            }) => (
              <Field data-invalid={fieldState.invalid}>
                <FieldLabel htmlFor="academic_year_id">
                  Academic Year
                </FieldLabel>
                <Select
                  {...field}
                  value={typeof field.value === 'string' ? field.value : ''}
                  onValueChange={(value) => {
                    if (value === null) return
                    field.onChange(value)
                  }}
                >
                  <SelectTrigger
                    id="academic_year_id"
                    aria-invalid={fieldState.invalid}
                  >
                    <SelectValue placeholder="Select an academic year" />
                  </SelectTrigger>
                  <SelectContent>
                    {academicYears.map((year) => (
                      <SelectItem key={year.id} value={year.id}>
                        {year.name}
                      </SelectItem>
                    ))}
                  </SelectContent>
                </Select>
                {fieldState.invalid && (
                  <FieldError errors={[fieldState.error]} />
                )}
              </Field>
            )}
          />
        </>
      ),
      bottom: (
        <DialogFooter className="pt-4">
          <Button type="submit" disabled={isSubmitting}>
            {isSubmitting ? <Spinner className="mr-2" /> : null}
            Assign Teacher
          </Button>
        </DialogFooter>
      ),
    },
  })

  return (
    <Dialog open={open} onOpenChange={onOpenChange}>
      <DialogContent className="max-w-md">
        <DialogHeader>
          <DialogTitle>Assign Teacher to Class</DialogTitle>
        </DialogHeader>
        <FormBuilder
          schema={zCreateClassSubjectTeacherRequest}
          config={config}
          defaultValues={{
            class_id: '',
            subject_id: '',
            teacher_id: '',
            academic_year_id: '',
          }}
          onSubmit={(values) => onConfirm(values)}
          isLoading={isSubmitting}
          showErrorSummary={false}
          toastErrors={false}
          showSuccessAlert={false}
          actions={[]}
          className="space-y-4"
        />
      </DialogContent>
    </Dialog>
  )
}
