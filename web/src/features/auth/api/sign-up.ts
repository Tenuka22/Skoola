import { useMutation } from '@tanstack/react-query'
import { signUpFn } from '@/lib/auth/actions'

export const useSignUp = () => {
  return useMutation({
    mutationFn: signUpFn,
  })
}
