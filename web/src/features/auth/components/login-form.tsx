import * as React from 'react'
import { useForm } from 'react-hook-form'
import { zodResolver } from '@hookform/resolvers/zod'
import { useNavigate } from '@tanstack/react-router'
import { HugeiconsIcon } from '@hugeicons/react'
import {
  Loading03Icon,
  CodeIcon,
  LanguageCircleIcon,
} from '@hugeicons/core-free-icons'
import { loginSchema } from '../schemas'
import type { LoginFormValues } from '../schemas'
import {
  getProfileC838C8E7Da73Bfc08645A117E4Df91F3 as getProfileApi,
  postAuthLogin9E9Be264D609C0E1A535693Ba4C389Aa as loginApi,
} from '@/lib/api/sdk.gen'
import { publicClient } from '@/lib/clients'
import { addSession } from '@/lib/auth/session'
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

export function LoginForm() {
  const navigate = useNavigate()
  const [error, setError] = React.useState<string | null>(null)

  const {
    register,
    handleSubmit,
    formState: { errors, isSubmitting },
  } = useForm<LoginFormValues>({
    resolver: zodResolver(loginSchema),
  })

  const onSubmit = async (data: LoginFormValues) => {
    setError(null)
    try {
      // 1. Login to get token
      const response = await loginApi({
        client: publicClient,
        body: data,
      })

      if (response.data && response.data.token) {
        const tempToken = response.data.token


        const profileResponse = await getProfileApi({
          client: publicClient, // Use public client but override headers
          headers: {
            Authorization: `Bearer ${tempToken}`,
          },
        })

        if (profileResponse.data) {
          // 3. Store Session
          addSession({
            token: response.data.token,
            refreshToken: response.data.refresh_token,
            user:
              profileResponse.data,
          })

          // 4. Redirect
          await navigate({ to: '/profile' })
        } else {
          setError('Failed to fetch user profile.')
        }
      }
    } catch (err: any) {
      console.error(err)
      setError(err?.message || 'Invalid email or password.')
    }
  }

  return (
    <form onSubmit={handleSubmit(onSubmit)} className="space-y-4">
      {error && (
        <Alert variant="destructive">
          <AlertDescription>{error}</AlertDescription>
        </Alert>
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

        <Button type="submit" className="w-full" disabled={isSubmitting}>
          {isSubmitting && (
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
            <span className="bg-background px-2 text-muted-foreground">
              Or continue with
            </span>
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
