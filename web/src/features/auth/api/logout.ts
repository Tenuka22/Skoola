import { useMutation } from '@tanstack/react-query'
import { logoutFn } from '@/lib/auth/actions'

export const useLogout = () => {
  return useMutation({
    mutationFn: async () => {
      await logoutFn()
      window.location.reload()
    },
  })
}
