import { parseAsInteger, parseAsString, useQueryState } from 'nuqs'

export const studentStatusParser = parseAsString.withDefault('all')

export const useStudentsSearchParams = () => {
  const [page, setPage] = useQueryState('page', parseAsInteger.withDefault(1))
  const [limit, setLimit] = useQueryState(
    'limit',
    parseAsInteger.withDefault(10),
  )
  const [search, setSearch] = useQueryState(
    'search',
    parseAsString.withDefault(''),
  )
  const [statusFilter, setStatusFilter] = useQueryState(
    'status',
    studentStatusParser,
  )
  const [sortBy, setSortBy] = useQueryState(
    'sort_by',
    parseAsString.withDefault('created_at'),
  )
  const [sortOrder, setSortOrder] = useQueryState(
    'sort_order',
    parseAsString.withDefault('desc'),
  )

  const [createdAfter, setCreatedAfter] = useQueryState(
    'created_after',
    parseAsString,
  )
  const [createdBefore, setCreatedBefore] = useQueryState(
    'created_before',
    parseAsString,
  )

  const [view, setView] = useQueryState(
    'view',
    parseAsString.withDefault('table'),
  )

  return {
    page,
    setPage,
    limit,
    setLimit,
    search,
    setSearch,
    statusFilter,
    setStatusFilter,
    sortBy,
    setSortBy,
    sortOrder,
    setSortOrder,
    createdAfter,
    setCreatedAfter,
    createdBefore,
    setCreatedBefore,
    view,
    setView,
  }
}
