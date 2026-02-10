import * as React from 'react'
import { useForm } from 'react-hook-form'
import { zodResolver } from '@hookform/resolvers/zod'
import { HugeiconsIcon } from '@hugeicons/react'
import {
  CodeIcon,
  LanguageCircleIcon,
  Loading03Icon,
} from '@hugeicons/core-free-icons'
import { loginSchema } from '../schemas'
import type { LoginFormValues } from '../schemas'
import { getAuthStorageServer } from '@/lib/auth/session'
import { Button, buttonVariants } from '@/components/ui/button'
import { Input } from '@/components/ui/input'
import {
  Field,
  FieldError,
  FieldGroup,
  FieldLabel,
} from '@/components/ui/field'
import { Alert, AlertDescription } from '@/components/ui/alert'
import { env } from '@/lib/env'
import { cn } from '@/lib/utils'
import {
  Avatar,
  AvatarFallback,
  AvatarGroup,
  AvatarImage,
} from '@/components/ui/avatar'
import type { AuthStorage } from '@/lib/auth/session'
import { useMutation } from '@tanstack/react-query'
import { loginFn } from '@/lib/auth/actions'
import { useNavigate } from '@tanstack/react-router'

export function LoginForm() {
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
  } = useForm<LoginFormValues>({
    resolver: zodResolver(loginSchema),
  })

  const loginMutation = useMutation({
    mutationFn: loginFn,
  })

  const navigate = useNavigate()

  const onSubmit = async (data: LoginFormValues) => {
    try {
      await loginMutation.mutateAsync({ data })

      if (loginMutation.data?.success) {
        navigate({ from: '/profile' })
      } else if (loginMutation.data?.error) {
        setFormError('root.serverError', {
          type: 'server',
          message: loginMutation.data.error,
        })
      }
    } catch (err: unknown) {
      console.error('Login error in component:', err)
      setFormError('root.serverError', {
        type: 'server',
        message:
          (err instanceof Error && err.message) ||
          'Login failed. Please try again.',
      })
    }
  }

  return (
    <form onSubmit={handleSubmit(onSubmit)} className="space-y-4">
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
              .map(([key, value]) => (
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

        {(loginMutation.error || errors.root?.serverError) && (
          <Alert variant="destructive">
            <AlertDescription>
              {loginMutation.error?.message ||
                errors.root?.serverError?.message}
            </AlertDescription>
          </Alert>
        )}

        <Button
          type="submit"
          className="w-full"
          disabled={isSubmitting || loginMutation.isPending}
        >
          {(isSubmitting || loginMutation.isPending) && (
            <HugeiconsIcon
              icon={Loading03Icon}
              className="mr-2 h-4 w-4 animate-spin"
            />
          )}
          Sign In
        </Button>

        <div className="relative">
          <div className="absolute inset-0 flex items-center">
            <span className="w-full border-t" />
          </div>
          <div className="relative flex justify-center text-xs uppercase">
            <span className="px-2 text-muted-foreground">Or continue with</span>
          </div>
        </div>

        <div className="flex flex-col gap-2">
          <a
            href={`${env.VITE_API_URL}/auth/google/login`}
            className={cn(buttonVariants({ variant: 'outline' }), 'w-full')}
          >
            <HugeiconsIcon icon={LanguageCircleIcon} className="mr-2 h-4 w-4" />
            Google
          </a>
          <a
            href={`${env.VITE_API_URL}/auth/github/login`}
            className={cn(buttonVariants({ variant: 'outline' }), 'w-full')}
          >
            <HugeiconsIcon icon={CodeIcon} className="mr-2 h-4 w-4" />
            GitHub
          </a>
        </div>
      </FieldGroup>
    </form>
  )
}
