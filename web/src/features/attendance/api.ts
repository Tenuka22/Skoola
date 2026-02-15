import { useMutation, useQuery, useQueryClient } from '@tanstack/react-query'
import { toast } from 'sonner'
import { authClient } from '@/lib/clients'

import {
  bulkMarkStudentAttendanceMutation,
  generateAttendanceReportOptions,
  getAllClassesOptions,
  getAllStaffOptions,
  getAttendanceByClassAndDateOptions,
  getAttendanceByClassAndDateQueryKey,
  getStaffAttendanceByDateOptions,
  getStaffAttendanceByDateQueryKey,
  markBulkStaffAttendanceMutation,
  updateStaffAttendanceMutation,
  updateStudentAttendanceMutation,
} from '@/lib/api/@tanstack/react-query.gen'

export const useStaffAttendance = (date: string) => {
  return useQuery(
    getStaffAttendanceByDateOptions({
      client: authClient,
      query: { date },
    }),
  )
}

export const useStaffList = (
  params: { page?: number; limit?: number; search?: string } = {},
) => {
  return useQuery(
    getAllStaffOptions({
      client: authClient,
      query: {
        page: params.page ?? 1,
        limit: params.limit ?? 100, // Fetch more for attendance marking
        search: params.search,
      },
    }),
  )
}

export const useMarkStaffAttendanceBulk = () => {
  const queryClient = useQueryClient()
  return useMutation({
    ...markBulkStaffAttendanceMutation({
      client: authClient,
    }),
    onSuccess: () => {
      toast.success('Attendance marked successfully')
      queryClient.invalidateQueries({
        queryKey: getStaffAttendanceByDateQueryKey({ query: { date: '' } }),
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
    ...updateStaffAttendanceMutation({
      client: authClient,
    }),
    onSuccess: () => {
      toast.success('Attendance updated successfully')
      queryClient.invalidateQueries({
        queryKey: getStaffAttendanceByDateQueryKey({ query: { date: '' } }),
      })
    },
    onError: (error) => {
      toast.error('Failed to update attendance')
      console.error(error)
    },
  })
}

export const useClasses = () => {
  return useQuery(
    getAllClassesOptions({
      client: authClient,
    }),
  )
}

export const useStudentsInClass = (classId: string, date: string) => {
  return useQuery(
    getAttendanceByClassAndDateOptions({
      client: authClient,
      path: { class_id: classId, date },
    }),
  )
}

export const useStudentAttendance = (classId: string, date: string) => {
  return useQuery(
    getAttendanceByClassAndDateOptions({
      client: authClient,
      path: { class_id: classId, date },
    }),
  )
}

export const useMarkStudentAttendanceBulk = () => {
  const queryClient = useQueryClient()
  return useMutation({
    ...bulkMarkStudentAttendanceMutation({
      client: authClient,
    }),
    onSuccess: () => {
      toast.success('Attendance marked successfully')
      queryClient.invalidateQueries({
        queryKey: getAttendanceByClassAndDateQueryKey({
          path: { class_id: '', date: '' },
        }),
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
    ...updateStudentAttendanceMutation({
      client: authClient,
    }),
    onSuccess: () => {
      toast.success('Attendance updated successfully')
      queryClient.invalidateQueries({
        queryKey: getAttendanceByClassAndDateQueryKey({
          path: { class_id: '', date: '' },
        }),
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
    ...generateAttendanceReportOptions({
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
