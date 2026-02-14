import { create } from 'zustand'
import type { SortingState } from '@tanstack/react-table'
import type { Permission } from '@/lib/api/types.gen'
import type { PermissionSet } from './types'

export type PermissionsViewMode = 'permissions' | 'permission-sets'

interface PermissionsState {
  // Permissions Table State
  permissionsPage: number
  permissionsSearch: string
  permissionsSorting: SortingState

  // Permission Sets State
  permissionSetsSearch: string

  // UI State
  view: PermissionsViewMode

  // Modal States
  permissionToDelete: number | null
  permissionToEdit: Permission | null
  isCreatePermissionOpen: boolean

  permissionSetToDelete: string | null
  permissionSetToEdit: PermissionSet | null
  isCreatePermissionSetOpen: boolean
  permissionSetToManage: PermissionSet | null

  // Actions
  setPermissionsPage: (page: number) => void
  setPermissionsSearch: (search: string) => void
  setPermissionsSorting: (
    sorting: SortingState | ((prev: SortingState) => SortingState),
  ) => void

  setPermissionSetsSearch: (search: string) => void

  setView: (view: PermissionsViewMode) => void

  setPermissionToDelete: (id: number | null) => void
  setPermissionToEdit: (permission: Permission | null) => void
  setIsCreatePermissionOpen: (open: boolean) => void

  setPermissionSetToDelete: (id: string | null) => void
  setPermissionSetToEdit: (set: PermissionSet | null) => void
  setIsCreatePermissionSetOpen: (open: boolean) => void
  setPermissionSetToManage: (set: PermissionSet | null) => void
}

export const usePermissionsStore = create<PermissionsState>((set) => ({
  permissionsPage: 1,
  permissionsSearch: '',
  permissionsSorting: [{ id: 'name', desc: false }],

  permissionSetsSearch: '',

  view: 'permissions',

  permissionToDelete: null,
  permissionToEdit: null,
  isCreatePermissionOpen: false,

  permissionSetToDelete: null,
  permissionSetToEdit: null,
  isCreatePermissionSetOpen: false,
  permissionSetToManage: null,

  setPermissionsPage: (permissionsPage) => set({ permissionsPage }),
  setPermissionsSearch: (permissionsSearch) =>
    set({ permissionsSearch, permissionsPage: 1 }),
  setPermissionsSorting: (sorting) =>
    set((state) => ({
      permissionsSorting:
        typeof sorting === 'function'
          ? sorting(state.permissionsSorting)
          : sorting,
      permissionsPage: 1,
    })),

  setPermissionSetsSearch: (permissionSetsSearch) =>
    set({ permissionSetsSearch }),

  setView: (view) => set({ view }),

  setPermissionToDelete: (permissionToDelete) => set({ permissionToDelete }),
  setPermissionToEdit: (permissionToEdit) => set({ permissionToEdit }),
  setIsCreatePermissionOpen: (isCreatePermissionOpen) =>
    set({ isCreatePermissionOpen }),

  setPermissionSetToDelete: (permissionSetToDelete) =>
    set({ permissionSetToDelete }),
  setPermissionSetToEdit: (permissionSetToEdit) => set({ permissionSetToEdit }),
  setIsCreatePermissionSetOpen: (isCreatePermissionSetOpen) =>
    set({ isCreatePermissionSetOpen }),
  setPermissionSetToManage: (permissionSetToManage) =>
    set({ permissionSetToManage }),
}))
