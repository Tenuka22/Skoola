import { parseAsInteger, parseAsJson, parseAsString, useQueryState } from 'nuqs'
import { sortingStateSchema } from '@/components/data-table/store'

export const staffTypeParser = parseAsString.withDefault('all')
export const employmentStatusParser = parseAsString.withDefault('all')

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
    staffTypeParser,
  )
  const [employmentStatusFilter, setEmploymentStatusFilter] = useQueryState(
    'employment_status',
    employmentStatusParser,
  )
  const [sort, setSort] = useQueryState(
    'sort',
    parseAsJson((val) => sortingStateSchema.parse(val)).withDefault([]),
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
    sort,
    setSort,
  }
}
