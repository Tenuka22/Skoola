import * as React from 'react'
import { useForm } from 'react-hook-form'
import { zodResolver } from '@hookform/resolvers/zod'
import { HugeiconsIcon } from '@hugeicons/react'
import { Loading03Icon } from '@hugeicons/core-free-icons'
import { signUpSchema } from '../schemas'
import type { SignUpFormValues } from '../schemas'
import { getAuthStorageServer } from '@/lib/auth/session'
import { Button } from '@/components/ui/button'
import { Input } from '@/components/ui/input'
import {
  Field,
  FieldError,
  FieldGroup,
  FieldLabel,
} from '@/components/ui/field'
import { Alert, AlertDescription } from '@/components/ui/alert'
import {
  Avatar,
  AvatarFallback,
  AvatarGroup,
  AvatarImage,
} from '@/components/ui/avatar'
import type { AuthStorage, Session } from '@/lib/auth/session'
import { useMutation } from '@tanstack/react-query'
import { signUpFn } from '@/lib/auth/actions'
import { redirect } from 'nitro/h3'

export function SignUpForm() {
  const [users, setUsers] = React.useState<AuthStorage | null>(null)

  React.useEffect(() => {
    const fetchAuthStorage = async () => {
      try {
        const storage = await getAuthStorageServer()
        setUsers(storage)
      } catch (e) {
        console.error('Failed to fetch auth storage:', e)
        setUsers(null)
      }
    }
    fetchAuthStorage()
  }, [])

  const {
    register,
    handleSubmit,
    formState: { errors, isSubmitting },
    setError: setFormError,
  } = useForm<SignUpFormValues>({
    resolver: zodResolver(signUpSchema),
  })

  const { mutate, isPending, error } = useMutation({
    mutationFn: signUpFn,
  })

  const onSubmit = async (data: SignUpFormValues) => {
    try {
      mutate({
        data: {
          name: data.name,
          email: data.email,
          password: data.password,
        },
      })

      redirect('/login')
    } catch (err: any) {
      console.error('Sign Up error in component:', err)
      setFormError('root.serverError', {
        type: 'server',
        message: err.message || 'Sign up failed. Please try again.',
      })
    }
  }

  return (
    <form onSubmit={handleSubmit(onSubmit)} className="space-y-4">
      {(error || errors.root?.serverError) && (
        <Alert variant="destructive">
          <AlertDescription>
            {error?.message || errors.root?.serverError?.message}
          </AlertDescription>
        </Alert>
      )}

      {users?.sessions && (
        <div className="flex flex-col gap-2">
          <span className="text-muted-foreground">
            Already Logged in with...
          </span>

          <AvatarGroup>
            {Object.entries(users.sessions)
              .sort(([keyA], [keyB]) => {
                if (keyA === users.activeUserId) return -1
                if (keyB === users.activeUserId) return 1
                return 0
              })
              .map(([key, value]: [string, Session]) => (
                <div key={key}>
                  <Avatar>
                    <AvatarImage src={undefined} alt={value.user.email} />
                    <AvatarFallback
                      className={
                        key === users.activeUserId
                          ? 'bg-primary text-primary-foreground rounded-full'
                          : ''
                      }
                    >
                      {String(value.user.email).substring(0, 2)}
                    </AvatarFallback>
                  </Avatar>
                </div>
              ))}
          </AvatarGroup>
        </div>
      )}

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

        <Button
          type="submit"
          className="w-full"
          disabled={isSubmitting || isPending}
        >
          {(isSubmitting || isPending) && (
            <HugeiconsIcon
              icon={Loading03Icon}
              className="mr-2 h-4 w-4 animate-spin"
            />
          )}
          Sign Up
        </Button>
      </FieldGroup>
    </form>
  )
}
