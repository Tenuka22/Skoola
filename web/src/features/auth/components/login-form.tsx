import { HugeiconsIcon } from '@hugeicons/react'
import { AlertCircle } from '@hugeicons/core-free-icons'
import { useMutation } from '@tanstack/react-query'
import { useNavigate } from '@tanstack/react-router'
import { loginSchema } from '../schemas'
import { ActiveSessions } from './active-sessions'
import type { LoginFormValues } from '../schemas'
import type { AuthStorage } from '@/lib/auth/session'
import type { UseFormReturn } from 'react-hook-form'
import { Button } from '@/components/ui/button'
import { Alert, AlertDescription } from '@/components/ui/alert'
import { env } from '@/lib/env'
import { loginFn } from '@/lib/auth/actions'
import {
  FormBuilder,
  defineFormConfig,
  normalizeErrorMessage,
} from '@/components/form-builder'
import { Box, Grid, HStack, Stack, Text } from '@/components/primitives'

export function LoginForm({
  authStorage,
}: {
  authStorage: AuthStorage | null
}) {
  const loginMutation = useMutation({
    mutationFn: loginFn,
  })

  const navigate = useNavigate()

  const config = defineFormConfig(loginSchema, {
    structure: [
      [
        {
          field: 'email',
          type: 'input',
          label: 'Email Address',
          inputType: 'email',
          placeholder: 'm@example.com',
          description: 'We will never share your email with anyone else.',
        },
      ],
      [
        {
          field: 'password',
          type: 'input',
          label: 'Password',
          inputType: 'password',
          placeholder: '**********',
          description: 'Enter your secure password.',
        },
      ],
    ],
    extras: {
      top: <ActiveSessions authStorage={authStorage} />,
      afterFields: (form) =>
        loginMutation.error || form.formState.errors.root?.serverError ? (
          <Alert variant="destructive">
            <HugeiconsIcon icon={AlertCircle} className="h-4 w-4" />
            <AlertDescription>
              {normalizeErrorMessage(
                loginMutation.error?.message ||
                  form.formState.errors.root?.serverError?.message ||
                  'Login failed. Please try again.',
              )}
            </AlertDescription>
          </Alert>
        ) : null,
      bottom: (
        <Stack gap={4}>
          <Box className="relative">
            <Box className="absolute inset-0 flex items-center">
              <span className="w-full border-t" />
            </Box>
            <HStack align="center" className="relative justify-center">
              <Text size="xs" muted className="bg-background px-2 uppercase">
                Or continue with
              </Text>
            </HStack>
          </Box>

          <Grid cols={2} gap={2}>
            <a
              href={`${env.VITE_API_URL}/auth/google/login`}
              target="_blank"
              rel="noopener noreferrer"
            >
              <Button variant="outline" className="w-full">
                <svg className="mr-2 h-4 w-4" viewBox="0 0 24 24">
                  <path
                    fill="currentColor"
                    d="M22.56 12.25c0-.78-.07-1.53-.2-2.25H12v4.26h5.92c-.26 1.37-1.04 2.53-2.21 3.31v2.77h3.57c2.08-1.92 3.28-4.74 3.28-8.09z"
                  />
                  <path
                    fill="currentColor"
                    d="M12 23c2.97 0 5.46-.98 7.28-2.66l-3.57-2.77c-.98.66-2.23 1.06-3.71 1.06-2.86 0-5.29-1.93-6.16-4.53H2.18v2.84C3.99 20.53 7.7 23 12 23z"
                  />
                  <path
                    fill="currentColor"
                    d="M5.84 14.09c-.22-.66-.35-1.36-.35-2.09s.13-1.43.35-2.09V7.07H2.18C1.43 8.55 1 10.22 1 12s.43 3.45 1.18 4.93l2.85-2.22.81-.62z"
                  />
                  <path
                    fill="currentColor"
                    d="M12 5.38c1.62 0 3.06.56 4.21 1.64l3.15-3.15C17.45 2.09 14.97 1 12 1 7.7 1 3.99 3.47 2.18 7.07l3.66 2.84c.87-2.6 3.3-4.53 6.16-4.53z"
                  />
                </svg>
                Google
              </Button>
            </a>
            <a
              href={`${env.VITE_API_URL}/auth/github/login`}
              target="_blank"
              rel="noopener noreferrer"
            >
              <Button variant="outline" className="w-full">
                <svg className="mr-2 h-4 w-4" viewBox="0 0 24 24">
                  <path
                    fill="currentColor"
                    d="M12 2A10 10 0 0 0 2 12c0 4.42 2.87 8.17 6.84 9.5.5.08.66-.23.66-.5v-1.69c-2.77.6-3.36-1.34-3.36-1.34-.46-1.16-1.11-1.47-1.11-1.47-.91-.62.07-.6.07-.6 1 .07 1.53 1.03 1.53 1.03.87 1.52 2.34 1.07 2.91.83.09-.65.35-1.09.63-1.34-2.22-.25-4.55-1.11-4.55-4.92 0-1.11.38-2 1.03-2.71-.1-.25-.45-1.29.1-2.64 0 0 .84-.27 2.75 1.02.79-.22 1.65-.33 2.5-.33.85 0 1.71.11 2.5.33 1.91-1.29 2.75-1.02 2.75-1.02.55 1.35.2 2.39.1 2.64.65.71 1.03 1.6 1.03 2.71 0 3.82-2.34 4.66-4.57 4.91.36.31.69.92.69 1.85V21c0 .27.16.59.67.5C19.14 20.16 22 16.42 22 12A10 10 0 0 0 12 2z"
                  />
                </svg>
                GitHub
              </Button>
            </a>
          </Grid>
        </Stack>
      ),
    },
  })

  const onSubmit = async (
    data: LoginFormValues,
    form: UseFormReturn<LoginFormValues>,
  ) => {
    try {
      const result = await loginMutation.mutateAsync({ data })

      if (result?.success) {
        setTimeout(() => {
          navigate({ from: '/profile' })
        }, 2000)
        return
      }

      form.setError('root.serverError', {
        type: 'server',
        message: normalizeErrorMessage(
          result?.error || 'Login failed. Please try again.',
        ),
      })
    } catch (err: unknown) {
      console.error('Login error in component:', err)
      form.setError('root.serverError', {
        type: 'server',
        message:
          err instanceof Error
            ? normalizeErrorMessage(err.message)
            : 'Login failed due to an unknown error.',
      })
    }
  }

  return (
    <FormBuilder
      schema={loginSchema}
      config={config}
      onSubmit={onSubmit}
      isLoading={loginMutation.isPending}
      actions={[
        {
          type: 'submit',
          label: 'Sign In',
          className: 'w-full gap-2',
        },
      ]}
    />
  )
}
