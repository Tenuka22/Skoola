import { parseAsInteger, parseAsString, useQueryState } from 'nuqs'

export const useSubjectsSearchParams = () => {
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
    parseAsString.withDefault('subject_name_en'),
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
