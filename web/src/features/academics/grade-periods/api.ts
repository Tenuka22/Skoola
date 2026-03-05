import { useMutation, useQueryClient } from '@tanstack/react-query'
import { authClient } from '@/lib/clients'
import {
  createGradePeriodMutation,
  getGradePeriodsByGradeOptions,
  updateGradePeriodMutation,
  deleteGradePeriodMutation,
} from '@/lib/api/@tanstack/react-query.gen'

export const getGradePeriodsByGradeQueryOptions = (gradeId: string) =>
  getGradePeriodsByGradeOptions({
    client: authClient,
    path: { grade_id: gradeId },
  })

export const useCreateGradePeriod = () => {
  const queryClient = useQueryClient()
  return useMutation({
    ...createGradePeriodMutation({
      client: authClient,
    }),
    onSuccess: (_, variables) => {
      queryClient.invalidateQueries({
        queryKey: getGradePeriodsByGradeQueryOptions(variables.body.grade_id)
          .queryKey,
      })
    },
  })
}

export const useUpdateGradePeriod = (gradeId: string) => {
  const queryClient = useQueryClient()
  return useMutation({
    ...updateGradePeriodMutation({
      client: authClient,
    }),
    onSuccess: () => {
      queryClient.invalidateQueries({
        queryKey: getGradePeriodsByGradeQueryOptions(gradeId).queryKey,
      })
    },
  })
}

export const useDeleteGradePeriod = (gradeId: string) => {
  const queryClient = useQueryClient()
  return useMutation({
    ...deleteGradePeriodMutation({
      client: authClient,
    }),
    onSuccess: () => {
      queryClient.invalidateQueries({
        queryKey: getGradePeriodsByGradeQueryOptions(gradeId).queryKey,
      })
    },
  })
}
