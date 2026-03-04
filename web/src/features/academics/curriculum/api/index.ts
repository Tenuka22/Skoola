import { useMutation, useQueryClient } from '@tanstack/react-query'
import {
  createCurriculumStandardMutation,
  deleteCurriculumStandardMutation,
  getAllCurriculumStandardsOptions,
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
      queryClient.invalidateQueries({ queryKey: ['getAllCurriculumStandards'] })
    },
  })
}

export const useUpdateCurriculumStandard = () => {
  const queryClient = useQueryClient()
  return useMutation({
    ...updateCurriculumStandardMutation({ client: authClient }),
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: ['getAllCurriculumStandards'] })
    },
  })
}

export const useDeleteCurriculumStandard = () => {
  const queryClient = useQueryClient()
  return useMutation({
    ...deleteCurriculumStandardMutation({ client: authClient }),
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: ['getAllCurriculumStandards'] })
    },
  })
}
