import * as React from 'react'
import { signUpSchema } from '../../auth/schemas'
import type { SignUpFormValues } from '../../auth/schemas'
import type { UseFormReturn } from 'react-hook-form'
import { Button } from '@/components/ui/button'
import {
  Dialog,
  DialogContent,
  DialogDescription,
  DialogFooter,
  DialogHeader,
  DialogTitle,
} from '@/components/ui/dialog'
import { Spinner } from '@/components/ui/spinner'
import { FormBuilder, defineFormConfig } from '@/components/form-builder'

interface UserCreateDialogProps {
  open: boolean
  onOpenChange: (open: boolean) => void
  onConfirm: (data: SignUpFormValues) => void
  isSubmitting?: boolean
}

export function UserCreateDialog({
  open,
  onOpenChange,
  onConfirm,
  isSubmitting,
}: UserCreateDialogProps) {
  const preload = React.useCallback(
    (form: UseFormReturn<SignUpFormValues, unknown, SignUpFormValues>) => {
      if (open) {
        form.reset()
      }
    },
    [open],
  )

  const config = defineFormConfig(signUpSchema, {
    structure: [
      [
        {
          field: 'name',
          type: 'input',
          label: 'Full Name',
          placeholder: 'John Doe',
        },
      ],
      [
        {
          field: 'email',
          type: 'input',
          label: 'Email Address',
          placeholder: 'john@example.com',
        },
      ],
      [
        {
          field: 'password',
          type: 'input',
          label: 'Password',
          inputType: 'password',
        },
      ],
      [
        {
          field: 'confirmPassword',
          type: 'input',
          label: 'Confirm Password',
          inputType: 'password',
        },
      ],
    ],
    extras: {
      bottom: (
        <DialogFooter>
          <Button
            type="button"
            variant="outline"
            onClick={() => onOpenChange(false)}
          >
            Cancel
          </Button>
          <Button type="submit" disabled={isSubmitting}>
            {isSubmitting && <Spinner className="mr-2 h-4 w-4" />}
            Create Account
          </Button>
        </DialogFooter>
      ),
    },
  })

  return (
    <Dialog open={open} onOpenChange={onOpenChange}>
      <DialogContent className="sm:max-w-[425px]">
        <DialogHeader>
          <DialogTitle>Create User</DialogTitle>
          <DialogDescription>
            Add a new user to the organization.
          </DialogDescription>
        </DialogHeader>
        <FormBuilder
          schema={signUpSchema}
          config={config}
          onSubmit={(values) => onConfirm(values)}
          preload={preload}
          isLoading={isSubmitting}
          showErrorSummary={false}
          toastErrors={false}
          showSuccessAlert={false}
          actions={[]}
          className="space-y-4 pt-4"
        />
      </DialogContent>
    </Dialog>
  )
}
