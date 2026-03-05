import { useMutation, useQueryClient } from '@tanstack/react-query'
import { toast } from 'sonner'
import {
  createCurriculumStandardMutation,
  deleteCurriculumStandardMutation,
  getAllCurriculumStandardsOptions,
  getAllCurriculumStandardsQueryKey,
  updateCurriculumStandardMutation,
} from '@/lib/api/@tanstack/react-query.gen'
import { authClient } from '@/lib/clients'

export const getAllCurriculumStandardsQueryOptions = () =>
  getAllCurriculumStandardsOptions({ client: authClient })

export const useCreateCurriculumStandard = () => {
  const queryClient = useQueryClient()
  return useMutation({
    ...createCurriculumStandardMutation({ client: authClient }),
    onSuccess: () => {
      queryClient.invalidateQueries({
        queryKey: getAllCurriculumStandardsQueryKey({ client: authClient }),
      })
      toast.success('Curriculum standard created successfully')
    },
    onError: (error) => {
      toast.error('Failed to create curriculum standard', {
        description: error instanceof Error ? error.message : 'Unknown error',
      })
    },
  })
}

export const useUpdateCurriculumStandard = () => {
  const queryClient = useQueryClient()
  return useMutation({
    ...updateCurriculumStandardMutation({ client: authClient }),
    onSuccess: () => {
      queryClient.invalidateQueries({
        queryKey: getAllCurriculumStandardsQueryKey({ client: authClient }),
      })
      toast.success('Curriculum standard updated successfully')
    },
    onError: (error) => {
      toast.error('Failed to update curriculum standard', {
        description: error instanceof Error ? error.message : 'Unknown error',
      })
    },
  })
}

export const useDeleteCurriculumStandard = () => {
  const queryClient = useQueryClient()
  return useMutation({
    ...deleteCurriculumStandardMutation({ client: authClient }),
    onSuccess: () => {
      queryClient.invalidateQueries({
        queryKey: getAllCurriculumStandardsQueryKey({ client: authClient }),
      })
      toast.success('Curriculum standard deleted successfully')
    },
    onError: (error) => {
      toast.error('Failed to delete curriculum standard', {
        description: error instanceof Error ? error.message : 'Unknown error',
      })
    },
  })
}
