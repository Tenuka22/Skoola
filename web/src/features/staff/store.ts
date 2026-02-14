import { create } from 'zustand'
import type { SortingState } from '@tanstack/react-table'
import type { StaffResponse } from '@/lib/api'

type ViewMode = 'table' | 'board' | 'list'

interface StaffState {
  page: number
  search: string
  debouncedSearch: string
  staffTypeFilter: string
  employmentStatusFilter: string
  sorting: SortingState
  columnVisibility: Record<string, boolean>
  view: ViewMode
  staffToDelete: string | null
  isBulkDeleteOpen: boolean
  isBulkEditOpen: boolean
  isCreateStaffOpen: boolean
  staffToEdit: StaffResponse | null
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
  setView: (view: ViewMode) => void
  setStaffToDelete: (id: string | null) => void
  setIsBulkDeleteOpen: (open: boolean) => void
  setIsBulkEditOpen: (open: boolean) => void
  setIsCreateStaffOpen: (open: boolean) => void
  setStaffToEdit: (staff: StaffResponse | null) => void
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
}))
