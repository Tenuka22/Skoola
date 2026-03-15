import { useMutation } from '@tanstack/react-query'
import { signUpFn } from '@/lib/auth/actions'

type SignUpInput = {
  data: {
    name: string
    email: string
    password: string
  }
}

type AuthActionResult = {
  success: boolean
  error?: string
}

export const useSignUp = () => {
  return useMutation<AuthActionResult, Error, SignUpInput>({
    mutationFn: (payload) => signUpFn(payload),
  })
}
