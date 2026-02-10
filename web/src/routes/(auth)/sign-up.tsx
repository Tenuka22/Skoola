import { Link, createFileRoute } from '@tanstack/react-router'
import { AuthLayout } from '@/features/auth/components/auth-layout'
import { SignUpForm } from '@/features/auth/components/sign-up-form'

export const Route = createFileRoute('/(auth)/sign-up')({
  component: SignUp,
})

function SignUp() {
  return (
    <AuthLayout
      title="Create an account"
      description="Enter your information to create an account"
    >
      <SignUpForm />
      <div className="mt-4 text-center text-sm">
        Already have an account?{' '}
        <Link to="/login" className="underline">
          Sign in
        </Link>
      </div>
    </AuthLayout>
  )
}
