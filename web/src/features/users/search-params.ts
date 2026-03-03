import { parseAsInteger, parseAsJson, parseAsString, useQueryState } from 'nuqs'
import { sortingStateSchema } from '@/components/data-table/store'

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
  const [sort, setSort] = useQueryState(
    'sort',
    parseAsJson((val) => sortingStateSchema.parse(val)).withDefault([]),
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
    sort,
    setSort,
    createdAfter,
    setCreatedAfter,
    createdBefore,
    setCreatedBefore,
    view,
    setView,
  }
}
