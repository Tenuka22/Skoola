import { create } from 'zustand'

interface TermsStore {
  isCreateTermOpen: boolean
  setIsCreateTermOpen: (isOpen: boolean) => void
}

export const useTermsStore = create<TermsStore>((set) => ({
  isCreateTermOpen: false,
  setIsCreateTermOpen: (isOpen) => set({ isCreateTermOpen: isOpen }),
}))
