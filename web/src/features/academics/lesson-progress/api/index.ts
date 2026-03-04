import { useMutation, useQueryClient } from '@tanstack/react-query'
import {
  getLessonProgressOptions,
  recordLessonProgressMutation,
} from '@/lib/api/@tanstack/react-query.gen'
import { authClient } from '@/lib/clients'

export const getLessonProgressQueryOptions = (
  classId: string,
  subjectId: string,
) =>
  getLessonProgressOptions({
    client: authClient,
    path: { class_id: classId, subject_id: subjectId },
  })

export const useRecordLessonProgress = () => {
  const queryClient = useQueryClient()
  return useMutation({
    ...recordLessonProgressMutation({ client: authClient }),
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: ['getLessonProgress'] })
    },
  })
}
