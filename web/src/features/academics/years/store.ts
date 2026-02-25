import { create } from 'zustand'
import type { SortingState } from '@tanstack/react-table'
import type { AcademicYearResponse } from '@/lib/api'

interface AcademicYearsState {
  page: number
  search: string
  debouncedSearch: string
  sorting: SortingState
  columnVisibility: Record<string, boolean>
  yearToDelete: string | null
  isBulkDeleteOpen: boolean
  isBulkEditOpen: boolean
  isCreateYearOpen: boolean
  yearToEdit: AcademicYearResponse | null
  setPage: (page: number) => void
  setSearch: (search: string) => void
  setDebouncedSearch: (search: string) => void
  setSorting: (
    sorting: SortingState | ((prev: SortingState) => SortingState),
  ) => void
  setColumnVisibility: (
    visibility:
      | Record<string, boolean>
      | ((prev: Record<string, boolean>) => Record<string, boolean>),
  ) => void
  setYearToDelete: (id: string | null) => void
  setIsBulkDeleteOpen: (open: boolean) => void
  setIsBulkEditOpen: (open: boolean) => void
  setIsCreateYearOpen: (open: boolean) => void
  setYearToEdit: (year: AcademicYearResponse | null) => void
}

export const useAcademicYearsStore = create<AcademicYearsState>((set) => ({
  page: 1,
  search: '',
  debouncedSearch: '',
  sorting: [],
  columnVisibility: {},
  yearToDelete: null,
  isBulkDeleteOpen: false,
  isBulkEditOpen: false,
  isCreateYearOpen: false,
  yearToEdit: null,

  setPage: (page) => set({ page }),
  setSearch: (search) => set({ search }),
  setDebouncedSearch: (debouncedSearch) => set({ debouncedSearch, page: 1 }),
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
  setYearToDelete: (yearToDelete) => set({ yearToDelete }),
  setIsBulkDeleteOpen: (isBulkDeleteOpen) => set({ isBulkDeleteOpen }),
  setIsBulkEditOpen: (isBulkEditOpen) => set({ isBulkEditOpen }),
  setIsCreateYearOpen: (isCreateYearOpen) => set({ isCreateYearOpen }),
  setYearToEdit: (yearToEdit) => set({ yearToEdit }),
}))
