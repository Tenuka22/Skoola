import { create } from 'zustand'
import type { SortingState } from '@tanstack/react-table'
import type { TimetableResponse } from '@/lib/api/types.gen'

interface TimetablesStore {
  search: string
  setSearch: (search: string) => void
  debouncedSearch: string
  setDebouncedSearch: (debouncedSearch: string) => void
  page: number
  setPage: (page: number) => void
  sorting: SortingState
  setSorting: (sorting: SortingState) => void
  selectedAcademicYearId: string | undefined
  setSelectedAcademicYearId: (id: string | undefined) => void
  selectedClassId: string | undefined
  setSelectedClassId: (id: string | undefined) => void
  selectedTeacherId: string | undefined
  setSelectedTeacherId: (id: string | undefined) => void
  selectedDayOfWeek: string | undefined
  setSelectedDayOfWeek: (day: string | undefined) => void
  viewMode: 'class' | 'teacher'
  setViewMode: (mode: 'class' | 'teacher') => void
  isCreateTimetableEntryOpen: boolean
  setIsCreateTimetableEntryOpen: (isOpen: boolean) => void
  timetableEntryToEdit: TimetableResponse | null
  setTimetableEntryToEdit: (entry: TimetableResponse | null) => void
  timetableEntryToDelete: string | null
  setTimetableEntryToDelete: (id: string | null) => void
}

export const useTimetablesStore = create<TimetablesStore>((set) => ({
  search: '',
  setSearch: (search) => set({ search, page: 1 }),
  debouncedSearch: '',
  setDebouncedSearch: (debouncedSearch) => set({ debouncedSearch }),
  page: 1,
  setPage: (page) => set({ page }),
  sorting: [],
  setSorting: (sorting) => set({ sorting }),
  selectedAcademicYearId: undefined,
  setSelectedAcademicYearId: (id) => set({ selectedAcademicYearId: id }),
  selectedClassId: undefined,
  setSelectedClassId: (id) => set({ selectedClassId: id }),
  selectedTeacherId: undefined,
  setSelectedTeacherId: (id) => set({ selectedTeacherId: id }),
  selectedDayOfWeek: undefined,
  setSelectedDayOfWeek: (day) => set({ selectedDayOfWeek: day }),
  viewMode: 'class',
  setViewMode: (mode) => set({ viewMode: mode }),
  isCreateTimetableEntryOpen: false,
  setIsCreateTimetableEntryOpen: (isOpen) =>
    set({ isCreateTimetableEntryOpen: isOpen }),
  timetableEntryToEdit: null,
  setTimetableEntryToEdit: (entry) => set({ timetableEntryToEdit: entry }),
  timetableEntryToDelete: null,
  setTimetableEntryToDelete: (id) => set({ timetableEntryToDelete: id }),
}))
