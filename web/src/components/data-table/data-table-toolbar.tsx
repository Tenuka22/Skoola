'use client'

import * as React from 'react'
import { HugeiconsIcon } from '@hugeicons/react'
import {
  Add01Icon,
  Cancel01Icon,
  Copy01Icon,
  Layout01Icon as DensityIcon,
  Download01Icon as DownloadIcon,
  FileDownloadFreeIcons as FileDownloadIcon,
  FileUploadFreeIcons as FileUploadIcon,
  FilterIcon,
  Search01Icon,
  Tick02Icon,
  Upload01FreeIcons as UploadIcon,
} from '@hugeicons/core-free-icons'

import { DataTablePasteDialog } from './data-table-paste-dialog'
import { getColHeader, getColId } from './utils'
import type {
  DataTableColumnDef,
  DataTableFacetedFilter,
  DataTableToolbarContext,
} from './types'
import type { Column, Table } from '@tanstack/react-table'
import { Button } from '@/components/ui/button'
import {
  DropdownMenu,
  DropdownMenuCheckboxItem,
  DropdownMenuContent,
  DropdownMenuGroup,
  DropdownMenuItem,
  DropdownMenuLabel,
  DropdownMenuRadioGroup,
  DropdownMenuRadioItem,
  DropdownMenuSeparator,
  DropdownMenuTrigger,
} from '@/components/ui/dropdown-menu'
import {
  Popover,
  PopoverContent,
  PopoverTrigger,
} from '@/components/ui/popover'
import { Badge } from '@/components/ui/badge'
import { Separator } from '@/components/ui/separator'
import {
  Command,
  CommandEmpty,
  CommandGroup,
  CommandInput,
  CommandItem,
  CommandList,
  CommandSeparator,
} from '@/components/ui/command'
import { cn } from '@/lib/utils'
import { Box, HStack, Stack, Text } from '@/components/primitives'
import {
  Tooltip,
  TooltipContent,
  TooltipTrigger,
} from '@/components/ui/tooltip'

export interface DataTableToolbarProps<TRow> {
  context: DataTableToolbarContext<TRow>
  allowColumnVisibilityToggle?: boolean
  allowPaste?: boolean
  onImportCSV?: (rows: Array<Record<string, unknown>>) => void
  onImportJSON?: (rows: Array<Record<string, unknown>>) => void
  onExportCSV?: (options?: {
    allData?: boolean
    selectedOnly?: boolean
  }) => void
  onExportPDF?: (options?: {
    allData?: boolean
    selectedOnly?: boolean
  }) => void
  onFetchFullData?: () => Promise<Array<TRow>>
  baseColumns: Array<DataTableColumnDef<TRow, unknown>>
  bulkActions?: (context: {
    selectedRows: Array<TRow>
    table: Table<TRow>
  }) => React.ReactNode
  facetedFilters?: Array<DataTableFacetedFilter>
  search?: string
  onSearchChange?: (value: string) => void
  searchPlaceholder?: string
  extraActions?: React.ReactNode
}

interface ActionTooltipProps {
  children: React.ReactElement
  label: string
}

function ActionTooltip({ children, label }: ActionTooltipProps) {
  return (
    <Tooltip>
      <TooltipTrigger render={children} />
      <TooltipContent>{label}</TooltipContent>
    </Tooltip>
  )
}

