import { format } from 'date-fns'
import { HugeiconsIcon } from '@hugeicons/react'
import { Calendar01Icon } from '@hugeicons/core-free-icons'
import { Controller } from 'react-hook-form'
import type {
  ControllerFieldState,
  ControllerRenderProps,
  FieldValues,
  Path,
  UseFormReturn,
} from 'react-hook-form'

import type { FormFieldConfig } from './types'
import { Button } from '@/components/ui/button'
import { Checkbox } from '@/components/ui/checkbox'
import { Field, FieldError, FieldLabel } from '@/components/ui/field'
import { Input } from '@/components/ui/input'
import {
  InputGroup,
  InputGroupAddon,
  InputGroupInput,
  InputGroupTextarea,
} from '@/components/ui/input-group'
import { Calendar } from '@/components/ui/calendar'
import {
  Popover,
  PopoverContent,
  PopoverTrigger,
} from '@/components/ui/popover'
import {
  Select,
  SelectContent,
  SelectItem,
  SelectTrigger,
  SelectValue,
} from '@/components/ui/select'
import { Skeleton } from '@/components/ui/skeleton'
import { Switch } from '@/components/ui/switch'
import { Textarea } from '@/components/ui/textarea'
import { cn } from '@/lib/utils'

function LocalFieldDescription({ description }: { description?: string }) {
  if (!description) return null
  return <p className="text-xs text-muted-foreground">{description}</p>
}

function parseDateValue(value: unknown) {
  if (typeof value !== 'string' || !value) return undefined
  const date = new Date(value)
  return Number.isNaN(date.getTime()) ? undefined : date
}

function getTimeValue(value: unknown) {
  if (typeof value !== 'string') return '00:00'
  const match = value.match(/T(\d{2}:\d{2})/)
  return match?.[1] ?? '00:00'
}

function formatDateValue(
  date: Date,
  inputType: React.HTMLInputTypeAttribute | undefined,
  existingValue: unknown,
) {
  if (inputType === 'datetime-local') {
    return `${format(date, 'yyyy-MM-dd')}T${getTimeValue(existingValue)}`
  }
  return format(date, 'yyyy-MM-dd')
}

type FormFieldRendererProps<TInput extends FieldValues> = {
  fieldConfig: FormFieldConfig<TInput>
  form: UseFormReturn<TInput, unknown, TInput>
  formDisabled?: boolean
  showSkeleton?: boolean
}

