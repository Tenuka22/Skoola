import type * as React from 'react'
import type {
  DefaultValues,
  FieldPath,
  FieldValues,
  UseFormReturn,
} from 'react-hook-form'
import type { ZodObject, input } from 'zod'
import type { Button } from '@/components/ui/button'

export type Slot<TInput extends FieldValues> =
  | React.ReactNode
  | ((form: UseFormReturn<TInput, unknown, TInput>) => React.ReactNode)

export type BaseField<TInput extends FieldValues> = {
  field: FieldPath<TInput>
  label?: string
  description?: string
  disabled?: boolean
  className?: string
  labelClassName?: string
}

export type InputField<TInput extends FieldValues> = BaseField<TInput> & {
  type: 'input'
  inputType?: React.HTMLInputTypeAttribute
  placeholder?: string
  inputClassName?: string
  inputGroup?: {
    className?: string
    addons?: Array<{
      content: React.ReactNode
      align?: 'inline-start' | 'inline-end' | 'block-start' | 'block-end'
      className?: string
    }>
  }
}

export type TextareaField<TInput extends FieldValues> = BaseField<TInput> & {
  type: 'textarea'
  placeholder?: string
  rows?: number
  textareaClassName?: string
  inputGroup?: {
    className?: string
    addons?: Array<{
      content: React.ReactNode
      align?: 'inline-start' | 'inline-end' | 'block-start' | 'block-end'
      className?: string
    }>
  }
}

export type SelectField<
  TInput extends FieldValues,
  TValue extends string = string,
> = BaseField<TInput> & {
  type: 'select'
  placeholder?: string
  triggerClassName?: string
  items: Array<{ label: string; value: TValue }>
  parse: (value: string) => TValue
}

export type CheckboxField<TInput extends FieldValues> = BaseField<TInput> & {
  type: 'checkbox'
  checkboxClassName?: string
}

export type SwitchField<TInput extends FieldValues> = BaseField<TInput> & {
  type: 'switch'
  switchClassName?: string
}

export type DateField<TInput extends FieldValues> = BaseField<TInput> & {
  type: 'date-picker'
  placeholder?: string
  format?: string // e.g., "PPP" for "MMM dd, yyyy"
  inputClassName?: string
}

export type FormFieldConfig<TInput extends FieldValues> =
  | InputField<TInput>
  | TextareaField<TInput>
  | SelectField<TInput>
  | CheckboxField<TInput>
  | SwitchField<TInput>
  | DateField<TInput>

export type FormAction = {
  id?: string
  label: string
  type?: 'submit' | 'button'
  variant?: React.ComponentProps<typeof Button>['variant']
  size?: React.ComponentProps<typeof Button>['size']
  onClick?: () => void
  disabled?: boolean
  loading?: boolean
  className?: string
}

export type FormInput<TSchema extends ZodObject> = input<TSchema>
export type FormOutput<TSchema extends ZodObject> = FormInput<TSchema>

export type FieldShorthand<TSchema extends ZodObject> = FieldPath<
  FormInput<TSchema>
>

export type FieldOrShorthand<TSchema extends ZodObject> =
  | FieldShorthand<TSchema>
  | FormFieldConfig<FormInput<TSchema>>

export type FormStructure<TSchema extends ZodObject> = Array<
  Array<FieldOrShorthand<TSchema>>
>

export type FormExtras<TSchema extends ZodObject> = {
  top?: Slot<FormInput<TSchema>>
  error?: Slot<FormInput<TSchema>>
  afterFields?: Slot<FormInput<TSchema>>
  bottom?: Slot<FormInput<TSchema>>
}

export type FormConfig<TSchema extends ZodObject> = {
  structure: FormStructure<TSchema>
  extras?: FormExtras<TSchema>
}

export type FormBuilderProps<TSchema extends ZodObject> = {
  schema: TSchema
  config: FormConfig<TSchema>
  defaultValues?: DefaultValues<FormInput<TSchema>>
  preload?: (
    form: UseFormReturn<FormInput<TSchema>, unknown, FormInput<TSchema>>,
  ) => void | Promise<void>
  isLoading?: boolean
  isFetching?: boolean
  disabled?: boolean
  actionsPlacement?: 'top' | 'bottom' | 'both'
  showErrorSummary?: boolean
  toastErrors?: boolean
  showSuccessAlert?: boolean
  successMessage?: string
  onSubmit: (
    values: FormInput<TSchema>,
    form: UseFormReturn<FormInput<TSchema>, unknown, FormInput<TSchema>>,
  ) => void | Promise<void>
  onError?: (error: unknown) => void
  actions: Array<FormAction>
  formId?: string
  className?: string
}
