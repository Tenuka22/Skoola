import { useForm } from 'react-hook-form'
import { zodResolver } from '@hookform/resolvers/zod'
import { HugeiconsIcon } from '@hugeicons/react'
import { AlertCircle } from '@hugeicons/core-free-icons'
import { useMutation } from '@tanstack/react-query'
import { useNavigate } from '@tanstack/react-router'
import { signUpSchema } from '../schemas'
import type { SignUpFormValues } from '../schemas'
import type { AuthStorage } from '@/lib/auth/session'
import { Button } from '@/components/ui/button'
import { Input } from '@/components/ui/input'
import {
  Field,
  FieldError,
  FieldGroup,
  FieldLabel,
} from '@/components/ui/field'
import { Alert, AlertDescription } from '@/components/ui/alert'
import { Avatar, AvatarFallback, AvatarImage } from '@/components/ui/avatar'
import { signUpFn } from '@/lib/auth/actions'
import {
  Tooltip,
  TooltipContent,
  TooltipProvider,
  TooltipTrigger,
} from '@/components/ui/tooltip'
import { Spinner } from '@/components/ui/spinner'
import { cn } from '@/lib/utils'
import { ActiveSessions } from './active-sessions'

export function SignUpForm({
  authStorage,
}: {
  authStorage: AuthStorage | null
}) {
  const {
    register,
    handleSubmit,
    formState: { errors, isSubmitting },
    setError: setFormError,
  } = useForm<SignUpFormValues>({
    resolver: zodResolver(signUpSchema),
  })

  const signUpMutation = useMutation({
    mutationFn: signUpFn,
  })

  const navigate = useNavigate()

  const onSubmit = async (data: SignUpFormValues) => {
    try {
      const result = await signUpMutation.mutateAsync({
        data: {
          name: data.name,
          email: data.email,
          password: data.password,
        },
      })

      if (result?.success) {
        navigate({ to: '/login' })
        return
      }

      if (result?.error) {
        setFormError('root.serverError', {
          type: 'server',
          message: result.error,
        })
      }
    } catch (err: unknown) {
      console.error('Sign Up error in component:', err)
      setFormError('root.serverError', {
        type: 'server',
        message:
          err instanceof Error
            ? err.message
            : 'Sign Up failed due to an unknown error.',
      })
    }
  }

  return (
    <form onSubmit={handleSubmit(onSubmit)} className="space-y-4">
      <ActiveSessions authStorage={authStorage} />
      <FieldGroup>
        <Field>
          <FieldLabel htmlFor="name">Full Name</FieldLabel>
          <Input
            id="name"
            placeholder="John Doe"
            {...register('name')}
            aria-invalid={!!errors.name}
          />
          <FieldError errors={[errors.name]} />
        </Field>

        <Field>
          <FieldLabel htmlFor="email">Email</FieldLabel>
          <Input
            id="email"
            type="email"
            placeholder="m@example.com"
            {...register('email')}
            aria-invalid={!!errors.email}
          />
          <FieldError errors={[errors.email]} />
        </Field>

        <Field>
          <FieldLabel htmlFor="password">Password</FieldLabel>
          <Input
            id="password"
            type="password"
            {...register('password')}
            aria-invalid={!!errors.password}
          />
          <FieldError errors={[errors.password]} />
        </Field>

        <Field>
          <FieldLabel htmlFor="confirmPassword">Confirm Password</FieldLabel>
          <Input
            id="confirmPassword"
            type="password"
            {...register('confirmPassword')}
            aria-invalid={!!errors.confirmPassword}
          />
          <FieldError errors={[errors.confirmPassword]} />
        </Field>

        {(signUpMutation.error || errors.root?.serverError) && (
          <Alert variant="destructive">
            <HugeiconsIcon icon={AlertCircle} className="h-4 w-4" />
            <AlertDescription>
              {signUpMutation.error?.message ||
                errors.root?.serverError?.message}
            </AlertDescription>
          </Alert>
        )}

        <Button
          type="submit"
          className="w-full gap-2"
          disabled={isSubmitting || signUpMutation.isPending}
        >
          {(isSubmitting || signUpMutation.isPending) && <Spinner />}
          Sign Up
        </Button>
      </FieldGroup>
    </form>
  )
}
