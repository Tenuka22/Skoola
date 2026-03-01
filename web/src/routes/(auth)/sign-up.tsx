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
      description="Get started with your free account"
    >
      <SignUpForm authStorage={authStorage} />
      <div className="mt-5 text-center text-sm text-muted-foreground">
        Already have an account?{' '}
        <Link
          to="/login"
          className="font-medium text-foreground hover:underline underline-offset-4 transition-colors"
        >
          Sign in
        </Link>
      </div>
    </AuthLayout>
  )
}