export function DataTableToolbar<TRow>({
  allowPaste = true,
  onImportCSV,
  onImportJSON,
  baseColumns,
  bulkActions,
  facetedFilters,
  context,
  onFetchFullData,
  search,
  onSearchChange,
  searchPlaceholder = 'Search...',
  extraActions,
}: DataTableToolbarProps<TRow>) {
  const {
    table,
    selectionCount,
    selectedRows,
    columnVisibility,
    density,
    setDensity,
    importCsv,
    importJson,
    exportCsv,
    exportPdf,
    generateTemplateCsv,
  } = context

  const handleDensityChange = (v: string) => {
    if (v === 'compact' || v === 'comfortable') {
      setDensity(v)
    }
  }

  const visibleFacetedFilters = facetedFilters?.slice(0, 2)
  const overflowFacetedFilters = facetedFilters?.slice(2)

  const [isPasteDialogOpen, setIsPasteDialogOpen] = React.useState(false)

  return (
    <Stack gap={3}>
      <Stack
        gap={4}
        p={3}
        rounded="lg"
        bg="bg-card"
        className="border border-border/40 shadow-sm sm:flex-row sm:items-center sm:justify-between"
      >
        <HStack gap={2} className="flex-wrap">
          {onSearchChange && (
            <ActionTooltip label={`Searching for: ${search || 'nothing'}`}>
              <div className="relative group mr-2">
                <div
                  className={cn(
                    'flex items-center gap-2 h-8 w-48 rounded-md border border-border/60 bg-background px-3 transition-all duration-200',
                    'focus-within:border-ring focus-within:ring-2 focus-within:ring-ring/20 focus-within:w-64',
                    'hover:border-border',
                  )}
                >
                  <HugeiconsIcon
                    icon={Search01Icon}
                    className="size-4 text-muted-foreground shrink-0"
                  />
                  <input
                    type="text"
                    value={search ?? ''}
                    onChange={(e) => onSearchChange(e.target.value)}
                    placeholder={searchPlaceholder}
                    className="flex-1 bg-transparent text-sm outline-none placeholder:text-muted-foreground/60"
                  />
                  {search && (
                    <button
                      type="button"
                      onClick={() => onSearchChange('')}
                      className="shrink-0 rounded-sm p-0.5 text-muted-foreground hover:text-foreground transition-colors"
                    >
                      <HugeiconsIcon icon={Cancel01Icon} size={14} />
                    </button>
                  )}
                </div>
              </div>
            </ActionTooltip>
          )}

          {visibleFacetedFilters?.map((filter) => (
            <DataTableFacetedFilterComponent
              key={filter.columnId}
              column={table.getColumn(filter.columnId)}
              title={filter.title}
              options={filter.options}
            />
          ))}

          {overflowFacetedFilters && overflowFacetedFilters.length > 0 && (
            <DropdownMenu>
              <ActionTooltip label="More filters">
                <DropdownMenuTrigger
                  render={
                    <Button
                      variant="outline"
                      size="sm"
                      className="h-8 border-dashed"
                    >
                      <HugeiconsIcon
                        icon={FilterIcon}
                        className="mr-2 h-4 w-4"
                      />
                      More Filters
                    </Button>
                  }
                />
              </ActionTooltip>
              <DropdownMenuContent align="start" className="w-56">
                <DropdownMenuGroup>
                  <DropdownMenuLabel>Additional Filters</DropdownMenuLabel>
                  <DropdownMenuSeparator />
                  {overflowFacetedFilters.map((filter) => (
                    <DropdownMenuItem
                      key={filter.columnId}
                      onSelect={(e) => e.preventDefault()}
                      className="p-0"
                    >
                      <DataTableFacetedFilterComponent
                        column={table.getColumn(filter.columnId)}
                        title={filter.title}
                        options={filter.options}
                        variant="ghost"
                        className="w-full justify-start h-9 border-none rounded-none px-2"
                      />
                    </DropdownMenuItem>
                  ))}
                </DropdownMenuGroup>
              </DropdownMenuContent>
            </DropdownMenu>
          )}
        </HStack>

        <HStack gap={2} className="flex-wrap">
          {context.onAdd && (
            <Button
              variant="default"
              size="sm"
              onClick={context.onAdd}
              className="bg-primary hover:bg-primary/90 text-primary-foreground"
            >
              <HugeiconsIcon icon={Add01Icon} className="size-4" />
              <span>{context.onAddLabel || 'Add'}</span>
            </Button>
          )}

          {(onImportCSV || onImportJSON) && (
            <DropdownMenu>
              <ActionTooltip label="Import data from various sources">
                <DropdownMenuTrigger
                  render={
                    <Button variant="outline" size="sm" className="h-8">
                      <HugeiconsIcon icon={UploadIcon} className="size-4" />
                      <span>Import</span>
                    </Button>
                  }
                />
              </ActionTooltip>
              <DropdownMenuContent align="end" className="w-48">
                <DropdownMenuGroup>
                  <DropdownMenuLabel>Import Options</DropdownMenuLabel>
                  {onImportCSV && (
                    <DropdownMenuItem onClick={importCsv}>
                      <HugeiconsIcon
                        icon={UploadIcon}
                        className="size-4 mr-2"
                      />
                      Import CSV
                    </DropdownMenuItem>
                  )}
                  {onImportJSON && (
                    <DropdownMenuItem onClick={importJson}>
                      <HugeiconsIcon
                        icon={FileUploadIcon}
                        className="size-4 mr-2"
                      />
                      Import JSON
                    </DropdownMenuItem>
                  )}
                  {allowPaste && (
                    <DropdownMenuItem
                      onClick={() => setIsPasteDialogOpen(true)}
                    >
                      <HugeiconsIcon
                        icon={Copy01Icon}
                        className="size-4 mr-2"
                      />
                      Paste Data
                    </DropdownMenuItem>
                  )}
                </DropdownMenuGroup>
              </DropdownMenuContent>
            </DropdownMenu>
          )}

          <DropdownMenu>
            <ActionTooltip label="Export data to various formats">
              <DropdownMenuTrigger
                render={
                  <Button variant="outline" size="sm">
                    <HugeiconsIcon icon={DownloadIcon} className="size-4" />
                    <span>Export</span>
                  </Button>
                }
              />
            </ActionTooltip>
            <DropdownMenuContent align="end" className="w-48">
              <DropdownMenuGroup>
                <DropdownMenuLabel>Export Current View</DropdownMenuLabel>
                <DropdownMenuItem onClick={() => exportCsv()}>
                  CSV (.csv)
                </DropdownMenuItem>
                <DropdownMenuItem onClick={() => exportPdf()}>
                  PDF (.pdf)
                </DropdownMenuItem>
                {selectionCount > 0 && (
                  <>
                    <DropdownMenuSeparator />
                    <DropdownMenuLabel>Export Selected</DropdownMenuLabel>
                    <DropdownMenuItem
                      onClick={() => exportCsv({ selectedOnly: true })}
                    >
                      Selected as CSV
                    </DropdownMenuItem>
                    <DropdownMenuItem
                      onClick={() => exportPdf({ selectedOnly: true })}
                    >
                      Selected as PDF
                    </DropdownMenuItem>
                  </>
                )}
                {onFetchFullData && (
                  <>
                    <DropdownMenuSeparator />
                    <DropdownMenuLabel>Export All Data</DropdownMenuLabel>
                    <DropdownMenuItem
                      onClick={() => exportCsv({ allData: true })}
                    >
                      All as CSV
                    </DropdownMenuItem>
                    <DropdownMenuItem
                      onClick={() => exportPdf({ allData: true })}
                    >
                      All as PDF
                    </DropdownMenuItem>
                  </>
                )}
                <DropdownMenuSeparator />
                <DropdownMenuItem onClick={generateTemplateCsv}>
                  <HugeiconsIcon
                    icon={FileDownloadIcon}
                    className="size-4 mr-2"
                  />
                  Download Template
                </DropdownMenuItem>
              </DropdownMenuGroup>
            </DropdownMenuContent>
          </DropdownMenu>

          {extraActions}

          <div className="hidden">
            {(onImportCSV || onImportJSON) && allowPaste && (
              <DataTablePasteDialog
                onImportCSV={onImportCSV}
                onImportJSON={onImportJSON}
                open={isPasteDialogOpen}
                onOpenChange={setIsPasteDialogOpen}
              />
            )}
          </div>

          <DropdownMenu>
            <ActionTooltip label="Table view settings and column visibility">
              <DropdownMenuTrigger
                render={
                  <Button variant="outline" size="sm">
                    <HugeiconsIcon icon={DensityIcon} className="size-4" />
                    <span>View</span>
                  </Button>
                }
              />
            </ActionTooltip>
            <DropdownMenuContent align="end" className="w-48">
              <DropdownMenuGroup>
                <DropdownMenuLabel>Row Density</DropdownMenuLabel>
                <DropdownMenuRadioGroup
                  value={density}
                  onValueChange={handleDensityChange}
                >
                  <DropdownMenuRadioItem value="compact">
                    Compact
                  </DropdownMenuRadioItem>
                  <DropdownMenuRadioItem value="comfortable">
                    Comfortable
                  </DropdownMenuRadioItem>
                </DropdownMenuRadioGroup>
                <DropdownMenuSeparator />
                <DropdownMenuLabel>Columns</DropdownMenuLabel>
                {baseColumns
                  .filter((column) => {
                    const id = getColId(column)
                    return id && id !== 'row-actions'
                  })
                  .map((column) => {
                    const id = getColId(column) || ''
                    return (
                      <DropdownMenuCheckboxItem
                        key={id}
                        checked={columnVisibility[id] !== false}
                        onCheckedChange={(checked) =>
                          table.getColumn(id)?.toggleVisibility(!!checked)
                        }
                      >
                        {getColHeader(column)}
                      </DropdownMenuCheckboxItem>
                    )
                  })}
                <DropdownMenuSeparator />
                <DropdownMenuItem onClick={() => table.resetColumnVisibility()}>
                  Reset visibility
                </DropdownMenuItem>
              </DropdownMenuGroup>
            </DropdownMenuContent>
          </DropdownMenu>
        </HStack>
      </Stack>

      {selectionCount > 0 && bulkActions && (
        <Box
          p={2}
          rounded="lg"
          bg="bg-primary/5"
          className="border border-primary/20 animate-in fade-in slide-in-from-top-1"
        >
          <HStack justify="between">
            <HStack gap={2}>
              <Badge variant="secondary" className="rounded-full">
                {selectionCount}
              </Badge>
              <Text size="sm" className="font-medium">
                rows selected
              </Text>
            </HStack>
            <HStack gap={2}>
              {bulkActions({ selectedRows, table })}
              <ActionTooltip label="Deselect all rows">
                <Button
                  variant="ghost"
                  size="sm"
                  onClick={() => table.resetRowSelection()}
                >
                  Cancel
                </Button>
              </ActionTooltip>
            </HStack>
          </HStack>
        </Box>
      )}
    </Stack>
  )
}

