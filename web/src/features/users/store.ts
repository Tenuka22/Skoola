import { create } from 'zustand'
import type { SortingState } from '@tanstack/react-table'

import type { UserResponse } from '@/lib/api/types.gen'

export type ViewMode = 'table' | 'board' | 'list'

interface UsersState {
  page: number
  search: string
  debouncedSearch: string
  statusFilter: string
  authFilter: string
  createdAfter: string | null
  createdBefore: string | null
  sorting: SortingState
  columnVisibility: Record<string, boolean>
  view: ViewMode
  userToDelete: string | null
  isBulkDeleteOpen: boolean
  isBulkEditOpen: boolean
  isCreateUserOpen: boolean
  userToEdit: UserResponse | null
  userToLock: UserResponse | null
  userToManagePermissions: UserResponse | null

  setPage: (page: number) => void
  setSearch: (search: string) => void
  setDebouncedSearch: (search: string) => void
  setStatusFilter: (filter: string) => void
  setAuthFilter: (filter: string) => void
  setCreatedAfter: (date: string | null) => void
  setCreatedBefore: (date: string | null) => void
  setSorting: (
    sorting: SortingState | ((prev: SortingState) => SortingState),
  ) => void
  setColumnVisibility: (
    visibility:
      | Record<string, boolean>
      | ((prev: Record<string, boolean>) => Record<string, boolean>),
  ) => void
  setView: (view: ViewMode) => void
  setUserToDelete: (id: string | null) => void
  setIsBulkDeleteOpen: (open: boolean) => void
  setIsBulkEditOpen: (open: boolean) => void
  setIsCreateUserOpen: (open: boolean) => void
  setUserToEdit: (user: UserResponse | null) => void
  setUserToLock: (user: UserResponse | null) => void
  setUserToManagePermissions: (user: UserResponse | null) => void
}

export const useUsersStore = create<UsersState>((set) => ({
  page: 1,
  search: '',
  debouncedSearch: '',
  statusFilter: 'all',
  authFilter: 'all',
  createdAfter: null,
  createdBefore: null,
  sorting: [],
  columnVisibility: {},
  view: 'table',
  userToDelete: null,
  isBulkDeleteOpen: false,
  isBulkEditOpen: false,
  isCreateUserOpen: false,
  userToEdit: null,
  userToLock: null,
  userToManagePermissions: null,

  setPage: (page) => set({ page }),
  setSearch: (search) => set({ search }),
  setDebouncedSearch: (debouncedSearch) => set({ debouncedSearch, page: 1 }),
  setStatusFilter: (statusFilter) => set({ statusFilter, page: 1 }),
  setAuthFilter: (authFilter) => set({ authFilter, page: 1 }),
  setCreatedAfter: (createdAfter) => set({ createdAfter, page: 1 }),
  setCreatedBefore: (createdBefore) => set({ createdBefore, page: 1 }),
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

  setView: (view) => set({ view }),
  setUserToDelete: (userToDelete) => set({ userToDelete }),
  setIsBulkDeleteOpen: (isBulkDeleteOpen) => set({ isBulkDeleteOpen }),
  setIsBulkEditOpen: (isBulkEditOpen) => set({ isBulkEditOpen }),
  setIsCreateUserOpen: (isCreateUserOpen) => set({ isCreateUserOpen }),
  setUserToEdit: (userToEdit) => set({ userToEdit }),
  setUserToLock: (userToLock) => set({ userToLock }),
  setUserToManagePermissions: (userToManagePermissions) =>
    set({ userToManagePermissions }),
}))
