import { create } from 'zustand'
import { format } from 'date-fns'

interface AttendanceState {
  // Staff UI State
  staffDate: string
  setStaffDate: (date: string) => void
  staffSearch: string
  setStaffSearch: (search: string) => void

  // Student UI State
  studentDate: string
  setStudentDate: (date: string) => void
  studentClassId: string | null
  setStudentClassId: (id: string | null) => void
  studentSearch: string
  setStudentSearch: (search: string) => void
}

const today = format(new Date(), 'yyyy-MM-dd')

export const useAttendanceStore = create<AttendanceState>((set) => ({
  staffDate: today,
  setStaffDate: (date) => set({ staffDate: date }),
  staffSearch: '',
  setStaffSearch: (search) => set({ staffSearch: search }),

  studentDate: today,
  setStudentDate: (date) => set({ studentDate: date }),
  studentClassId: null,
  setStudentClassId: (id) => set({ studentClassId: id }),
  studentSearch: '',
  setStudentSearch: (search) => set({ studentSearch: search }),
}))
