import { useMutation } from '@tanstack/react-query'
import { loginFn } from '@/lib/auth/actions'

export const useLogin = () => {
  return useMutation({
    mutationFn: loginFn,
  })
}
