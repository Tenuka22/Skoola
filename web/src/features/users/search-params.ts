import { parseAsInteger, parseAsString, useQueryState } from 'nuqs'

export const userStatusParser = parseAsString.withDefault('all')
export const userAuthMethodParser = parseAsString.withDefault('all')

export const useUsersSearchParams = () => {
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
    userStatusParser,
  )
  const [authFilter, setAuthFilter] = useQueryState(
    'auth_method',
    userAuthMethodParser,
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
    authFilter,
    setAuthFilter,
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
