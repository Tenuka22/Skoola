import {
  parseAsInteger,
  parseAsJson,
  parseAsString,
  parseAsStringLiteral,
} from 'nuqs'
import { z } from 'zod'

export const sortingStateSchema = z.array(
  z.object({
    id: z.string(),
    desc: z.boolean(),
  }),
)

export const visibilityStateSchema = z.record(z.string(), z.boolean())

export const densitySchema = z.enum(['compact', 'comfortable'])

export function createDataTableParsers<T extends z.ZodRawShape>(
  filterSchema: z.ZodObject<T>,
) {
  const defaultFilters = filterSchema.parse({})

  return {
    page: parseAsInteger.withDefault(1),
    limit: parseAsInteger.withDefault(10),
    sort: parseAsJson((val) => sortingStateSchema.parse(val)).withDefault([]),
    search: parseAsString.withDefault(''),
    visibility: parseAsJson((val) =>
      visibilityStateSchema.parse(val),
    ).withDefault({}),
    density: parseAsStringLiteral(['compact', 'comfortable']).withDefault(
      'compact',
    ),
    filters: parseAsJson((val) => filterSchema.parse(val)).withDefault(
      defaultFilters,
    ),
  }
}

// Default parsers for cases where no custom filter is needed
export const dataTableParsers = createDataTableParsers(z.object({}))

export type DataTableUrlState<T extends z.ZodRawShape> = {
  page: number
  limit: number
  sort: z.infer<typeof sortingStateSchema>
  search: string
  visibility: z.infer<typeof visibilityStateSchema>
  density: z.infer<typeof densitySchema>
  filters: z.infer<z.ZodObject<T>>
}
