import { useMutation, useQueryClient } from '@tanstack/react-query'
import {
  createSyllabusTopicMutation,
  deleteSyllabusTopicMutation,
  getSyllabusTopicsForStandardOptions,
  updateSyllabusTopicMutation,
} from '@/lib/api/@tanstack/react-query.gen'
import { authClient } from '@/lib/clients'

export const getSyllabusTopicsForStandardQueryOptions = (standardId: string) =>
  getSyllabusTopicsForStandardOptions({
    client: authClient,
    path: { standard_id: standardId },
  })

export const useCreateSyllabusTopic = () => {
  const queryClient = useQueryClient()
  return useMutation({
    ...createSyllabusTopicMutation({ client: authClient }),
    onSuccess: () => {
      queryClient.invalidateQueries({
        queryKey: ['getSyllabusTopicsForStandard'],
      })
    },
  })
}

export const useUpdateSyllabusTopic = () => {
  const queryClient = useQueryClient()
  return useMutation({
    ...updateSyllabusTopicMutation({ client: authClient }),
    onSuccess: () => {
      queryClient.invalidateQueries({
        queryKey: ['getSyllabusTopicsForStandard'],
      })
      queryClient.invalidateQueries({ queryKey: ['getSyllabusTopicById'] })
    },
  })
}

export const useDeleteSyllabusTopic = () => {
  const queryClient = useQueryClient()
  return useMutation({
    ...deleteSyllabusTopicMutation({ client: authClient }),
    onSuccess: () => {
      queryClient.invalidateQueries({
        queryKey: ['getSyllabusTopicsForStandard'],
      })
    },
  })
}
