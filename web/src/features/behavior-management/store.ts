import { create } from 'zustand'
import type {
  BehaviorIncidentResponse,
  BehaviorIncidentTypeResponse,
  StudentResponse,
} from '@/lib/api/types.gen'

type BehaviorTab = 'incidents' | 'types'

interface BehaviorState {
  activeTab: BehaviorTab
  setActiveTab: (tab: BehaviorTab) => void

  // Behavior Types
  isCreateTypeOpen: boolean
  setIsCreateTypeOpen: (open: boolean) => void
  typeToEdit: BehaviorIncidentTypeResponse | null
  setTypeToEdit: (type: BehaviorIncidentTypeResponse | null) => void
  typeToDelete: string | null
  setTypeToDelete: (id: string | null) => void

  // Behavior Incidents
  isRecordIncidentOpen: boolean
  setIsRecordIncidentOpen: (open: boolean) => void
  incidentToEdit: BehaviorIncidentResponse | null
  setIncidentToEdit: (incident: BehaviorIncidentResponse | null) => void
  incidentToDelete: string | null
  setIncidentToDelete: (id: string | null) => void
  studentForIncident: StudentResponse | null
  setStudentForIncident: (student: StudentResponse | null) => void

  // Search and Pagination
  search: string
  setSearch: (search: string) => void
  debouncedSearch: string
  setDebouncedSearch: (search: string) => void
  page: number
  setPage: (page: number) => void
}

export const useBehaviorStore = create<BehaviorState>((set) => ({
  activeTab: 'incidents',
  setActiveTab: (tab) => set({ activeTab: tab }),

  isCreateTypeOpen: false,
  setIsCreateTypeOpen: (open) => set({ isCreateTypeOpen: open }),
  typeToEdit: null,
  setTypeToEdit: (type) => set({ typeToEdit: type }),
  typeToDelete: null,
  setTypeToDelete: (id) => set({ typeToDelete: id }),

  isRecordIncidentOpen: false,
  setIsRecordIncidentOpen: (open) => set({ isRecordIncidentOpen: open }),
  incidentToEdit: null,
  setIncidentToEdit: (incident) => set({ incidentToEdit: incident }),
  incidentToDelete: null,
  setIncidentToDelete: (id) => set({ incidentToDelete: id }),
  studentForIncident: null,
  setStudentForIncident: (student) => set({ studentForIncident: student }),

  search: '',
  setSearch: (search) => set({ search }),
  debouncedSearch: '',
  setDebouncedSearch: (search) => set({ debouncedSearch: search }),
  page: 1,
  setPage: (page) => set({ page }),
}))
