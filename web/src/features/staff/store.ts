import { create } from 'zustand'
import type { SortingState } from '@tanstack/react-table'
import type { StaffResponse } from '@/lib/api'
import type { ListViewMode } from '@/lib/constants/view-modes'

export type StaffViewMode = ListViewMode

interface StaffState {
  page: number
  search: string
  debouncedSearch: string
  staffTypeFilter: string
  employmentStatusFilter: string
  sorting: SortingState
  columnVisibility: Record<string, boolean>
  view: StaffViewMode
  staffToDelete: string | null
  isBulkDeleteOpen: boolean
  isBulkEditOpen: boolean
  isCreateStaffOpen: boolean
  staffToEdit: StaffResponse | null
  isUploadPhotoOpen: boolean
  staffToUploadPhotoFor: StaffResponse | null
  isAssignClassOpen: boolean
  staffToAssignClassFor: StaffResponse | null
  isAssignSubjectOpen: boolean
  staffToAssignSubjectFor: StaffResponse | null
  isWorkloadOpen: boolean
  staffToViewWorkloadFor: StaffResponse | null
  isAttendanceOpen: boolean
  staffToManageAttendanceFor: StaffResponse | null
  isLeavesOpen: boolean
  staffToManageLeavesFor: StaffResponse | null
  isPermissionSetsOpen: boolean
  staffToManagePermissionSetsFor: StaffResponse | null
  setPage: (page: number) => void
  setSearch: (search: string) => void
  setDebouncedSearch: (search: string) => void
  setStaffTypeFilter: (filter: string) => void
  setEmploymentStatusFilter: (filter: string) => void
  setSorting: (
    sorting: SortingState | ((prev: SortingState) => SortingState),
  ) => void
  setColumnVisibility: (
    visibility:
      | Record<string, boolean>
      | ((prev: Record<string, boolean>) => Record<string, boolean>),
  ) => void
  setView: (view: StaffViewMode) => void
  setStaffToDelete: (id: string | null) => void
  setIsBulkDeleteOpen: (open: boolean) => void
  setIsBulkEditOpen: (open: boolean) => void
  setIsCreateStaffOpen: (open: boolean) => void
  setStaffToEdit: (staff: StaffResponse | null) => void
  setIsUploadPhotoOpen: (open: boolean) => void
  setStaffToUploadPhotoFor: (staff: StaffResponse | null) => void
  setIsAssignClassOpen: (isOpen: boolean) => void
  setStaffToAssignClassFor: (staff: StaffResponse | null) => void
  setIsAssignSubjectOpen: (isOpen: boolean) => void
  setStaffToAssignSubjectFor: (staff: StaffResponse | null) => void
  setIsWorkloadOpen: (isOpen: boolean) => void
  setStaffToViewWorkloadFor: (staff: StaffResponse | null) => void
  setIsAttendanceOpen: (isOpen: boolean) => void
  setStaffToManageAttendanceFor: (staff: StaffResponse | null) => void
  setIsLeavesOpen: (isOpen: boolean) => void
  setStaffToManageLeavesFor: (staff: StaffResponse | null) => void
  setIsPermissionSetsOpen: (isOpen: boolean) => void
  setStaffToManagePermissionSetsFor: (staff: StaffResponse | null) => void
}

export const useStaffStore = create<StaffState>((set) => ({
  page: 1,
  search: '',
  debouncedSearch: '',
  staffTypeFilter: 'all',
  employmentStatusFilter: 'all',
  sorting: [],
  columnVisibility: {},
  view: 'table',
  staffToDelete: null,
  isBulkDeleteOpen: false,
  isBulkEditOpen: false,
  isCreateStaffOpen: false,
  staffToEdit: null,
  isUploadPhotoOpen: false,
  staffToUploadPhotoFor: null,
  isAssignClassOpen: false,
  staffToAssignClassFor: null,
  isAssignSubjectOpen: false,
  staffToAssignSubjectFor: null,
  isWorkloadOpen: false,
  staffToViewWorkloadFor: null,
  isAttendanceOpen: false,
  staffToManageAttendanceFor: null,
  isLeavesOpen: false,
  staffToManageLeavesFor: null,
  isPermissionSetsOpen: false,
  staffToManagePermissionSetsFor: null,

  setPage: (page) => set({ page }),
  setSearch: (search) => set({ search }),
  setDebouncedSearch: (debouncedSearch) => set({ debouncedSearch, page: 1 }),
  setStaffTypeFilter: (staffTypeFilter) => set({ staffTypeFilter, page: 1 }),
  setEmploymentStatusFilter: (employmentStatusFilter) =>
    set({ employmentStatusFilter, page: 1 }),
  setSorting: (sorting) =>
    set((state) => ({
      sorting: typeof sorting === 'function' ? sorting(state.sorting) : sorting,
      page: 1,
    })),
  setColumnVisibility: (visibility) =>
    set((state) => ({
      columnVisibility:
        typeof visibility === 'function'
          ? visibility(state.columnVisibility)
          : visibility,
    })),
  setView: (view) => set({ view: view }),
  setStaffToDelete: (staffToDelete) => set({ staffToDelete }),
  setIsBulkDeleteOpen: (isBulkDeleteOpen) => set({ isBulkDeleteOpen }),
  setIsBulkEditOpen: (isBulkEditOpen) => set({ isBulkEditOpen }),
  setIsCreateStaffOpen: (isCreateStaffOpen) => set({ isCreateStaffOpen }),
  setStaffToEdit: (staffToEdit) => set({ staffToEdit }),
  setIsUploadPhotoOpen: (isUploadPhotoOpen) => set({ isUploadPhotoOpen }),
  setStaffToUploadPhotoFor: (staffToUploadPhotoFor) =>
    set({ staffToUploadPhotoFor }),
  setIsAssignClassOpen: (isAssignClassOpen) => set({ isAssignClassOpen }),
  setStaffToAssignClassFor: (staffToAssignClassFor) =>
    set({ staffToAssignClassFor }),
  setIsAssignSubjectOpen: (isAssignSubjectOpen) => set({ isAssignSubjectOpen }),
  setStaffToAssignSubjectFor: (staffToAssignSubjectFor) =>
    set({ staffToAssignSubjectFor }),
  setIsWorkloadOpen: (isWorkloadOpen) => set({ isWorkloadOpen }),
  setStaffToViewWorkloadFor: (staffToViewWorkloadFor) =>
    set({ staffToViewWorkloadFor }),
  setIsAttendanceOpen: (isAttendanceOpen) => set({ isAttendanceOpen }),
  setStaffToManageAttendanceFor: (staffToManageAttendanceFor) =>
    set({ staffToManageAttendanceFor }),
  setIsLeavesOpen: (isLeavesOpen) => set({ isLeavesOpen }),
  setStaffToManageLeavesFor: (staffToManageLeavesFor) =>
    set({ staffToManageLeavesFor }),
  setIsPermissionSetsOpen: (isPermissionSetsOpen) =>
    set({ isPermissionSetsOpen }),
  setStaffToManagePermissionSetsFor: (staffToManagePermissionSetsFor) =>
    set({ staffToManagePermissionSetsFor }),
}))
