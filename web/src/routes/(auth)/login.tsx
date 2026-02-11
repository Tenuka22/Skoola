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
      description="Enter your email to sign in to your account"
    >
      <LoginForm authStorage={authStorage} />
      <div className="mt-4 text-center text-sm">
        Don&apos;t have an account?{' '}
        <Link to="/sign-up" className="underline">
          Sign up
        </Link>
      </div>
    </AuthLayout>
  )
}
