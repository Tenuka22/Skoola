import { useMutation } from '@tanstack/react-query'
import { loginFn } from '@/lib/auth/actions'

type LoginInput = {
  data: {
    email: string
    password: string
  }
}

type AuthActionResult = {
  success: boolean
  error?: string
}

export const useLogin = () => {
  return useMutation<AuthActionResult, Error, LoginInput>({
    mutationFn: (payload) => loginFn(payload),
  })
}
