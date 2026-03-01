import * as React from 'react'
import { UserAdd01Icon } from '@hugeicons/core-free-icons'
import { HugeiconsIcon } from '@hugeicons/react'

import { signUpSchema } from '../../auth/schemas'
import type { SignUpFormValues } from '../../auth/schemas'
import type { UseFormReturn } from 'react-hook-form'
import { Button } from '@/components/ui/button'
import {
  Dialog,
  DialogContent,
  DialogDescription,
  DialogTitle,
} from '@/components/ui/dialog'
import { Spinner } from '@/components/ui/spinner'
import { FormBuilder, defineFormConfig } from '@/components/form-builder'
import { Stack } from '@/components/primitives'

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
        <div className="flex items-center justify-end gap-2 border-t border-border/40 bg-muted/20 px-6 py-4 mt-6 -mx-6 -mb-6">
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
        </div>
      ),
    },
  })

  return (
    <Dialog open={open} onOpenChange={onOpenChange}>
      <DialogContent className="p-0 overflow-hidden sm:max-w-[425px] border-border/60 shadow-xl">
        <div className="flex flex-col border-b border-border/40 bg-muted/20 p-6 pb-6">
          <div className="flex gap-4 items-start">
            <div className="flex h-10 w-10 shrink-0 items-center justify-center rounded-full bg-primary/10 ring-1 ring-primary/20">
              <HugeiconsIcon icon={UserAdd01Icon} className="size-5 text-primary" />
            </div>
            <Stack gap={1} className="pt-1">
              <DialogTitle className="text-xl">Create User</DialogTitle>
              <DialogDescription className="text-sm">
                Add a new user to the organization.
              </DialogDescription>
            </Stack>
          </div>
        </div>

        <div className="px-6 pb-6 pt-4">
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
            className="space-y-4"
          />
        </div>
      </DialogContent>
    </Dialog>
  )
}
