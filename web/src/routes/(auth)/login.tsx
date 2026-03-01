import { Link, createFileRoute } from '@tanstack/react-router'
import { AuthLayout } from '@/features/auth/components/auth-layout'
import { LoginForm } from '@/features/auth/components/login-form'
import { getAuthStorageServer } from '@/lib/auth/session'

export const Route = createFileRoute('/(auth)/login')({
  component: Login,
  loader: async () => {
    try {
      const storage = await getAuthStorageServer()
      return { authStorage: storage }
    } catch (e) {
      console.error('Failed to fetch auth storage:', e)
      return { authStorage: null }
    }
  },
})

function Login() {
  const data = Route.useLoaderData()
  const authStorage = data.authStorage ?? null

  return (
    <AuthLayout
      title="Welcome back"
      description="Sign in to your account to continue"
    >
      <LoginForm authStorage={authStorage} />
      <div className="mt-5 text-center text-sm text-muted-foreground">
        Don&apos;t have an account?{' '}
        <Link
          to="/sign-up"
          className="font-medium text-foreground hover:underline underline-offset-4 transition-colors"
        >
          Sign up
        </Link>
      </div>
    </AuthLayout>
  )
}
