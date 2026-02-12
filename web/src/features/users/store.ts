import type { SortingState } from '@tanstack/react-table'
import { create } from 'zustand'

import type { UserResponse } from '@/lib/api/types.gen'

type ViewMode = 'table' | 'board' | 'list'

interface UsersState {
  page: number
  search: string
  debouncedSearch: string
  statusFilter: string
  authFilter: string
  sorting: SortingState
  columnVisibility: Record<string, boolean>
  selectedUsers: Set<string>
  view: ViewMode
  userToDelete: string | null
  isBulkDeleteOpen: boolean
  isBulkEditOpen: boolean
  isBulkPermissionsOpen: boolean
  isCreateUserOpen: boolean
  userToEdit: UserResponse | null
  userToManagePermissions: UserResponse | null

  setPage: (page: number) => void
  setSearch: (search: string) => void
  setDebouncedSearch: (search: string) => void
  setStatusFilter: (filter: string) => void
  setAuthFilter: (filter: string) => void
  setSorting: (
    sorting: SortingState | ((prev: SortingState) => SortingState),
  ) => void
  setColumnVisibility: (
    visibility:
      | Record<string, boolean>
      | ((prev: Record<string, boolean>) => Record<string, boolean>),
  ) => void
  setSelectedUsers: (
    users: Set<string> | ((prev: Set<string>) => Set<string>),
  ) => void
  setView: (view: ViewMode) => void
  setUserToDelete: (id: string | null) => void
  setIsBulkDeleteOpen: (open: boolean) => void
  setIsBulkEditOpen: (open: boolean) => void
  setIsBulkPermissionsOpen: (open: boolean) => void
  setIsCreateUserOpen: (open: boolean) => void
  setUserToEdit: (user: UserResponse | null) => void
  setUserToManagePermissions: (user: UserResponse | null) => void
  resetSelection: () => void
}

export const useUsersStore = create<UsersState>((set) => ({
  page: 1,
  search: '',
  debouncedSearch: '',
  statusFilter: 'all',
  authFilter: 'all',
  sorting: [],
  columnVisibility: {},
  selectedUsers: new Set(),
  view: 'table',
  userToDelete: null,
  isBulkDeleteOpen: false,
  isBulkEditOpen: false,
  isBulkPermissionsOpen: false,
  isCreateUserOpen: false,
  userToEdit: null,
  userToManagePermissions: null,

  setPage: (page) => set({ page }),
  setSearch: (search) => set({ search }),
  setDebouncedSearch: (debouncedSearch) => set({ debouncedSearch, page: 1 }),
  setStatusFilter: (statusFilter) => set({ statusFilter, page: 1 }),
  setAuthFilter: (authFilter) => set({ authFilter, page: 1 }),
  setSorting: (sorting) =>
    set((state) => ({
      sorting: typeof sorting === 'function' ? sorting(state.sorting) : sorting,
    })),
  setColumnVisibility: (visibility) =>
    set((state) => ({
      columnVisibility:
        typeof visibility === 'function'
          ? visibility(state.columnVisibility)
          : visibility,
    })),
  setSelectedUsers: (selectedUsers) =>
    set((state) => ({
      selectedUsers:
        typeof selectedUsers === 'function'
          ? selectedUsers(state.selectedUsers)
          : selectedUsers,
    })),
  setView: (view) => set({ view }),
  setUserToDelete: (userToDelete) => set({ userToDelete }),
  setIsBulkDeleteOpen: (isBulkDeleteOpen) => set({ isBulkDeleteOpen }),
  setIsBulkEditOpen: (isBulkEditOpen) => set({ isBulkEditOpen }),
  setIsBulkPermissionsOpen: (isBulkPermissionsOpen) =>
    set({ isBulkPermissionsOpen }),
  setIsCreateUserOpen: (isCreateUserOpen) => set({ isCreateUserOpen }),
  setUserToEdit: (userToEdit) => set({ userToEdit }),
  setUserToManagePermissions: (userToManagePermissions) =>
    set({ userToManagePermissions }),
  resetSelection: () => set({ selectedUsers: new Set() }),
}))
