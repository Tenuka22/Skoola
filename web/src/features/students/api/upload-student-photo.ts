import { useMutation, useQueryClient } from '@tanstack/react-query'
import { toast } from 'sonner'
import type { UploadStudentPhotoData } from '@/lib/api/types.gen'
import type { Options } from '@/lib/api/sdk.gen'
import {
  getAllStudentsQueryKey,
  uploadStudentPhotoMutation,
} from '@/lib/api/@tanstack/react-query.gen'
import { authClient } from '@/lib/clients'

export const useUploadStudentPhoto = (
  options?: Partial<Options<UploadStudentPhotoData>>,
) => {
  const queryClient = useQueryClient()
  return useMutation({
    ...uploadStudentPhotoMutation({ client: authClient, ...options }),
    onSuccess: () => {
      toast.success('Student photo uploaded successfully')
      queryClient.invalidateQueries({ queryKey: getAllStudentsQueryKey() })
    },
    onError: (error) => {
      toast.error(error.message || 'Failed to upload photo')
    },
  })
}
