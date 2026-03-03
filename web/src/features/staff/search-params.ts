import { parseAsInteger, parseAsString, useQueryState } from 'nuqs'

export const useStaffSearchParams = () => {
  const [page, setPage] = useQueryState('page', parseAsInteger.withDefault(1))
  const [limit, setLimit] = useQueryState(
    'limit',
    parseAsInteger.withDefault(10),
  )
  const [search, setSearch] = useQueryState(
    'search',
    parseAsString.withDefault(''),
  )
  const [staffTypeFilter, setStaffTypeFilter] = useQueryState(
    'staff_type',
    parseAsString.withDefault('all'),
  )
  const [employmentStatusFilter, setEmploymentStatusFilter] = useQueryState(
    'employment_status',
    parseAsString.withDefault('all'),
  )
  const [sortBy, setSortBy] = useQueryState(
    'sort_by',
    parseAsString.withDefault('created_at'),
  )
  const [sortOrder, setSortOrder] = useQueryState(
    'sort_order',
    parseAsString.withDefault('desc'),
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
    staffTypeFilter,
    setStaffTypeFilter,
    employmentStatusFilter,
    setEmploymentStatusFilter,
    sortBy,
    setSortBy,
    sortOrder,
    setSortOrder,
    view,
    setView,
  }
}
