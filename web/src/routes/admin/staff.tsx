import { createFileRoute } from '@tanstack/react-router'
import {
  keepPreviousData,
  useQuery,
  useQueryClient,
} from '@tanstack/react-query'
import { HugeiconsIcon } from '@hugeicons/react'
import {
  FileExportIcon,
  FileImportIcon,
  GridViewIcon,
  ListViewIcon,
  Search01Icon,
  Table01Icon,
  User02Icon,
  UserAdd01Icon,
} from '@hugeicons/core-free-icons'

import * as React from 'react'

import type { Staff } from '@/features/staff/types'
import {
  StaffContextMenuItems,
  StaffGridView,
  StaffListContainer,
  StaffModals,
  StaffToolbar,
  getStaffColumns,
} from '@/features/staff/components'
import {
  getStaffQueryOptions,
  useBulkDeleteStaff,
  useBulkImportStaff,
  useCreateStaff,
  useDeleteStaff,
} from '@/features/staff/api'

import { Box, HStack, Heading, Stack, Text } from '@/components/primitives'
import { Button } from '@/components/ui/button'
import { Tabs, TabsList, TabsTrigger } from '@/components/ui/tabs'
import {
  Card,
  CardContent,
  CardDescription,
  CardHeader,
  CardTitle,
} from '@/components/ui/card'
import {
  InputGroup,
  InputGroupAddon,
  InputGroupInput,
} from '@/components/ui/input-group'
import {
  DropdownMenu,
  DropdownMenuContent,
  DropdownMenuItem,
  DropdownMenuTrigger,
} from '@/components/ui/dropdown-menu'
import { useDebounce } from '@/hooks/use-debounce'
import { authClient } from '@/lib/clients'
import { useStaffSearchParams } from '@/features/staff/search-params'

interface StaffSearch {
  page: number
  limit: number
  sort: Array<{ id: string; desc: boolean }>
  search: string
}

export const Route = createFileRoute('/admin/staff')({
  component: StaffDashboard,
  validateSearch: (search: Record<string, unknown>): StaffSearch => {
    return {
      page: typeof search.page === 'number' ? search.page : 1,
      limit: typeof search.limit === 'number' ? search.limit : 10,
      sort: Array.isArray(search.sort) 
        ? search.sort.filter((item): item is { id: string; desc: boolean } =>
            typeof item === 'object' && item !== null && 'id' in item && 'desc' in item
          )
        : [],
      search: typeof search.search === 'string' ? search.search : '',
    }
  },
})

// Stats Card Component
function StatsCard({
  icon,
  label,
  value,
  description,
}: {
  icon: React.ReactNode
  label: string
  value: number
  description?: string
}) {
  return (
    <Card>
      <CardHeader className="flex flex-row items-center justify-between space-y-0 pb-2">
        <CardTitle className="text-sm font-medium">{label}</CardTitle>
        {icon}
      </CardHeader>
      <CardContent>
        <div className="text-2xl font-bold">{value}</div>
        {description && (
          <Text size="xs" muted>
            {description}
          </Text>
        )}
      </CardContent>
    </Card>
  )
}

