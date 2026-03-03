import { parseAsString, useQueryState } from 'nuqs'

export const useTimetablesSearchParams = () => {
  const [selectedAcademicYearId, setSelectedAcademicYearId] = useQueryState(
    'academic_year_id',
    parseAsString,
  )
  const [selectedClassId, setSelectedClassId] = useQueryState(
    'class_id',
    parseAsString,
  )
  const [selectedTeacherId, setSelectedTeacherId] = useQueryState(
    'teacher_id',
    parseAsString,
  )
  const [selectedDayOfWeek, setSelectedDayOfWeek] = useQueryState(
    'day_of_week',
    parseAsString,
  )
  const [viewMode, setViewMode] = useQueryState(
    'view_mode',
    parseAsString.withDefault('class'),
  )
  const [search, setSearch] = useQueryState(
    'search',
    parseAsString.withDefault(''),
  )

  return {
    selectedAcademicYearId,
    setSelectedAcademicYearId,
    selectedClassId,
    setSelectedClassId,
    selectedTeacherId,
    setSelectedTeacherId,
    selectedDayOfWeek,
    setSelectedDayOfWeek,
    viewMode,
    setViewMode,
    search,
    setSearch,
  }
}
