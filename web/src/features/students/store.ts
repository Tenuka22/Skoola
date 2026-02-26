import { create } from 'zustand'
import type { SortingState } from '@tanstack/react-table'
import type { StudentResponse } from '@/features/students/types'
import type { StudentStatus } from '@/lib/api/types.gen'
import type { ListViewMode } from '@/lib/constants/view-modes'

export type ViewMode = ListViewMode

interface StudentsState {
  page: number
  search: string
  debouncedSearch: string
  statusFilter: StudentStatus | 'all'
  sorting: SortingState
  createdAfter: string | null
  createdBefore: string | null
  columnVisibility: Record<string, boolean>
  view: ViewMode
  studentToDelete: string | null
  isBulkDeleteOpen: boolean
  isBulkEditOpen: boolean
  isCreateStudentOpen: boolean
  studentToEdit: StudentResponse | null
  isUploadPhotoOpen: boolean
  studentToUploadPhotoFor: StudentResponse | null
  isAssignClassOpen: boolean
  studentToAssignClassFor: StudentResponse | null
  isGuardiansOpen: boolean
  studentToManageGuardiansFor: StudentResponse | null
  isAttendanceOpen: boolean
  studentToManageAttendanceFor: StudentResponse | null
  isMarksOpen: boolean
  studentToManageMarksFor: StudentResponse | null

  setPage: (page: number) => void
  setSearch: (search: string) => void
  setDebouncedSearch: (search: string) => void
  setStatusFilter: (filter: StudentStatus | 'all') => void
  setSorting: (
    sorting: SortingState | ((prev: SortingState) => SortingState),
  ) => void
  setCreatedAfter: (date: string | null) => void
  setCreatedBefore: (date: string | null) => void
  setColumnVisibility: (
    visibility:
      | Record<string, boolean>
      | ((prev: Record<string, boolean>) => Record<string, boolean>),
  ) => void
  setView: (view: ViewMode) => void
  setStudentToDelete: (id: string | null) => void
  setIsBulkDeleteOpen: (open: boolean) => void
  setIsBulkEditOpen: (open: boolean) => void
  setIsCreateStudentOpen: (open: boolean) => void
  setStudentToEdit: (student: StudentResponse | null) => void
  setIsUploadPhotoOpen: (open: boolean) => void
  setStudentToUploadPhotoFor: (student: StudentResponse | null) => void
  setIsAssignClassOpen: (open: boolean) => void
  setStudentToAssignClassFor: (student: StudentResponse | null) => void
  setIsGuardiansOpen: (open: boolean) => void
  setStudentToManageGuardiansFor: (student: StudentResponse | null) => void
  setIsAttendanceOpen: (open: boolean) => void
  setStudentToManageAttendanceFor: (student: StudentResponse | null) => void
  setIsMarksOpen: (open: boolean) => void
  setStudentToManageMarksFor: (student: StudentResponse | null) => void
}

export const useStudentsStore = create<StudentsState>((set) => ({
  page: 1,
  search: '',
  debouncedSearch: '',
  statusFilter: 'all',
  sorting: [],
  createdAfter: null,
  createdBefore: null,
  columnVisibility: {},
  view: 'table',
  studentToDelete: null,
  isBulkDeleteOpen: false,
  isBulkEditOpen: false,
  isCreateStudentOpen: false,
  studentToEdit: null,
  isUploadPhotoOpen: false,
  studentToUploadPhotoFor: null,
  isAssignClassOpen: false,
  studentToAssignClassFor: null,
  isGuardiansOpen: false,
  studentToManageGuardiansFor: null,
  isAttendanceOpen: false,
  studentToManageAttendanceFor: null,
  isMarksOpen: false,
  studentToManageMarksFor: null,

  setPage: (page) => set({ page }),
  setSearch: (search) => set({ search }),
  setDebouncedSearch: (debouncedSearch) => set({ debouncedSearch, page: 1 }),
  setStatusFilter: (statusFilter) => set({ statusFilter, page: 1 }),
  setSorting: (sorting) =>
    set((state) => ({
      sorting: typeof sorting === 'function' ? sorting(state.sorting) : sorting,
      page: 1,
    })),
  setCreatedAfter: (createdAfter) => set({ createdAfter, page: 1 }),
  setCreatedBefore: (createdBefore) => set({ createdBefore, page: 1 }),
  setColumnVisibility: (visibility) =>
    set((state) => ({
      columnVisibility:
        typeof visibility === 'function'
          ? visibility(state.columnVisibility)
          : visibility,
    })),
  setView: (view) => set({ view: view }),
  setStudentToDelete: (studentToDelete) => set({ studentToDelete }),
  setIsBulkDeleteOpen: (isBulkDeleteOpen) => set({ isBulkDeleteOpen }),
  setIsBulkEditOpen: (isBulkEditOpen) => set({ isBulkEditOpen }),
  setIsCreateStudentOpen: (isCreateStudentOpen) => set({ isCreateStudentOpen }),
  setStudentToEdit: (studentToEdit) => set({ studentToEdit }),
  setIsUploadPhotoOpen: (isUploadPhotoOpen) => set({ isUploadPhotoOpen }),
  setStudentToUploadPhotoFor: (studentToUploadPhotoFor) =>
    set({ studentToUploadPhotoFor }),
  setIsAssignClassOpen: (isAssignClassOpen) => set({ isAssignClassOpen }),
  setStudentToAssignClassFor: (studentToAssignClassFor) =>
    set({ studentToAssignClassFor }),
  setIsGuardiansOpen: (isGuardiansOpen) => set({ isGuardiansOpen }),
  setStudentToManageGuardiansFor: (studentToManageGuardiansFor) =>
    set({ studentToManageGuardiansFor }),
  setIsAttendanceOpen: (isAttendanceOpen) => set({ isAttendanceOpen }),
  setStudentToManageAttendanceFor: (studentToManageAttendanceFor) =>
    set({ studentToManageAttendanceFor }),
  setIsMarksOpen: (isMarksOpen) => set({ isMarksOpen }),
  setStudentToManageMarksFor: (studentToManageMarksFor) =>
    set({ studentToManageMarksFor }),
}))
