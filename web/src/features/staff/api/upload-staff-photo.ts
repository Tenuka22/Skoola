import { useMutation, useQueryClient } from '@tanstack/react-query'
import { toast } from 'sonner'
import type { UploadStaffPhotoData } from '@/lib/api/types.gen'
import type { Options } from '@/lib/api/sdk.gen'
import {
  getAllStaffQueryKey,
  uploadStaffPhotoMutation,
} from '@/lib/api/@tanstack/react-query.gen'
import { authClient } from '@/lib/clients'

export const useUploadStaffPhoto = (
  options?: Partial<Options<UploadStaffPhotoData>>,
) => {
  const queryClient = useQueryClient()
  return useMutation({
    ...uploadStaffPhotoMutation({ client: authClient, ...options }),
    onSuccess: () => {
      toast.success('Staff photo uploaded successfully')
      queryClient.invalidateQueries({ queryKey: getAllStaffQueryKey() })
    },
    onError: (error) => {
      toast.error(error.message || 'Failed to upload photo')
    },
  })
}
