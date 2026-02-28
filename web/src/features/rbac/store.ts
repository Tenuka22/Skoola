import { create } from 'zustand'
import type { RBACActiveTab } from './constants'
import type { RoleEnum } from '@/lib/api/types.gen'

interface RBACState {
  activeTab: RBACActiveTab
  selectedUserId: string | null
  selectedRoleId: RoleEnum | null
  selectedPermissionSetId: string | null
  selectedRoleSetId: string | null

  // Modal states
  isRoleEditorOpen: boolean
  isPermissionSetEditorOpen: boolean
  isRoleSetEditorOpen: boolean

  setActiveTab: (tab: RBACActiveTab) => void
  setSelectedUserId: (id: string | null) => void
  setSelectedRoleId: (role: RoleEnum | null) => void
  setSelectedPermissionSetId: (id: string | null) => void
  setSelectedRoleSetId: (id: string | null) => void
  setIsRoleEditorOpen: (open: boolean) => void
  setIsPermissionSetEditorOpen: (open: boolean) => void
  setIsRoleSetEditorOpen: (open: boolean) => void
}

export const useRBACStore = create<RBACState>((set) => ({
  activeTab: 'users',
  selectedUserId: null,
  selectedRoleId: null,
  selectedPermissionSetId: null,
  selectedRoleSetId: null,
  isRoleEditorOpen: false,
  isPermissionSetEditorOpen: false,
  isRoleSetEditorOpen: false,

  setActiveTab: (activeTab) => set({ activeTab }),
  setSelectedUserId: (selectedUserId) => set({ selectedUserId }),
  setSelectedRoleId: (selectedRoleId) => set({ selectedRoleId }),
  setSelectedPermissionSetId: (selectedPermissionSetId) =>
    set({ selectedPermissionSetId }),
  setSelectedRoleSetId: (selectedRoleSetId) => set({ selectedRoleSetId }),
  setIsRoleEditorOpen: (isRoleEditorOpen) => set({ isRoleEditorOpen }),
  setIsPermissionSetEditorOpen: (isPermissionSetEditorOpen) =>
    set({ isPermissionSetEditorOpen }),
  setIsRoleSetEditorOpen: (isRoleSetEditorOpen) => set({ isRoleSetEditorOpen }),
}))
