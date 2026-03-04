import { useMutation, useQueryClient } from '@tanstack/react-query'
import {
  createExamMutation,
  createExamTypeMutation,
  deleteExamMutation,
  getAllExamTypesOptions,
  getAllExamsOptions,
  updateExamMutation,
} from '@/lib/api/@tanstack/react-query.gen'
import { authClient } from '@/lib/clients'

export const getAllExamsQueryOptions = () =>
  getAllExamsOptions({ client: authClient })

export const useCreateExam = () => {
  const queryClient = useQueryClient()
  return useMutation({
    ...createExamMutation({ client: authClient }),
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: ['getAllExams'] })
    },
  })
}

export const useUpdateExam = () => {
  const queryClient = useQueryClient()
  return useMutation({
    ...updateExamMutation({ client: authClient }),
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: ['getAllExams'] })
    },
  })
}

export const useDeleteExam = () => {
  const queryClient = useQueryClient()
  return useMutation({
    ...deleteExamMutation({ client: authClient }),
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: ['getAllExams'] })
    },
  })
}

export const getAllExamTypesQueryOptions = () =>
  getAllExamTypesOptions({ client: authClient })

export const useCreateExamType = () => {
  const queryClient = useQueryClient()
  return useMutation({
    ...createExamTypeMutation({ client: authClient }),
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: ['getAllExamTypes'] })
    },
  })
}
