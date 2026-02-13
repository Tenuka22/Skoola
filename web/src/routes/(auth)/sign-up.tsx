import { Link, createFileRoute } from '@tanstack/react-router'
import { AuthLayout } from '@/features/auth/components/auth-layout'
import { SignUpForm } from '@/features/auth/components/sign-up-form'
import { getAuthStorageServer } from '@/lib/auth/session'

export const Route = createFileRoute('/(auth)/sign-up')({
  component: SignUp,
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

function SignUp() {
  const data = Route.useLoaderData()
  const authStorage = data.authStorage ?? null

  return (
    <AuthLayout
      title="Create an account"
      description="Enter your information to create an account"
    >
      <SignUpForm authStorage={authStorage} />
      <div className="mt-4 text-center text-sm">
        Already have an account?{' '}
        <Link to="/login" className="underline">
          Sign in
        </Link>
      </div>
    </AuthLayout>
  )
}
