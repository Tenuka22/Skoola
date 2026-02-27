import { HugeiconsIcon } from '@hugeicons/react'
import { AlertCircle } from '@hugeicons/core-free-icons'
import { useMutation } from '@tanstack/react-query'
import { useNavigate } from '@tanstack/react-router'
import { signUpSchema } from '../schemas'
import { ActiveSessions } from './active-sessions'
import type { SignUpFormValues } from '../schemas'
import type { AuthStorage } from '@/lib/auth/session'
import type { UseFormReturn } from 'react-hook-form'
import { Button } from '@/components/ui/button'
import { Alert, AlertDescription } from '@/components/ui/alert'
import { signUpFn } from '@/lib/auth/actions'
import { Spinner } from '@/components/ui/spinner'
import {
  FormBuilder,
  defineFormConfig,
  normalizeErrorMessage,
} from '@/components/form-builder'

export function SignUpForm({
  authStorage,
}: {
  authStorage: AuthStorage | null
}) {
  const signUpMutation = useMutation({
    mutationFn: signUpFn,
  })

  const navigate = useNavigate()

  const onSubmit = async (
    data: SignUpFormValues,
    form: UseFormReturn<SignUpFormValues, unknown, SignUpFormValues>,
  ) => {
    try {
      const result = await signUpMutation.mutateAsync({
        data: {
          name: data.name,
          email: data.email,
          password: data.password,
        },
      })

      if (result?.success) {
        setTimeout(() => {
          navigate({ from: '/login' })
        }, 2000)
        return
      }

      if (result?.error) {
        form.setError('root.serverError', {
          type: 'server',
          message: normalizeErrorMessage(result.error),
        })
      }
    } catch (err: unknown) {
      console.error('Sign Up error in component:', err)
      form.setError('root.serverError', {
        type: 'server',
        message:
          err instanceof Error
            ? normalizeErrorMessage(err.message)
            : 'Sign Up failed due to an unknown error.',
      })
    }
  }

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
          label: 'Email',
          inputType: 'email',
          placeholder: 'm@example.com',
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
      top: <ActiveSessions authStorage={authStorage} />,
      afterFields: (form) =>
        signUpMutation.error || form.formState.errors.root?.serverError ? (
          <Alert variant="destructive">
            <HugeiconsIcon icon={AlertCircle} className="h-4 w-4" />
            <AlertDescription>
              {normalizeErrorMessage(
                signUpMutation.error?.message ||
                  form.formState.errors.root?.serverError?.message ||
                  'Sign Up failed. Please try again.',
              )}
            </AlertDescription>
          </Alert>
        ) : null,
      bottom: (
        <Button
          type="submit"
          className="w-full gap-2"
          disabled={signUpMutation.isPending}
        >
          {signUpMutation.isPending && <Spinner />}
          Sign Up
        </Button>
      ),
    },
  })

  return (
    <FormBuilder
      schema={signUpSchema}
      config={config}
      onSubmit={onSubmit}
      isLoading={signUpMutation.isPending}
      showErrorSummary={false}
      toastErrors={false}
      showSuccessAlert={false}
      actions={[]}
      className="space-y-4"
    />
  )
}
