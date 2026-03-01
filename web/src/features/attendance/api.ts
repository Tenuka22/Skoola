import {
  useMutation,
  useQuery,
  useQueryClient,
  useSuspenseQuery,
} from '@tanstack/react-query'
import { toast } from 'sonner'
import type {
  DefaultError,
  UseMutationResult,
  UseQueryResult,
} from '@tanstack/react-query'
import type { Options } from '@/lib/api/sdk.gen'
import type {
  BulkMarkStudentAttendanceData,
  BulkMarkStudentAttendanceResponse,
  CalculateStudentAttendancePercentageData,
  GetAttendanceByClassAndDateResponse,
  GetAttendanceByStudentData,
  GetEnrichedStudentListResponse,
  GetStudentByIdData,
  IssueExitPassData,
  IssueExitPassResponse,
  MarkStaffAttendanceBulkData,
  MarkStaffAttendanceBulkResponse,
  PaginatedClassResponse,
  PaginatedStaffResponse,
  StaffAttendanceResponse,
  StudentAttendanceReportResponse,
  SubmitExcuseData,
  SubmitExcuseResponse,
  UpdateStaffAttendanceData,
  UpdateStaffAttendanceResponse,
  UpdateStudentAttendanceData,
  UpdateStudentAttendanceResponse as UpdateStudentAttendanceResponseApi,
} from '@/lib/api/types.gen'
import { authClient } from '@/lib/clients'

import {
  bulkMarkStudentAttendanceMutation,
  calculateStudentAttendancePercentageOptions,
  generateAttendanceReportOptions,
  getAllClassesOptions,
  getAllStaffOptions,
  getAttendanceByClassAndDateOptions,
  getAttendanceByStudentOptions,
  getEnrichedStudentListOptions,
  getStaffAttendanceByDateOptions,
  getStudentByIdOptions,
  issueExitPassMutation,
  markStaffAttendanceBulkMutation,
  submitExcuseMutation,
  updateStaffAttendanceMutation,
  updateStudentAttendanceMutation,
} from '@/lib/api/@tanstack/react-query.gen'

export const useStaffAttendance = (
  date: string,
): UseQueryResult<Array<StaffAttendanceResponse>, DefaultError> => {
  return useQuery({
    ...getStaffAttendanceByDateOptions({
      client: authClient,
      query: { date },
    }),
  })
}

export const useStaffList = (
  params: {
    page?: number
    limit?: number
    search?: string
    staff_type?: 'Teaching' | 'NonTeaching' | 'Administrative'
  } = {},
): UseQueryResult<PaginatedStaffResponse, DefaultError> => {
  return useQuery({
    ...getAllStaffOptions({
      client: authClient,
      query: {
        page: params.page ?? 1,
        limit: params.limit ?? 100, // Fetch more for attendance marking
        search: params.search,
        staff_type: params.staff_type,
      },
    }),
  })
}

export const useMarkStaffAttendanceBulk = (): UseMutationResult<
  MarkStaffAttendanceBulkResponse,
  DefaultError,
  Options<MarkStaffAttendanceBulkData>
> => {
  const queryClient = useQueryClient()
  return useMutation({
    ...markStaffAttendanceBulkMutation({
      client: authClient,
    }),
    onSuccess: () => {
      toast.success('Attendance marked successfully')
      queryClient.invalidateQueries({ queryKey: ['staff-attendance'] })
    },
    onError: (error) => {
      toast.error('Failed to mark attendance')
      console.error(error)
    },
  })
}

export const useUpdateStaffAttendance = (): UseMutationResult<
  UpdateStaffAttendanceResponse,
  DefaultError,
  Options<UpdateStaffAttendanceData>
> => {
  const queryClient = useQueryClient()
  return useMutation({
    ...updateStaffAttendanceMutation({
      client: authClient,
    }),
    onSuccess: () => {
      toast.success('Attendance updated successfully')
      queryClient.invalidateQueries({ queryKey: ['staff-attendance'] })
    },
    onError: (error) => {
      toast.error('Failed to update attendance')
      console.error(error)
    },
  })
}

export const useClasses = (): UseQueryResult<
  PaginatedClassResponse,
  DefaultError
> => {
  return useQuery({
    ...getAllClassesOptions({
      client: authClient,
    }),
  })
}

export const useSuspenseClasses = () => {
  return useSuspenseQuery({
    ...getAllClassesOptions({
      client: authClient,
    }),
  })
}

export const useStudentsInClass = (
  classId: string,
  date: string,
): UseQueryResult<GetAttendanceByClassAndDateResponse, DefaultError> => {
  return useQuery({
    ...getAttendanceByClassAndDateOptions({
      client: authClient,
      path: { class_id: classId, date },
    }),
  })
}

export const useStudentAttendance = (
  classId: string,
  date: string,
): UseQueryResult<GetAttendanceByClassAndDateResponse, DefaultError> => {
  return useQuery({
    ...getAttendanceByClassAndDateOptions({
      client: authClient,
      path: { class_id: classId, date },
    }),
  })
}