export function FormFieldRenderer<TInput extends FieldValues>({
  fieldConfig,
  form,
  formDisabled,
  showSkeleton,
}: FormFieldRendererProps<TInput>) {
  const disabled = fieldConfig.disabled || formDisabled

  if (showSkeleton) {
    const description = fieldConfig.description ? (
      <Skeleton className="h-3 w-40 rounded-md" />
    ) : null

    if (fieldConfig.type === 'checkbox') {
      return (
        <Field className={fieldConfig.className}>
          <div className="flex items-center gap-2">
            <Skeleton className="h-4 w-4 rounded-[4px]" />
            <Skeleton className="h-4 w-24 rounded-md" />
          </div>
          {description}
        </Field>
      )
    }

    if (fieldConfig.type === 'switch') {
      return (
        <Field className={fieldConfig.className}>
          <div className="flex items-center justify-between">
            <div className="space-y-2">
              <Skeleton className="h-4 w-28 rounded-md" />
              {description}
            </div>
            <Skeleton className="h-[18.4px] w-[32px] rounded-full" />
          </div>
        </Field>
      )
    }

    return (
      <Field className={fieldConfig.className}>
        <Skeleton className="h-4 w-28 rounded-md" />
        <Skeleton
          className={
            fieldConfig.type === 'textarea' ? 'min-h-16 w-full' : 'h-8 w-full'
          }
        />
        {description}
      </Field>
    )
  }

  if (fieldConfig.type === 'date-picker') {
    return (
      <Controller
        name={fieldConfig.field}
        control={form.control}
        render={({
          field,
          fieldState,
        }: {
          field: ControllerRenderProps<TInput, Path<TInput>>
          fieldState: ControllerFieldState
        }) => (
          <Field
            data-invalid={fieldState.invalid}
            className={fieldConfig.className}
          >
            <FieldLabel
              htmlFor={fieldConfig.field}
              className={fieldConfig.labelClassName}
            >
              {fieldConfig.label ?? ''}
            </FieldLabel>
            <Popover>
              <PopoverTrigger
                disabled={disabled}
                render={
                  <Input
                    {...field}
                    id={fieldConfig.field}
                    placeholder={fieldConfig.placeholder ?? 'Pick a date'}
                    value={
                      parseDateValue(field.value)
                        ? format(
                            parseDateValue(field.value) ?? new Date(),
                            fieldConfig.format ?? 'PPP',
                          )
                        : ''
                    }
                    disabled={disabled}
                    className={cn(
                      'w-full pl-3 text-left font-normal',
                      !field.value && 'text-muted-foreground',
                      fieldConfig.inputClassName,
                    )}
                    aria-invalid={fieldState.invalid}
                  />
                }
              />
              <PopoverContent className="w-auto p-0 rounded-xl bg-background border-none shadow-xl">
                <Calendar
                  mode="single"
                  selected={parseDateValue(field.value)}
                  onSelect={(date) => {
                    const nextValue = date ? format(date, 'yyyy-MM-dd') : null
                    field.onChange(nextValue)
                    fieldConfig.onValueChange?.(nextValue, form)
                  }}
                  initialFocus
                />
              </PopoverContent>
            </Popover>
            <LocalFieldDescription description={fieldConfig.description} />
            {fieldState.invalid && <FieldError errors={[fieldState.error]} />}
          </Field>
        )}
      />
    )
  }

  if (fieldConfig.type === 'input') {
    return (
      <Controller
        name={fieldConfig.field}
        control={form.control}
        render={({
          field,
          fieldState,
        }: {
          field: ControllerRenderProps<TInput, Path<TInput>>
          fieldState: ControllerFieldState
        }) => (
          <Field
            data-invalid={fieldState.invalid}
            className={fieldConfig.className}
          >
            <FieldLabel htmlFor={fieldConfig.field}>
              {fieldConfig.label ?? ''}
            </FieldLabel>
            {fieldConfig.inputType === 'datetime-local' ? (
              <Popover>
                <PopoverTrigger
                  disabled={disabled}
                  render={
                    <Button
                      variant="outline"
                      disabled={disabled}
                      className={cn(
                        'w-full justify-start text-left font-normal h-10 rounded-xl border-none bg-muted/50 ring-1 ring-border focus-visible:ring-2 focus-visible:ring-primary shadow-sm',
                        !parseDateValue(field.value) && 'text-muted-foreground',
                        fieldConfig.inputClassName,
                      )}
                    >
                      <HugeiconsIcon
                        icon={Calendar01Icon}
                        className="mr-2 h-4 w-4"
                      />
                      {parseDateValue(field.value) ? (
                        format(
                          parseDateValue(field.value) ?? new Date(),
                          'PPP p',
                        )
                      ) : (
                        <span>{fieldConfig.placeholder ?? 'Pick a date'}</span>
                      )}
                    </Button>
                  }
                />
                <PopoverContent className="w-auto p-0 rounded-xl bg-background border-none shadow-xl">
                  <Calendar
                    mode="single"
                    selected={parseDateValue(field.value)}
                    onSelect={(date) => {
                      if (!date) {
                        field.onChange(null)
                        return
                      }
                      field.onChange(
                        formatDateValue(
                          date,
                          fieldConfig.inputType,
                          field.value,
                        ),
                      )
                    }}
                    initialFocus
                  />
                </PopoverContent>
              </Popover>
            ) : fieldConfig.inputGroup ? (
              <InputGroup
                data-disabled={disabled}
                className={fieldConfig.inputGroup.className}
              >
                {fieldConfig.inputGroup.addons?.map((addon, index) => (
                  <InputGroupAddon
                    key={`${fieldConfig.field}-addon-${index}`}
                    align={addon.align}
                    className={addon.className}
                  >
                    {addon.content}
                  </InputGroupAddon>
                ))}
                <InputGroupInput
                  {...field}
                  onChange={(e) => {
                    const val = e.target.value
                    field.onChange(
                      fieldConfig.parse ? fieldConfig.parse(val) : val,
                    )
                  }}
                  type={fieldConfig.inputType}
                  placeholder={fieldConfig.placeholder}
                  disabled={disabled}
                  className={fieldConfig.inputClassName}
                  aria-invalid={fieldState.invalid}
                />
              </InputGroup>
            ) : (
              <Input
                {...field}
                onChange={(e) => {
                  const val = e.target.value
                  field.onChange(
                    fieldConfig.parse ? fieldConfig.parse(val) : val,
                  )
                }}
                type={fieldConfig.inputType}
                placeholder={fieldConfig.placeholder}
                disabled={disabled}
                className={fieldConfig.inputClassName}
                aria-invalid={fieldState.invalid}
              />
            )}
            <LocalFieldDescription description={fieldConfig.description} />
            {fieldState.invalid && <FieldError errors={[fieldState.error]} />}
          </Field>
        )}
      />
    )
  }

  if (fieldConfig.type === 'textarea') {
    return (
      <Controller
        name={fieldConfig.field}
        control={form.control}
        render={({
          field,
          fieldState,
        }: {
          field: ControllerRenderProps<TInput, Path<TInput>>
          fieldState: ControllerFieldState
        }) => (
          <Field
            data-invalid={fieldState.invalid}
            className={fieldConfig.className}
          >
            <FieldLabel htmlFor={fieldConfig.field}>
              {fieldConfig.label ?? ''}
            </FieldLabel>
            {fieldConfig.inputGroup ? (
              <InputGroup
                data-disabled={disabled}
                className={cn('h-auto', fieldConfig.inputGroup.className)}
              >
                {fieldConfig.inputGroup.addons?.map((addon, index) => (
                  <InputGroupAddon
                    key={`${fieldConfig.field}-addon-${index}`}
                    align={addon.align}
                    className={addon.className}
                  >
                    {addon.content}
                  </InputGroupAddon>
                ))}
                <InputGroupTextarea
                  {...field}
                  onChange={(e) => {
                    const val = e.target.value
                    field.onChange(
                      fieldConfig.parse ? fieldConfig.parse(val) : val,
                    )
                  }}
                  rows={fieldConfig.rows}
                  placeholder={fieldConfig.placeholder}
                  disabled={disabled}
                  className={fieldConfig.textareaClassName}
                  aria-invalid={fieldState.invalid}
                />
              </InputGroup>
            ) : (
              <Textarea
                {...field}
                onChange={(e) => {
                  const val = e.target.value
                  field.onChange(
                    fieldConfig.parse ? fieldConfig.parse(val) : val,
                  )
                }}
                rows={fieldConfig.rows}
                placeholder={fieldConfig.placeholder}
                disabled={disabled}
                className={fieldConfig.textareaClassName}
                aria-invalid={fieldState.invalid}
              />
            )}
            <LocalFieldDescription description={fieldConfig.description} />
            {fieldState.invalid && <FieldError errors={[fieldState.error]} />}
          </Field>
        )}
      />
    )
  }

  if (fieldConfig.type === 'select') {
    return (
      <Controller
        name={fieldConfig.field}
        control={form.control}
        render={({
          field,
          fieldState,
        }: {
          field: ControllerRenderProps<TInput, Path<TInput>>
          fieldState: ControllerFieldState
        }) => (
          <Field
            data-invalid={fieldState.invalid}
            className={fieldConfig.className}
          >
            <FieldLabel htmlFor={fieldConfig.field}>
              {fieldConfig.label ?? ''}
            </FieldLabel>
            <Select
              {...field}
              value={typeof field.value === 'string' ? field.value : ''}
              onValueChange={(value) => {
                if (value === null) return
                field.onChange(fieldConfig.parse(value))
                fieldConfig.onValueChange?.(value, form)
              }}
              disabled={disabled}
            >
              <SelectTrigger
                id={fieldConfig.field}
                className={cn('w-full', fieldConfig.triggerClassName)}
                aria-invalid={fieldState.invalid}
              >
                <SelectValue placeholder={fieldConfig.placeholder} />
              </SelectTrigger>
              <SelectContent>
                {fieldConfig.items.map((item) => (
                  <SelectItem key={item.value} value={item.value}>
                    {item.label}
                  </SelectItem>
                ))}
              </SelectContent>
            </Select>
            <LocalFieldDescription description={fieldConfig.description} />
            {fieldState.invalid && <FieldError errors={[fieldState.error]} />}
          </Field>
        )}
      />
    )
  }

  if (fieldConfig.type === 'checkbox') {
    return (
      <Controller
        name={fieldConfig.field}
        control={form.control}
        render={({
          field,
          fieldState,
        }: {
          field: ControllerRenderProps<TInput, Path<TInput>>
          fieldState: ControllerFieldState
        }) => (
          <Field
            data-invalid={fieldState.invalid}
            className={fieldConfig.className}
          >
            <div className="flex items-center gap-2">
              <Checkbox
                {...field}
                id={fieldConfig.field}
                checked={Boolean(field.value)}
                onCheckedChange={field.onChange}
                disabled={disabled}
                className={fieldConfig.checkboxClassName}
                aria-invalid={fieldState.invalid}
              />
              <FieldLabel
                htmlFor={fieldConfig.field}
                className={cn('text-sm', fieldConfig.labelClassName)}
              >
                {fieldConfig.label ?? ''}
              </FieldLabel>
            </div>
            <LocalFieldDescription description={fieldConfig.description} />
            {fieldState.invalid && <FieldError errors={[fieldState.error]} />}
          </Field>
        )}
      />
    )
  }

  return (
    <Controller
      name={fieldConfig.field}
      control={form.control}
      render={({
        field,
        fieldState,
      }: {
        field: ControllerRenderProps<TInput, Path<TInput>>
        fieldState: ControllerFieldState
      }) => (
        <Field
          data-invalid={fieldState.invalid}
          className={fieldConfig.className}
        >
          <div className="flex items-center justify-between">
            <div>
              <FieldLabel
                htmlFor={fieldConfig.field}
                className={cn('text-sm', fieldConfig.labelClassName)}
              >
                {fieldConfig.label ?? ''}
              </FieldLabel>
              <LocalFieldDescription description={fieldConfig.description} />
            </div>
            <Switch
              {...field}
              id={fieldConfig.field}
              checked={Boolean(field.value)}
              onCheckedChange={field.onChange}
              disabled={disabled}
              className={fieldConfig.switchClassName}
              aria-invalid={fieldState.invalid}
            />
          </div>
          {fieldState.invalid && <FieldError errors={[fieldState.error]} />}
        </Field>
      )}
    />
  )
}
