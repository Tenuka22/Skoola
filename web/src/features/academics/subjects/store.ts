import { create } from 'zustand'
import type { OnChangeFn, SortingState } from '@tanstack/react-table'
import type { SubjectResponse } from '@/lib/api/types.gen'

interface SubjectsStore {
  search: string
  setSearch: (search: string) => void
  debouncedSearch: string
  setDebouncedSearch: (debouncedSearch: string) => void
  page: number
  setPage: (page: number) => void
  sorting: SortingState
  setSorting: OnChangeFn<SortingState>
  isCreateSubjectOpen: boolean
  setIsCreateSubjectOpen: (isOpen: boolean) => void
  subjectToEdit: SubjectResponse | null
  setSubjectToEdit: (subject: SubjectResponse | null) => void
  subjectToDelete: string | null
  setSubjectToDelete: (id: string | null) => void
  isBulkDeleteOpen: boolean
  setIsBulkDeleteOpen: (isOpen: boolean) => void
  subjectToAssignToGrade: SubjectResponse | null
  setSubjectToAssignToGrade: (subject: SubjectResponse | null) => void
  subjectToAssignToStream: SubjectResponse | null
  setSubjectToAssignToStream: (subject: SubjectResponse | null) => void
  subjectToEnrollStudent: SubjectResponse | null
  setSubjectToEnrollStudent: (subject: SubjectResponse | null) => void
  subjectToViewEnrollments: SubjectResponse | null
  setSubjectToViewEnrollments: (subject: SubjectResponse | null) => void
}

export const useSubjectsStore = create<SubjectsStore>((set) => ({
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
  isCreateSubjectOpen: false,
  setIsCreateSubjectOpen: (isOpen) => set({ isCreateSubjectOpen: isOpen }),
  subjectToEdit: null,
  setSubjectToEdit: (subject) => set({ subjectToEdit: subject }),
  subjectToDelete: null,
  setSubjectToDelete: (id) => set({ subjectToDelete: id }),
  isBulkDeleteOpen: false,
  setIsBulkDeleteOpen: (isOpen) => set({ isBulkDeleteOpen: isOpen }),
  subjectToAssignToGrade: null,
  setSubjectToAssignToGrade: (subject) =>
    set({ subjectToAssignToGrade: subject }),
  subjectToAssignToStream: null,
  setSubjectToAssignToStream: (subject) =>
    set({ subjectToAssignToStream: subject }),
  subjectToEnrollStudent: null,
  setSubjectToEnrollStudent: (subject) =>
    set({ subjectToEnrollStudent: subject }),
  subjectToViewEnrollments: null,
  setSubjectToViewEnrollments: (subject) =>
    set({ subjectToViewEnrollments: subject }),
}))
