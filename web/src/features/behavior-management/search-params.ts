import { parseAsString, useQueryState } from 'nuqs'

export const useBehaviorSearchParams = () => {
  const [search, setSearch] = useQueryState(
    'search',
    parseAsString.withDefault(''),
  )

  return {
    search,
    setSearch,
  }
}
