'use client'

import { zodResolver } from '@hookform/resolvers/zod'
import { useForm } from 'react-hook-form'
import * as React from 'react'
import { toast } from 'sonner'
import { HugeiconsIcon } from '@hugeicons/react'
import { Alert02Icon } from '@hugeicons/core-free-icons'
import { FormFieldRenderer } from './fields'
import type { FieldValues, UseFormReturn } from 'react-hook-form'
import type { ZodObject } from 'zod'

import type {
  FieldOrShorthand,
  FormBuilderProps,
  FormConfig,
  FormFieldConfig,
  FormInput,
  Slot,
} from './types'
import { Button } from '@/components/ui/button'
import { Alert, AlertDescription, AlertTitle } from '@/components/ui/alert'
import { Form } from '@/components/ui/form'
import { Spinner } from '@/components/ui/spinner'
import { cn } from '@/lib/utils'

export function createFormConfig<T extends FieldValues>() {
  return (fields: Array<FormFieldConfig<T>>) => fields
}

function renderSlot<TInput extends FieldValues>(
  slot: Slot<TInput> | undefined,
  form: UseFormReturn<TInput, unknown, TInput>,
): React.ReactNode {
  if (!slot) return null
  return typeof slot === 'function' ? slot(form) : slot
}

function inferLabel(field: string) {
  return field
    .split('.')
    .pop()
    ?.split('_')
    .map((part) => part.charAt(0).toUpperCase() + part.slice(1))
    .join(' ')
}

function normalizeField<TSchema extends ZodObject>(
  field: FieldOrShorthand<TSchema>,
): FormFieldConfig<FormInput<TSchema>> {
  if (typeof field === 'string') {
    return {
      type: 'input',
      field,
      label: inferLabel(field) ?? field,
    }
  }

  return {
    ...field,
    label: field.label ?? inferLabel(field.field) ?? field.field,
  }
}

export function defineFormConfig<TSchema extends ZodObject>(
  schema: TSchema,
  config: FormConfig<TSchema>,
) {
  void schema
  return config
}

function rowClassName(columnCount: number) {
  if (columnCount === 1) return 'grid gap-4'
  if (columnCount === 2) return 'grid gap-4 sm:grid-cols-2'
  if (columnCount === 3) return 'grid gap-4 sm:grid-cols-3'
  if (columnCount === 4) return 'grid gap-4 sm:grid-cols-4'
  return 'grid gap-4'
}

type ErrorSummaryItem = {
  field: string
  message: string
}

function isObject(value: unknown): value is Record<string, unknown> {
  return typeof value === 'object' && value !== null
}

function extractMessageFromUnknown(value: unknown): string | null {
  if (typeof value === 'string') return value
  if (Array.isArray(value)) {
    for (const item of value) {
      const message = extractMessageFromUnknown(item)
      if (message) return message
    }
    return null
  }
  if (!isObject(value)) return null
  if (!('message' in value)) return null
  return extractMessageFromUnknown(value.message)
}

export function normalizeErrorMessage(message: string): string {
  const trimmed = message.trim()
  if (!trimmed.startsWith('[') && !trimmed.startsWith('{')) {
    return message
  }

  try {
    const parsed: unknown = JSON.parse(trimmed)
    const messageFromParsed = extractMessageFromUnknown(parsed)
    if (messageFromParsed) return messageFromParsed
  } catch {
    return message
  }

  return message
}

function collectErrorSummary(
  errors: unknown,
  prefix = '',
): Array<ErrorSummaryItem> {
  const summary: Array<ErrorSummaryItem> = []

  if (!isObject(errors)) {
    return summary
  }

  Object.entries(errors).forEach(([key, value]) => {
    if (!value) return
    const path = prefix ? `${prefix}.${key}` : key

    const fieldMessage = extractMessageFromUnknown(value)
    if (fieldMessage) {
      summary.push({
        field: path,
        message: normalizeErrorMessage(fieldMessage),
      })
      return
    }

    if (isObject(value)) {
      summary.push(...collectErrorSummary(value, path))
    }
  })

  return summary
}

