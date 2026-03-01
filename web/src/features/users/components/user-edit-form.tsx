import * as React from 'react'
import { updateUserSchema } from '../schemas'
import type { UpdateUserValues } from '../schemas'
import type { User } from '../types'
import type { UseFormReturn } from 'react-hook-form'
import { Button } from '@/components/ui/button'
import { Spinner } from '@/components/ui/spinner'
import { zRoleEnum } from '@/lib/api/zod.gen'
import { FormBuilder, defineFormConfig } from '@/components/form-builder'

interface UserEditFormProps {
  user: User
  onConfirm: (data: UpdateUserValues) => void
  onOpenChange: (open: boolean) => void
  isSubmitting?: boolean
}

export function UserEditForm({
  user,
  onConfirm,
  onOpenChange,
  isSubmitting,
}: UserEditFormProps) {
  const availableRoles = {
    data: Object.values(zRoleEnum.enum).map((roleName) => ({
      id: roleName,
      name: roleName,
    })),
  }
  const preload = React.useCallback(
    (form: UseFormReturn<UpdateUserValues, unknown, UpdateUserValues>) => {
      if (user) {
        form.reset({
          email: user.email,
          is_verified: user.is_verified ?? false,
          lockout_until: user.lockout_until ?? undefined,
          role: user.role ?? undefined,
        })
      }
    },
    [user],
  )

  const config = defineFormConfig(updateUserSchema, {
    structure: [
      [
        {
          field: 'email',
          type: 'input',
          label: 'Email Address',
          placeholder: 'email@example.com',
        },
      ],
      [
        {
          field: 'role',
          type: 'select',
          label: 'Role',
          placeholder: 'Select Role',
          items: availableRoles.data.map((role) => ({
            label: role.name,
            value: role.name,
          })),
          parse: (value) => zRoleEnum.parse(value),
        },
      ],
      [
        {
          field: 'is_verified',
          type: 'switch',
          label: 'Verified',
          description: 'User can log in and access authorized areas.',
        },
      ],
      [
        {
          field: 'lockout_until',
          type: 'date-picker',
          label: 'Lockout Until',
          description: 'Prevent user from logging in until this date.',
        },
      ],
    ],
    extras: {
      bottom: (
        <div className="flex items-center justify-end gap-2 border-t border-border/40 bg-muted/20 px-6 py-4 mt-8 -mx-6 -mb-6">
          <Button
            type="button"
            variant="outline"
            onClick={() => onOpenChange(false)}
          >
            Cancel
          </Button>
          <Button type="submit" disabled={isSubmitting}>
            {isSubmitting && <Spinner className="mr-2 h-4 w-4" />}
            Save Changes
          </Button>
        </div>
      ),
    },
  })

  return (
    <FormBuilder
      schema={updateUserSchema}
      config={config}
      onSubmit={(values) => onConfirm(values)}
      preload={preload}
      isLoading={isSubmitting}
      showErrorSummary={false}
      toastErrors={false}
      showSuccessAlert={false}
      actions={[]}
      className="space-y-6 pt-4"
    />
  )
}
