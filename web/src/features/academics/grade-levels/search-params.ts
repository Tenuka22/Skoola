import { parseAsInteger, parseAsString, useQueryState } from 'nuqs'

export const useGradeLevelsSearchParams = () => {
  const [page, setPage] = useQueryState('page', parseAsInteger.withDefault(1))
  const [limit, setLimit] = useQueryState(
    'limit',
    parseAsInteger.withDefault(10),
  )
  const [search, setSearch] = useQueryState(
    'search',
    parseAsString.withDefault(''),
  )
  const [sortBy, setSortBy] = useQueryState(
    'sort_by',
    parseAsString.withDefault('grade_number'),
  )
  const [sortOrder, setSortOrder] = useQueryState(
    'sort_order',
    parseAsString.withDefault('asc'),
  )

  return {
    page,
    setPage,
    limit,
    setLimit,
    search,
    setSearch,
    sortBy,
    setSortBy,
    sortOrder,
    setSortOrder,
  }
}