function StaffDashboard() {
  const queryClient = useQueryClient()
  const { page, limit, sort, search: urlSearch, setPage, setLimit, setSort, setSearch, staffTypeFilter, setStaffTypeFilter } = useStaffSearchParams()
  const [searchQuery, setSearchQuery] = React.useState(urlSearch ?? '')
  const debouncedSearch = useDebounce(searchQuery, 300)

  React.useEffect(() => {
    setSearchQuery(urlSearch ?? '')
  }, [urlSearch])

  React.useEffect(() => {
    if (debouncedSearch !== urlSearch) {
      setSearch(debouncedSearch || null)
    }
  }, [debouncedSearch, urlSearch, setSearch])

  const [staffToDelete, setStaffToDelete] = React.useState<string | null>(null)
  const [isBulkDeleteOpen, setIsBulkDeleteOpen] = React.useState(false)
  const [isCreateStaffOpen, setIsCreateStaffOpen] = React.useState(false)
  const [staffToEdit, setStaffToEdit] = React.useState<Staff | null>(null)
  const [viewMode, setViewMode] = React.useState<'table' | 'grid'>('table')

  // Sync activeTab with URL staff_type filter
  const activeTab = React.useMemo<'all' | 'teaching' | 'non-teaching' | 'administrative'>(() => {
    if (!staffTypeFilter || staffTypeFilter === 'all') return 'all'
    if (staffTypeFilter === 'Teaching') return 'teaching'
    if (staffTypeFilter === 'NonTeaching') return 'non-teaching'
    if (staffTypeFilter === 'Administrative') return 'administrative'
    return 'all'
  }, [staffTypeFilter])

  const setActiveTab = React.useCallback((tab: 'all' | 'teaching' | 'non-teaching' | 'administrative') => {
    if (tab === 'all') setStaffTypeFilter(null)
    else if (tab === 'teaching') setStaffTypeFilter('Teaching')
    else if (tab === 'non-teaching') setStaffTypeFilter('NonTeaching')
    else if (tab === 'administrative') setStaffTypeFilter('Administrative')
  }, [setStaffTypeFilter])

  // Determine staff_type filter based on active tab
  const staffTypeFilterValue = React.useMemo(() => {
    if (activeTab === 'all') return undefined
    if (activeTab === 'teaching') return 'Teaching' as const
    if (activeTab === 'non-teaching') return 'NonTeaching' as const
    if (activeTab === 'administrative') return 'Administrative' as const
    return undefined
  }, [activeTab])

  // Fetch staff with server-side pagination and filtering
  const staffQuery = useQuery({
    ...getStaffQueryOptions({
      client: authClient,
      query: {
        page: page ?? 1,
        limit: limit ?? 10,
        search: debouncedSearch || undefined,
        sort_by: sort[0]?.id ?? 'created_at',
        sort_order: sort[0]?.desc ? 'desc' : 'asc',
        staff_type: staffTypeFilterValue,
      },
    }),
    placeholderData: keepPreviousData,
  })

  // Fetch all staff for stats calculation only
  const allStaffQuery = useQuery({
    ...getStaffQueryOptions({
      client: authClient,
      query: {
        page: 1,
        limit: 10000,
        sort_by: 'created_at',
        sort_order: 'asc',
      },
    }),
    placeholderData: keepPreviousData,
  })

  const deleteStaff = useDeleteStaff()
  const bulkDeleteStaff = useBulkDeleteStaff()
  const createStaff = useCreateStaff()
  const bulkImportStaff = useBulkImportStaff()

  const [rowSelection, setRowSelection] = React.useState<
    Record<string, boolean>
  >({})

  // Calculate stats from all staff data
  const totalStaff = allStaffQuery.data?.total ?? 0
  const teachingCount = allStaffQuery.data?.data?.filter(
    (s) => s.staff_type === 'Teaching',
  ).length ?? 0
  const nonTeachingCount = allStaffQuery.data?.data?.filter(
    (s) => s.staff_type === 'NonTeaching',
  ).length ?? 0
  const administrativeCount = allStaffQuery.data?.data?.filter(
    (s) => s.staff_type === 'Administrative',
  ).length ?? 0

  const facetedFilters = React.useMemo(
    () => [
      {
        columnId: 'staff_type',
        title: 'Staff Type',
        options: [
          { label: 'Teaching', value: 'Teaching' },
          { label: 'Non-Teaching', value: 'NonTeaching' },
          { label: 'Administrative', value: 'Administrative' },
        ],
      },
      {
        columnId: 'employment_status',
        title: 'Employment Status',
        options: [
          { label: 'Active', value: 'Active' },
          { label: 'Inactive', value: 'Inactive' },
          { label: 'On Leave', value: 'OnLeave' },
          { label: 'Terminated', value: 'Terminated' },
          { label: 'Retired', value: 'Retired' },
        ],
      },
    ],
    [],
  )

  const fetchFullData = React.useCallback(async () => {
    const options = getStaffQueryOptions({
      client: authClient,
      query: {
        page: 1,
        limit: 10000,
        search: debouncedSearch || undefined,
        sort_by: sort[0]?.id ?? 'created_at',
        sort_order: sort[0]?.desc ? 'desc' : 'asc',
        staff_type: staffTypeFilterValue,
      },
    })

    if (!options.queryFn) return []
    const result = await options.queryFn({
      client: queryClient,
      queryKey: options.queryKey,
      signal: new AbortController().signal,
      meta: undefined,
    })

    return result.data
  }, [debouncedSearch, sort, staffTypeFilterValue, queryClient])

  const columns = getStaffColumns({
    onEdit: setStaffToEdit,
    onDelete: setStaffToDelete,
    onViewProfile: undefined,
  })

  return (
    <Stack gap={6} p={8} className="h-full overflow-y-auto">
      {/* Header Section */}
      <Stack gap={4}>
        <HStack justify="between" align="center">
          <Stack gap={1}>
            <Heading size="h2">Staff Management</Heading>
            <Text muted as="p">
              Manage staff members, their profiles, and employment details
            </Text>
          </Stack>
          <HStack gap={2}>
            <Button
              variant="outline"
              size="sm"
              onClick={() => {
                /* Export functionality */
              }}
            >
              <HugeiconsIcon icon={FileExportIcon} className="size-4 mr-2" />
              Export
            </Button>
            <Button
              variant="outline"
              size="sm"
              onClick={() => {
                /* Import functionality */
              }}
            >
              <HugeiconsIcon icon={FileImportIcon} className="size-4 mr-2" />
              Import
            </Button>
            <Button size="sm" onClick={() => setIsCreateStaffOpen(true)}>
              <HugeiconsIcon icon={UserAdd01Icon} className="size-4 mr-2" />
              Add Staff
            </Button>
          </HStack>
        </HStack>

        {/* Search Bar */}
        <HStack gap={4}>
          <InputGroup className="max-w-sm">
            <InputGroupInput
              placeholder="Search staff..."
              value={searchQuery}
              onChange={(e) => setSearchQuery(e.target.value)}
              className="h-10"
            />
            <InputGroupAddon align="inline-start">
              <HugeiconsIcon icon={Search01Icon} className="size-4" />
            </InputGroupAddon>
          </InputGroup>
        </HStack>
      </Stack>

      {/* Stats Cards */}
      <div className="grid gap-4 md:grid-cols-2 lg:grid-cols-4">
        <StatsCard
          icon={
            <HugeiconsIcon
              icon={User02Icon}
              className="size-4 text-muted-foreground"
            />
          }
          label="Total Staff"
          value={totalStaff}
          description="All staff members"
        />
        <StatsCard
          icon={
            <HugeiconsIcon
              icon={User02Icon}
              className="size-4 text-blue-500"
            />
          }
          label="Teaching"
          value={teachingCount}
          description="Teaching staff"
        />
        <StatsCard
          icon={
            <HugeiconsIcon
              icon={User02Icon}
              className="size-4 text-purple-500"
            />
          }
          label="Non-Teaching"
          value={nonTeachingCount}
          description="Non-teaching staff"
        />
        <StatsCard
          icon={
            <HugeiconsIcon
              icon={User02Icon}
              className="size-4 text-orange-500"
            />
          }
          label="Administrative"
          value={administrativeCount}
          description="Administrative staff"
        />
      </div>

      {/* Main Content */}
      <Card className="flex-1 flex flex-col min-h-0">
        <CardHeader className="border-b">
          <HStack justify="between" align="center">
            <Stack gap={1}>
              <CardTitle>Staff Members</CardTitle>
              <CardDescription>
                Manage staff accounts and their employment details
              </CardDescription>
            </Stack>
            <HStack gap={2}>
              <Tabs
                value={activeTab}
                onValueChange={(v) => {
                  if (v === 'all') setActiveTab('all')
                  else if (v === 'teaching') setActiveTab('teaching')
                  else if (v === 'non-teaching') setActiveTab('non-teaching')
                  else if (v === 'administrative') setActiveTab('administrative')
                }}
              >
                <TabsList>
                  <TabsTrigger value="all">All</TabsTrigger>
                  <TabsTrigger value="teaching">Teaching</TabsTrigger>
                  <TabsTrigger value="non-teaching">Non-Teaching</TabsTrigger>
                  <TabsTrigger value="administrative">Administrative</TabsTrigger>
                </TabsList>
              </Tabs>
              <DropdownMenu>
                <DropdownMenuTrigger render={
                  <Button variant="outline" size="sm">
                    <HugeiconsIcon
                      icon={Table01Icon}
                      className="size-4 mr-2"
                    />
                    View
                  </Button>
                }></DropdownMenuTrigger>
                <DropdownMenuContent align="end">
                  <DropdownMenuItem
                    onClick={() => setViewMode('table')}
                    className={viewMode === 'table' ? 'bg-muted' : ''}
                  >
                    <HugeiconsIcon
                      icon={ListViewIcon}
                      className="size-4 mr-2"
                    />
                    List
                  </DropdownMenuItem>
                  <DropdownMenuItem
                    onClick={() => setViewMode('grid')}
                    className={viewMode === 'grid' ? 'bg-muted' : ''}
                  >
                    <HugeiconsIcon
                      icon={GridViewIcon}
                      className="size-4 mr-2"
                    />
                    Grid
                  </DropdownMenuItem>
                </DropdownMenuContent>
              </DropdownMenu>
            </HStack>
          </HStack>
        </CardHeader>

        <Box className="flex-1 min-h-0 overflow-y-auto">
          {viewMode === 'table' ? (
            <StaffListContainer
              staffQuery={staffQuery}
              limit={limit ?? 10}
              columns={columns}
              rowSelection={rowSelection}
              setRowSelection={setRowSelection}
              facetedFilters={facetedFilters}
              onAdd={() => setIsCreateStaffOpen(true)}
              onAddLabel="Add Staff"
              onFetchFullData={fetchFullData}
              onImportCSV={(rows) => bulkImportStaff.mutate(rows)}
              onImportJSON={(rows) => bulkImportStaff.mutate(rows)}
              bulkActions={({ selectedRows }) => {
                return (
                  <StaffToolbar
                    selectedStaff={new Set(selectedRows.map((r) => r.id))}
                    floating={false}
                    onBulkDelete={() => setIsBulkDeleteOpen(true)}
                    onBulkEdit={() => {}}
                    onCreateStaff={() => setIsCreateStaffOpen(true)}
                  />
                )
              }}
              contextMenuItems={(row) => {
                return (
                  <StaffContextMenuItems
                    staff={row}
                    onEdit={setStaffToEdit}
                    onDelete={setStaffToDelete}
                  />
                )
              }}
              onPageChange={(p) => setPage(p)}
              onLimitChange={(l) => setLimit(l)}
              onSortChange={(s) => setSort(s)}
              onSearchChange={(s) => setSearch(s || null)}
            />
          ) : (
            <Box p={4}>
              <StaffGridView
                staff={staffQuery.data?.data ?? []}
                limit={limit ?? 10}
                isLoading={staffQuery.isFetching}
                onEdit={setStaffToEdit}
                onDelete={setStaffToDelete}
                onCreateStaff={() => setIsCreateStaffOpen(true)}
              />
            </Box>
          )}
        </Box>
      </Card>

      {/* Modals */}
      <StaffModals
        staffToDelete={staffToDelete}
        setStaffToDelete={setStaffToDelete}
        onDeleteConfirm={(id: string) =>
          deleteStaff.mutate(
            { id },
            {
              onSuccess: () => {
                setStaffToDelete(null)
              },
            },
          )
        }
        isBulkDeleteOpen={isBulkDeleteOpen}
        setIsBulkDeleteOpen={setIsBulkDeleteOpen}
        onBulkDeleteConfirm={() =>
          bulkDeleteStaff.mutate(
            undefined,
            {
              onSuccess: () => {
                setRowSelection({})
                setIsBulkDeleteOpen(false)
              },
            },
          )
        }
        staffToEdit={staffToEdit}
        setStaffToEdit={setStaffToEdit}
        onEditConfirm={() => {
          setStaffToEdit(null)
        }}
        isCreateStaffOpen={isCreateStaffOpen}
        setIsCreateStaffOpen={setIsCreateStaffOpen}
        onCreateConfirm={(data) =>
          createStaff.mutate(
            {
              body: {
                name: data.name,
                created_at: new Date().toISOString(),
                id: crypto.randomUUID(),
                updated_at: new Date().toISOString(),
              },
            },
            {
              onSuccess: () => {
                setIsCreateStaffOpen(false)
              },
            },
          )
        }
        isSubmitting={createStaff.isPending}
      />
    </Stack>
  )
}
