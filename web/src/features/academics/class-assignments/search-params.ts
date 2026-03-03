import { parseAsString, useQueryState } from 'nuqs'

export const useClassAssignmentsSearchParams = () => {
  const [selectedAcademicYearId, setSelectedAcademicYearId] = useQueryState(
    'academic_year_id',
    parseAsString,
  )
  const [selectedClassId, setSelectedClassId] = useQueryState(
    'class_id',
    parseAsString,
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
    search,
    setSearch,
  }
}
