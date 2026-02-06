import { Link, createFileRoute } from '@tanstack/react-router'
import { AuthLayout } from '@/features/auth/components/auth-layout'
import { LoginForm } from '@/features/auth/components/login-form'

export const Route = createFileRoute('/(auth)/login')({
  component: Login,
})

function Login() {
  return (
    <AuthLayout
      title="Welcome back"
      description="Enter your email to sign in to your account"
    >
      <LoginForm />
      <div className="mt-4 text-center text-sm">
        Don&apos;t have an account?{' '}
        <Link to="/sign-up" className="underline">
          Sign up
        </Link>
      </div>
    </AuthLayout>
  )
}