export function FormBuilder<TSchema extends ZodObject>({
  schema,
  config,
  defaultValues,
  preload,
  isLoading,
  isFetching,
  disabled,
  actionsPlacement = 'bottom',
  showErrorSummary = true,
  toastErrors = true,
  showSuccessAlert = true,
  successMessage = 'Changes saved successfully.',
  onSubmit,
  onError,
  actions,
  formId,
  className,
}: FormBuilderProps<TSchema>) {
  type TInput = FormInput<TSchema>

  const [isPreloading, setIsPreloading] = React.useState(false)
  const [lastSuccessMessage, setLastSuccessMessage] = React.useState<
    string | null
  >(null)

  const form = useForm<TInput, unknown, TInput>({
    resolver: zodResolver(schema, undefined, { raw: true }),
    defaultValues,
  })

  React.useEffect(() => {
    if (!preload) return
    let active = true
    setIsPreloading(true)
    Promise.resolve(preload(form))
      .catch((err) => {
        console.error('Form preload failed:', err)
      })
      .finally(() => {
        if (active) setIsPreloading(false)
      })
    return () => {
      active = false
    }
  }, [preload, form])

  const handleSubmit = form.handleSubmit(
    async (values) => {
      try {
        setLastSuccessMessage(null)
        await onSubmit(values, form)
        setLastSuccessMessage(successMessage)
      } catch (err) {
        setLastSuccessMessage(null)
        if (toastErrors) {
          toast.error(
            err instanceof Error
              ? err.message
              : 'Something went wrong while submitting the form.',
          )
        }
        onError?.(err)
      }
    },
    (errors) => {
      if (!toastErrors) return
      setLastSuccessMessage(null)
      const summary = collectErrorSummary(errors)
      if (summary.length > 0) {
        toast.error(summary[0].message)
      } else {
        toast.error('Please check the form for errors.')
      }
    },
  )

  const isBusy = Boolean(isPreloading || isLoading || isFetching)
  const isDisabled = Boolean(disabled || isBusy)
  const showSkeleton = Boolean(isPreloading || isFetching)
  const rawErrorSummary = showErrorSummary
    ? collectErrorSummary(form.formState.errors)
    : []
  const errorSummary = rawErrorSummary.length > 1 ? rawErrorSummary : []

  React.useEffect(() => {
    if (rawErrorSummary.length > 0) {
      setLastSuccessMessage(null)
    }
  }, [rawErrorSummary.length])

  const actionsBlock =
    actions.length > 0 ? (
      <div className="flex flex-col gap-2">
        {actions.map((action) => (
          <Button
            key={action.id ?? action.label}
            type={action.type ?? 'button'}
            variant={action.variant}
            size={action.size}
            onClick={action.onClick}
            disabled={action.disabled || isDisabled}
            className={action.className}
          >
            {action.loading || isBusy ? <Spinner className="mr-2" /> : null}
            {action.label}
          </Button>
        ))}
      </div>
    ) : null

  return (
    <Form {...form}>
      <form
        id={formId}
        onSubmit={handleSubmit}
        className={cn('space-y-4', className)}
      >
        {renderSlot(config.extras?.top, form)}

        {renderSlot(config.extras?.error, form)}

        {showSuccessAlert && lastSuccessMessage ? (
          <Alert className="text-green-500 bg-green-500/10 border-green-500/20">
            <HugeiconsIcon icon={Alert02Icon} className="h-4 w-4" />
            <AlertTitle>Success</AlertTitle>
            <AlertDescription>{lastSuccessMessage}</AlertDescription>
          </Alert>
        ) : null}

        {showErrorSummary && errorSummary.length > 0 ? (
          <Alert variant="destructive">
            <HugeiconsIcon icon={Alert02Icon} className="h-4 w-4" />
            <AlertTitle>Fix the highlighted fields</AlertTitle>
            <AlertDescription>
              <ul className="list-disc space-y-1 pl-4">
                {errorSummary.map((item) => (
                  <li key={item.field} className="text-sm">
                    {item.message}
                  </li>
                ))}
              </ul>
            </AlertDescription>
          </Alert>
        ) : null}

        {actionsPlacement === 'top' || actionsPlacement === 'both'
          ? actionsBlock
          : null}

        {config.structure.map((row, rowIndex) => (
          <div
            key={`form-row-${rowIndex}`}
            className={rowClassName(row.length)}
          >
            {row.map((fieldConfig) => {
              const normalized = normalizeField<TSchema>(fieldConfig)
              return (
                <FormFieldRenderer
                  key={normalized.field}
                  fieldConfig={normalized}
                  form={form}
                  formDisabled={isDisabled}
                  showSkeleton={showSkeleton}
                />
              )
            })}
          </div>
        ))}

        {renderSlot(config.extras?.afterFields, form)}

        {actionsPlacement === 'bottom' || actionsPlacement === 'both'
          ? actionsBlock
          : null}

        {renderSlot(config.extras?.bottom, form)}
      </form>
    </Form>
  )
}

export type {
  FormAction,
  FormBuilderProps,
  FormFieldConfig,
  FormConfig,
  FormExtras,
  FormStructure,
  FieldOrShorthand,
  FieldShorthand,
  BaseField,
  InputField,
  TextareaField,
  SelectField,
  CheckboxField,
  SwitchField,
  Slot,
} from './types'
