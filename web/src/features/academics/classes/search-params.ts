import { parseAsInteger, parseAsString, useQueryState } from 'nuqs'

export const useClassesSearchParams = () => {
  const [page, setPage] = useQueryState('page', parseAsInteger.withDefault(1))
  const [limit, setLimit] = useQueryState(
    'limit',
    parseAsInteger.withDefault(10),
  )
  const [search, setSearch] = useQueryState(
    'search',
    parseAsString.withDefault(''),
  )
  const [gradeId, setGradeId] = useQueryState('grade_id', parseAsString)
  const [academicYearId, setAcademicYearId] = useQueryState(
    'academic_year_id',
    parseAsString,
  )
  const [sortBy, setSortBy] = useQueryState(
    'sort_by',
    parseAsString.withDefault('created_at'),
  )
  const [sortOrder, setSortOrder] = useQueryState(
    'sort_order',
    parseAsString.withDefault('desc'),
  )

  return {
    page,
    setPage,
    limit,
    setLimit,
    search,
    setSearch,
    gradeId,
    setGradeId,
    academicYearId,
    setAcademicYearId,
    sortBy,
    setSortBy,
    sortOrder,
    setSortOrder,
  }
}
