import * as React from 'react'
import { useForm } from 'react-hook-form'
import { zodResolver } from '@hookform/resolvers/zod'
import { useNavigate } from '@tanstack/react-router'
import { HugeiconsIcon } from '@hugeicons/react'
import { Loading03Icon } from '@hugeicons/core-free-icons'
import { signUpSchema } from '../schemas'
import type { SignUpFormValues } from '../schemas';
import { getProfileC838C8E7Da73Bfc08645A117E4Df91F3 as getProfileApi, postAuthRegisterD7296Dbacc4Fd751Aeb142Bbb8A63Fd9 as registerApi } from '@/lib/api/sdk.gen'
import { publicClient } from '@/lib/clients'
import { addSession } from '@/lib/auth/session'
import { Button } from '@/components/ui/button'
import { Input } from '@/components/ui/input'
import {
  Field,
  FieldError,
  FieldGroup,
  FieldLabel,
} from '@/components/ui/field'
import { Alert, AlertDescription } from '@/components/ui/alert'

export function SignUpForm() {
  const navigate = useNavigate()
  const [error, setError] = React.useState<string | null>(null)

  const {
    register,
    handleSubmit,
    formState: { errors, isSubmitting },
  } = useForm<SignUpFormValues>({
    resolver: zodResolver(signUpSchema),
  })

  const onSubmit = async (data: SignUpFormValues) => {
    setError(null)
    try {
      // 1. Register
      const response: any = await registerApi({
        client: publicClient,
        body: {
          email: data.email,
          password: data.password,
          // 'name' might not be in the minimal register payload, checking API...
          // If the API only takes email/pass, we might need to update profile later.
          // But let's assume standard fields or strict adherence to API types.
          // The schemas.gen.ts showed RegisterRequest only has email/password.
          // So we ignore 'name' for now or handle it after login?
          // The user asked for "auth login and sign up".
          // I'll send email/pass.
        },
      })

      // 2. Check for token (Auto-login)
      if (response.data && response.data.token) {
        const tempToken = response.data.token

        // Fetch Profile
        const profileResponse = await getProfileApi({
          client: publicClient,
          headers: {
            Authorization: `Bearer ${tempToken}`,
          },
        })

        if (profileResponse.data) {
          addSession({
            token: response.data.token,
            refreshToken: response.data.refresh_token,
            user: profileResponse.data,
          })
          await navigate({ to: '/profile' })
          return
        }
      }

      // If no token, redirect to login
      await navigate({ to: '/login' })
    } catch (err: any) {
      console.error(err)
      setError(err?.message || 'Registration failed.')
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

        <Button type="submit" className="w-full" disabled={isSubmitting}>
          {isSubmitting && (
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