interface DataTableFacetedFilterProps<TData, TValue> {
  column?: Column<TData, TValue>
  title?: string
  options: Array<{
    label: string
    value: string
    icon?: React.ComponentType<{ className?: string }>
  }>
  variant?: 'outline' | 'ghost'
  className?: string
}

function DataTableFacetedFilterComponent<TData, TValue>({
  column,
  title,
  options,
  variant = 'outline',
  className,
}: DataTableFacetedFilterProps<TData, TValue>) {
  const facets = column?.getFacetedUniqueValues()
  const filterValue = column?.getFilterValue()

  const selectedValues = new Set<string>()
  if (Array.isArray(filterValue)) {
    filterValue.forEach((v) => {
      if (typeof v === 'string') selectedValues.add(v)
    })
  }

  const getFacetCount = (value: string): number => {
    if (facets instanceof Map) {
      const count = facets.get(value)
      return typeof count === 'number' ? count : 0
    }
    return 0
  }

  return (
    <Popover>
      <ActionTooltip label={`Filter by ${title}`}>
        <PopoverTrigger
          render={
            <Button
              variant={variant}
              size="sm"
              className={cn(
                'h-8',
                variant === 'outline' && 'border-dashed',
                className,
              )}
            >
              <HugeiconsIcon icon={FilterIcon} className="mr-2 h-4 w-4" />
              {title}
              {selectedValues.size > 0 && (
                <>
                  <Separator orientation="vertical" className="mx-2 h-4" />
                  <Badge
                    variant="secondary"
                    className="rounded-sm px-1 font-normal lg:hidden"
                  >
                    {selectedValues.size}
                  </Badge>
                  <div className="hidden space-x-1 lg:flex">
                    {selectedValues.size > 2 ? (
                      <Badge
                        variant="secondary"
                        className="rounded-sm px-1 font-normal"
                      >
                        {selectedValues.size} selected
                      </Badge>
                    ) : (
                      options
                        .filter((option) => selectedValues.has(option.value))
                        .map((option) => (
                          <Badge
                            variant="secondary"
                            key={option.value}
                            className="rounded-sm px-1 font-normal"
                          >
                            {option.label}
                          </Badge>
                        ))
                    )}
                  </div>
                </>
              )}
            </Button>
          }
        />
      </ActionTooltip>
      <PopoverContent className="w-[200px] p-0" align="start">
        <Command>
          <CommandInput placeholder={title} />
          <CommandList>
            <CommandEmpty>No results found.</CommandEmpty>
            <CommandGroup>
              {options.map((option) => {
                const isSelected = selectedValues.has(option.value)
                const count = getFacetCount(option.value)
                return (
                  <CommandItem
                    key={option.value}
                    onSelect={() => {
                      if (isSelected) {
                        selectedValues.delete(option.value)
                      } else {
                        selectedValues.add(option.value)
                      }
                      const filterValues = Array.from(selectedValues)
                      column?.setFilterValue(
                        filterValues.length ? filterValues : undefined,
                      )
                    }}
                  >
                    <div
                      className={cn(
                        'mr-2 flex h-4 w-4 items-center justify-center rounded-sm border border-primary',
                        isSelected
                          ? 'bg-primary text-primary-foreground'
                          : 'opacity-50 [&_svg]:invisible',
                      )}
                    >
                      <HugeiconsIcon icon={Tick02Icon} className="h-4 w-4" />
                    </div>
                    {option.icon && (
                      <option.icon className="mr-2 h-4 w-4 text-muted-foreground" />
                    )}
                    <span>{option.label}</span>
                    {count > 0 && (
                      <span className="ml-auto flex h-4 w-4 items-center justify-center font-mono text-xs">
                        {String(getFacetCount(option.value))}
                      </span>
                    )}
                  </CommandItem>
                )
              })}
            </CommandGroup>
            {selectedValues.size > 0 && (
              <>
                <CommandSeparator />
                <CommandGroup>
                  <CommandItem
                    onSelect={() => column?.setFilterValue(undefined)}
                    className="justify-center text-center"
                  >
                    Clear filters
                  </CommandItem>
                </CommandGroup>
              </>
            )}
          </CommandList>
        </Command>
      </PopoverContent>
    </Popover>
  )
}
