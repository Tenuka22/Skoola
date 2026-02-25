import { create } from 'zustand'
import type { SortingState } from '@tanstack/react-table'
import type { GradeLevelResponse } from '@/lib/api/types.gen'

interface GradeLevelsStore {
  search: string
  setSearch: (search: string) => void
  debouncedSearch: string
  setDebouncedSearch: (debouncedSearch: string) => void
  page: number
  setPage: (page: number) => void
  sorting: SortingState
  setSorting: (
    sorting: SortingState | ((old: SortingState) => SortingState),
  ) => void
  isCreateGradeLevelOpen: boolean
  setIsCreateGradeLevelOpen: (isOpen: boolean) => void
  gradeLevelToEdit: GradeLevelResponse | null
  setGradeLevelToEdit: (gradeLevel: GradeLevelResponse | null) => void
  gradeLevelToDelete: string | null
  setGradeLevelToDelete: (id: string | null) => void
  isBulkDeleteOpen: boolean
  setIsBulkDeleteOpen: (isOpen: boolean) => void
}

export const useGradeLevelsStore = create<GradeLevelsStore>((set) => ({
  search: '',
  setSearch: (search) => set({ search, page: 1 }),
  debouncedSearch: '',
  setDebouncedSearch: (debouncedSearch) => set({ debouncedSearch }),
  page: 1,
  setPage: (page) => set({ page }),
  sorting: [],
  setSorting: (updater) =>
    set((state) => ({
      sorting: typeof updater === 'function' ? updater(state.sorting) : updater,
    })),
  isCreateGradeLevelOpen: false,
  setIsCreateGradeLevelOpen: (isOpen) =>
    set({ isCreateGradeLevelOpen: isOpen }),
  gradeLevelToEdit: null,
  setGradeLevelToEdit: (gradeLevel) => set({ gradeLevelToEdit: gradeLevel }),
  gradeLevelToDelete: null,
  setGradeLevelToDelete: (id) => set({ gradeLevelToDelete: id }),
  isBulkDeleteOpen: false,
  setIsBulkDeleteOpen: (isOpen) => set({ isBulkDeleteOpen: isOpen }),
}))
