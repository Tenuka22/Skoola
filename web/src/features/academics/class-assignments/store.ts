import { create } from 'zustand'
import type { SortingState } from '@tanstack/react-table'
import type { ClassAssignmentRow } from './components/class-assignments-table-columns'

interface ClassAssignmentsStore {
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
  selectedAcademicYearId: string | undefined
  setSelectedAcademicYearId: (id: string | undefined) => void
  selectedClassId: string | undefined
  setSelectedClassId: (id: string | undefined) => void
  isAssignTeacherOpen: boolean
  setIsAssignTeacherOpen: (isOpen: boolean) => void
  assignmentToEdit: ClassAssignmentRow | null
  setAssignmentToEdit: (assignment: ClassAssignmentRow | null) => void
  assignmentToDelete: ClassAssignmentRow | null
  setAssignmentToDelete: (assignment: ClassAssignmentRow | null) => void
}

export const useClassAssignmentsStore = create<ClassAssignmentsStore>(
  (set) => ({
    search: '',
    setSearch: (search) => set({ search, page: 1 }),
    debouncedSearch: '',
    setDebouncedSearch: (debouncedSearch) => set({ debouncedSearch }),
    page: 1,
    setPage: (page) => set({ page }),
    sorting: [],
    setSorting: (updater) =>
      set((state) => ({
        sorting:
          typeof updater === 'function' ? updater(state.sorting) : updater,
      })),
    selectedAcademicYearId: undefined,
    setSelectedAcademicYearId: (id) => set({ selectedAcademicYearId: id }),
    selectedClassId: undefined,
    setSelectedClassId: (id) => set({ selectedClassId: id }),
    isAssignTeacherOpen: false,
    setIsAssignTeacherOpen: (isOpen) => set({ isAssignTeacherOpen: isOpen }),
    assignmentToEdit: null,
    setAssignmentToEdit: (assignment) => set({ assignmentToEdit: assignment }),
    assignmentToDelete: null,
    setAssignmentToDelete: (assignment) =>
      set({ assignmentToDelete: assignment }),
  }),
)
