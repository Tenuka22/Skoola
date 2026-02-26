export const TIMETABLE_VIEW_MODES = ['class', 'teacher'] as const
export type TimetableViewMode = (typeof TIMETABLE_VIEW_MODES)[number]
export const isTimetableViewMode = (
  value: string,
): value is TimetableViewMode =>
  TIMETABLE_VIEW_MODES.some((mode) => mode === value)

export const DAYS_OF_WEEK = [
  'Monday',
  'Tuesday',
  'Wednesday',
  'Thursday',
  'Friday',
  'Saturday',
  'Sunday',
] as const

export type DayOfWeek = (typeof DAYS_OF_WEEK)[number]
