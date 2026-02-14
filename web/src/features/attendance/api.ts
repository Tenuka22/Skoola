import { useMutation, useQuery, useQueryClient } from '@tanstack/react-query'
import { toast } from 'sonner'
import { authClient } from '@/lib/clients'
import {
  getClasses7A8C467E0Ba0893E8F4F0Bc9A21037BbOptions,
  getStaffAttendanceDateD0Fe5B3F1730787C38A30326Ac928B80Options,
  getStaffDb2Ddf96Bd86Cfcd0342B203Ba78A857Options,
  getStudentAttendanceClass2Fec35217B2F2C3727031Ce26765D12dOptions,
  getStudentAttendanceReport7382Fd100A69D43Ad28Ae81434Ab938dOptions,
  postStaffAttendanceBulk8F2A2Bc0B290E669419582F4B20549F7Mutation,
  postStudentAttendanceBulkEe86115B6Fcc8B311828E782275Ec9F4Mutation,
  putStaffAttendanceDb2F8533D2Be67Cf8725Bfeb7Eb137BbMutation,
  putStudentAttendance8Ee593Dc2Eb175A5E213A7Cb2A5Fa69bMutation,
} from '@/lib/api/@tanstack/react-query.gen'

export const useStaffAttendance = (date: string) => {
  return useQuery({
    ...getStaffAttendanceDateD0Fe5B3F1730787C38A30326Ac928B80Options({
      client: authClient,
      query: { date },
    }),
  })
}

export const useStaffList = (
  params: { page?: number; limit?: number; search?: string } = {},
) => {
  return useQuery({
    ...getStaffDb2Ddf96Bd86Cfcd0342B203Ba78A857Options({
      client: authClient,
      query: {
        page: params.page ?? 1,
        limit: params.limit ?? 100, // Fetch more for attendance marking
        search: params.search,
      },
    }),
  })
}

export const useMarkStaffAttendanceBulk = () => {
  const queryClient = useQueryClient()
  return useMutation({
    ...postStaffAttendanceBulk8F2A2Bc0B290E669419582F4B20549F7Mutation({
      client: authClient,
    }),
    onSuccess: () => {
      toast.success('Attendance marked successfully')
      queryClient.invalidateQueries({
        queryKey: [
          { _id: 'getStaffAttendanceDateD0Fe5B3F1730787C38A30326Ac928B80' },
        ],
      })
    },
    onError: (error) => {
      toast.error('Failed to mark attendance')
      console.error(error)
    },
  })
}

export const useUpdateStaffAttendance = () => {
  const queryClient = useQueryClient()
  return useMutation({
    ...putStaffAttendanceDb2F8533D2Be67Cf8725Bfeb7Eb137BbMutation({
      client: authClient,
    }),
    onSuccess: () => {
      toast.success('Attendance updated successfully')
      queryClient.invalidateQueries({
        queryKey: [
          { _id: 'getStaffAttendanceDateD0Fe5B3F1730787C38A30326Ac928B80' },
        ],
      })
    },
    onError: (error) => {
      toast.error('Failed to update attendance')
      console.error(error)
    },
  })
}

export const useClasses = () => {
  return useQuery({
    ...getClasses7A8C467E0Ba0893E8F4F0Bc9A21037BbOptions({
      client: authClient,
    }),
  })
}

export const useStudentsInClass = (classId: string, date: string) => {
  return useQuery({
    ...getStudentAttendanceClass2Fec35217B2F2C3727031Ce26765D12dOptions({
      client: authClient,
      path: { class_id: classId, date },
    }),
    enabled: !!classId && !!date,
  })
}

export const useStudentAttendance = (classId: string, date: string) => {
  return useQuery({
    ...getStudentAttendanceClass2Fec35217B2F2C3727031Ce26765D12dOptions({
      client: authClient,
      path: { class_id: classId, date },
    }),
    enabled: !!classId && !!date,
  })
}

export const useMarkStudentAttendanceBulk = () => {
  const queryClient = useQueryClient()
  return useMutation({
    ...postStudentAttendanceBulkEe86115B6Fcc8B311828E782275Ec9F4Mutation({
      client: authClient,
    }),
    onSuccess: () => {
      toast.success('Attendance marked successfully')
      queryClient.invalidateQueries({
        queryKey: [
          { _id: 'getStudentAttendanceClass2Fec35217B2F2C3727031Ce26765D12D' },
        ],
      })
    },
    onError: (error) => {
      toast.error('Failed to mark attendance')
      console.error(error)
    },
  })
}

export const useUpdateStudentAttendance = () => {
  const queryClient = useQueryClient()
  return useMutation({
    ...putStudentAttendance8Ee593Dc2Eb175A5E213A7Cb2A5Fa69bMutation({
      client: authClient,
    }),
    onSuccess: () => {
      toast.success('Attendance updated successfully')
      queryClient.invalidateQueries({
        queryKey: [
          { _id: 'getStudentAttendanceClass2Fec35217B2F2C3727031Ce26765D12D' },
        ],
      })
    },
    onError: (error) => {
      toast.error('Failed to update attendance')
      console.error(error)
    },
  })
}

export const useGenerateStudentAttendanceReport = (
  classId: string,
  fromDate: string,
  toDate: string,
  enabled: boolean = false,
) => {
  return useQuery({
    ...getStudentAttendanceReport7382Fd100A69D43Ad28Ae81434Ab938dOptions({
      client: authClient,
      query: { class_id: classId, from_date: fromDate, to_date: toDate },
    }),
    enabled: enabled && !!classId && !!fromDate && !!toDate,
    select: (data) =>
      data.map((item) => ({
        ...item,
        // Assuming `student_id` is available and unique for student data
        id: item.student_id,
      })),
  })
}
