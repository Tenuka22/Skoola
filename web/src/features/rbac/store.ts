import { create } from 'zustand'
import type { RBACActiveTab } from './constants'
import type { RoleEnum } from '@/lib/api/types.gen'

interface RBACState {
  activeTab: RBACActiveTab
  selectedUserId: string | null
  selectedRoleId: RoleEnum | null
  selectedPermissionSetId: string | null

  // Modal states
  isRoleEditorOpen: boolean
  isPermissionSetEditorOpen: boolean

  setActiveTab: (tab: RBACActiveTab) => void
  setSelectedUserId: (id: string | null) => void
  setSelectedRoleId: (role: RoleEnum | null) => void
  setSelectedPermissionSetId: (id: string | null) => void
  setIsRoleEditorOpen: (open: boolean) => void
  setIsPermissionSetEditorOpen: (open: boolean) => void
}

export const useRBACStore = create<RBACState>((set) => ({
  activeTab: 'users',
  selectedUserId: null,
  selectedRoleId: null,
  selectedPermissionSetId: null,
  isRoleEditorOpen: false,
  isPermissionSetEditorOpen: false,

  setActiveTab: (activeTab) => set({ activeTab }),
  setSelectedUserId: (selectedUserId) => set({ selectedUserId }),
  setSelectedRoleId: (selectedRoleId) => set({ selectedRoleId }),
  setSelectedPermissionSetId: (selectedPermissionSetId) =>
    set({ selectedPermissionSetId }),
  setIsRoleEditorOpen: (isRoleEditorOpen) => set({ isRoleEditorOpen }),
  setIsPermissionSetEditorOpen: (isPermissionSetEditorOpen) =>
    set({ isPermissionSetEditorOpen }),
}))
