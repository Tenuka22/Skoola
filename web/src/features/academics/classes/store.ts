import { create } from 'zustand'
import type { SortingState } from '@tanstack/react-table'
import type { ClassResponse } from '@/lib/api/types.gen'

interface ClassesStore {
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
  isCreateClassOpen: boolean
  setIsCreateClassOpen: (isOpen: boolean) => void
  classToEdit: ClassResponse | null
  setClassToEdit: (classItem: ClassResponse | null) => void
  classToDelete: string | null
  setClassToDelete: (id: string | null) => void
  isBulkDeleteOpen: boolean
  setIsBulkDeleteOpen: (isOpen: boolean) => void
  gradeId: string | null
  setGradeId: (gradeId: string | null) => void
  academicYearId: string | null
  setAcademicYearId: (academicYearId: string | null) => void
}

export const useClassesStore = create<ClassesStore>((set) => ({
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
  isCreateClassOpen: false,
  setIsCreateClassOpen: (isOpen) => set({ isCreateClassOpen: isOpen }),
  classToEdit: null,
  setClassToEdit: (classItem) => set({ classToEdit: classItem }),
  classToDelete: null,
  setClassToDelete: (id) => set({ classToDelete: id }),
  isBulkDeleteOpen: false,
  setIsBulkDeleteOpen: (isOpen) => set({ isBulkDeleteOpen: isOpen }),
  gradeId: null,
  setGradeId: (gradeId) => set({ gradeId, page: 1 }),
  academicYearId: null,
  setAcademicYearId: (academicYearId) => set({ academicYearId, page: 1 }),
}))