export const useEnrichedStudentAttendance = (
  classId: string,
  date: string,
): UseQueryResult<GetEnrichedStudentListResponse, DefaultError> => {
  return useQuery({
    ...getEnrichedStudentListOptions({
      client: authClient,
      path: { class_id: classId, date: date },
    }),
  })
}

export const useMarkStudentAttendanceBulk = (): UseMutationResult<
  BulkMarkStudentAttendanceResponse,
  DefaultError,
  Options<BulkMarkStudentAttendanceData>
> => {
  const queryClient = useQueryClient()
  return useMutation({
    ...bulkMarkStudentAttendanceMutation({
      client: authClient,
    }),
    onSuccess: () => {
      toast.success('Attendance marked successfully')
      queryClient.invalidateQueries({ queryKey: ['student-attendance'] })
    },
    onError: (error) => {
      toast.error('Failed to mark attendance')
      console.error(error)
    },
  })
}

export const useUpdateStudentAttendance = (): UseMutationResult<
  UpdateStudentAttendanceResponseApi,
  DefaultError,
  Options<UpdateStudentAttendanceData>
> => {
  const queryClient = useQueryClient()
  return useMutation({
    ...updateStudentAttendanceMutation({
      client: authClient,
    }),
    onSuccess: () => {
      toast.success('Attendance updated successfully')
      queryClient.invalidateQueries({ queryKey: ['student-attendance'] })
    },
    onError: (error) => {
      toast.error('Failed to update attendance')
      console.error(error)
    },
  })
}

export const useIssueExitPass = (): UseMutationResult<
  IssueExitPassResponse,
  DefaultError,
  Options<IssueExitPassData>
> => {
  const queryClient = useQueryClient()
  return useMutation({
    ...issueExitPassMutation({ client: authClient }),
    onSuccess: () => {
      toast.success('Exit pass issued successfully')
      queryClient.invalidateQueries({ queryKey: ['student-attendance'] })
    },
    onError: (error) => {
      toast.error('Failed to issue exit pass')
      console.error(error)
    },
  })
}

export const useSubmitExcuse = (): UseMutationResult<
  SubmitExcuseResponse,
  DefaultError,
  Options<SubmitExcuseData>
> => {
  const queryClient = useQueryClient()
  return useMutation({
    ...submitExcuseMutation({ client: authClient }),
    onSuccess: () => {
      toast.success('Excuse submitted successfully')
      queryClient.invalidateQueries({ queryKey: ['student-attendance'] })
    },
    onError: (error) => {
      toast.error('Failed to submit excuse')
      console.error(error)
    },
  })
}

export const useGenerateStudentAttendanceReport = (
  classId: string,
  fromDate: string,
  toDate: string,
  enabled: boolean = false,
): UseQueryResult<Array<StudentAttendanceReportResponse>, DefaultError> => {
  return useQuery({
    ...generateAttendanceReportOptions({
      client: authClient,
      query: { class_id: classId, from_date: fromDate, to_date: toDate },
    }),
    enabled: enabled && !!classId && !!fromDate && !!toDate,
    select: (data: Array<StudentAttendanceReportResponse>) =>
      data.map((item) => ({
        ...item,
        // Assuming `student_id` is available and unique for student data
        id: item.student_id,
      })),
  })
}

export const useSuspenseGenerateStudentAttendanceReport = (
  classId: string,
  fromDate: string,
  toDate: string,
) => {
  return useSuspenseQuery({
    ...generateAttendanceReportOptions({
      client: authClient,
      query: { class_id: classId, from_date: fromDate, to_date: toDate },
    }),
    select: (data: Array<StudentAttendanceReportResponse>) =>
      data.map((item) => ({
        ...item,
        // Assuming `student_id` is available and unique for student data
        id: item.student_id,
      })),
  })
}

export const useAttendanceByStudent = (
  options: Options<GetAttendanceByStudentData>,
) => {
  return useQuery({
    ...getAttendanceByStudentOptions({
      client: authClient,
      ...options,
    }),
  })
}

export const useCalculateStudentAttendancePercentage = (
  options: Options<CalculateStudentAttendancePercentageData>,
) => {
  return useQuery({
    ...calculateStudentAttendancePercentageOptions({
      client: authClient,
      ...options,
    }),
  })
}

export const useStudentById = (options: Options<GetStudentByIdData>) => {
  return useQuery({
    ...getStudentByIdOptions({
      client: authClient,
      ...options,
    }),
  })
}

export const useSuspenseAttendanceByStudent = (
  options: Options<GetAttendanceByStudentData>,
) => {
  return useSuspenseQuery({
    ...getAttendanceByStudentOptions({
      client: authClient,
      ...options,
    }),
  })
}

export const useSuspenseCalculateStudentAttendancePercentage = (
  options: Options<CalculateStudentAttendancePercentageData>,
) => {
  return useSuspenseQuery({
    ...calculateStudentAttendancePercentageOptions({
      client: authClient,
      ...options,
    }),
  })
}

export const useSuspenseStudentById = (
  options: Options<GetStudentByIdData>,
) => {
  return useSuspenseQuery({
    ...getStudentByIdOptions({
      client: authClient,
      ...options,
    }),
  })
}
