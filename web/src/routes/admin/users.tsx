/* eslint-disable @typescript-eslint/no-unsafe-assignment, @typescript-eslint/consistent-type-assertions */
import { createFileRoute } from '@tanstack/react-router'
import {
  keepPreviousData,
  useQuery,
  useQueryClient,
} from '@tanstack/react-query'
import { HugeiconsIcon } from '@hugeicons/react'
import {
  ExportIcon,
  Grid01Icon,
  ImportIcon,
  List01Icon,
  LockIcon,
  Search01Icon,
  TablePropertiesIcon,
  User01Icon,
  UserCheckIcon,
  UserPlusIcon,
  UserXIcon,
} from '@hugeicons/core-free-icons'

import * as React from 'react'

import { UserCreateDialog } from '../../features/users/components/user-create-dialog'
import { UserModals } from '../../features/users/components/user-modals'
import { UserToolbar } from '../../features/users/components/user-toolbar'
import {
  UserContextMenuItems,
  getUserColumns,
} from '../../features/users/components/user-table-columns'
import { UsersListContainer } from '../../features/users/components/users-list-container'
import { UserGridView } from '../../features/users/components/user-grid-view'

import type { UserResponse } from '@/lib/api'
import { Box, HStack, Heading, Stack, Text } from '@/components/primitives'
import {
  getUserProfilesQueryOptions,
  getUsersQueryOptions,
  useBulkDeleteUsers,
  useBulkImportUsers,
  useDeleteUser,
  useRegisterUser,
} from '@/features/users/api'
import { useUsersSearchParams } from '@/features/users/search-params'
import { authClient } from '@/lib/clients'
import { Button } from '@/components/ui/button'
import { Tabs, TabsContent, TabsList, TabsTrigger } from '@/components/ui/tabs'
import {
  Card,
  CardContent,
  CardDescription,
  CardHeader,
  CardTitle,
} from '@/components/ui/card'
import { Badge } from '@/components/ui/badge'
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

export const Route = createFileRoute('/admin/users')({
  component: UsersDashboard,
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

function UsersDashboard() {
  const queryClient = useQueryClient()
  const { page, limit, search, sort } = useUsersSearchParams()
  const [searchQuery, setSearchQuery] = React.useState('')
  const debouncedSearch = useDebounce(searchQuery, 300)

  const [userToDelete, setUserToDelete] = React.useState<string | null>(null)
  const [isBulkDeleteOpen, setIsBulkDeleteOpen] = React.useState(false)
  const [isCreateUserOpen, setIsCreateUserOpen] = React.useState(false)
  const [userToLock, setUserToLock] = React.useState<UserResponse | null>(null)
  const [userToEdit, setUserToEdit] = React.useState<UserResponse | null>(null)
  const [userToManagePermissions, setUserToManagePermissions] =
    React.useState<UserResponse | null>(null)
  const [showProfilePictures, setShowProfilePictures] = React.useState(true)
  const [activeTab, setActiveTab] = React.useState<
    'all' | 'verified' | 'unverified' | 'locked'
  >('all')
  const [viewMode, setViewMode] = React.useState<'table' | 'grid'>('table')

  // Fetch users
  const usersQuery = useQuery({
    ...getUsersQueryOptions({
      client: authClient,
      query: {
        page: page ?? 1,
        limit: limit ?? 10,
        search: debouncedSearch || undefined,
        sort_by: sort?.[0]?.id ?? 'created_at',
        sort_order: sort?.[0]?.desc ? 'desc' : 'asc',
      },
    }),
    placeholderData: keepPreviousData,
  })

  // Fetch user profiles for stats
  const profilesQuery = useQuery({
    ...getUserProfilesQueryOptions({
      client: authClient,
      query: {
        limit: 1000,
      },
    }),
  })

  const deleteUser = useDeleteUser()
  const bulkDeleteUsers = useBulkDeleteUsers()
  const createUser = useRegisterUser()
  const bulkImportUsers = useBulkImportUsers()

  const [rowSelection, setRowSelection] = React.useState<
    Record<string, boolean>
  >({})

  // Calculate stats
  const usersData = usersQuery.data?.data ?? []
  const totalUsers = usersQuery.data?.total ?? 0
  const verifiedCount = usersQuery.data?.data?.filter(
    (u) => u.is_verified,
  ).length
  const unverifiedCount = usersQuery.data?.data?.filter(
    (u) => !u.is_verified,
  ).length
  const lockedCount = usersQuery.data?.data?.filter(
    (u) => u.lockout_until,
  ).length

  const facetedFilters = React.useMemo(
    () => [
      {
        columnId: 'is_verified',
        title: 'Status',
        options: [
          { label: 'Verified', value: 'true' },
          { label: 'Unverified', value: 'false' },
        ],
      },
      {
        columnId: 'auth_method',
        title: 'Auth Method',
        options: [
          { label: 'Password', value: 'password' },
          { label: 'Google', value: 'google' },
          { label: 'GitHub', value: 'github' },
        ],
      },
    ],
    [],
  )

  const fetchFullData = React.useCallback(async () => {
    const options = getUsersQueryOptions({
      client: authClient,
      query: {
        page: 1,
        limit: 1000,
        search: debouncedSearch || undefined,
        sort_by: sort?.[0]?.id ?? 'created_at',
        sort_order: sort?.[0]?.desc ? 'desc' : 'asc',
      },
    })

    if (!options.queryFn) return []
    const result = await options.queryFn({
      client: queryClient,
      queryKey: options.queryKey,
      signal: new AbortController().signal,
      meta: undefined,
    })
     
    return (result as unknown as { data: Array<UserResponse> }).data
  }, [debouncedSearch, sort, queryClient])

  const columns = getUserColumns({
    setUserToDelete,
    setUserToEdit,
    onToggleLock: (user: UserResponse) => {
      if (user.lockout_until) {
        setUserToLock(null)
      } else {
        setUserToLock(user)
      }
    },
    setUserToManagePermissions,
    showProfilePictures,
  })

  // Filter users based on active tab
  const filteredUsers = React.useMemo(() => {
    if (!usersQuery.data?.data) return []
    switch (activeTab) {
      case 'verified':
        return usersQuery.data.data.filter((u) => u.is_verified)
      case 'unverified':
        return usersQuery.data.data.filter((u) => !u.is_verified)
      case 'locked':
        return usersQuery.data.data.filter((u) => u.lockout_until)
      default:
        return usersQuery.data.data
    }
  }, [usersQuery.data?.data, activeTab])

  return (
    <Stack gap={6} p={8} className="h-full overflow-y-auto">
      {/* Header Section */}
      <Stack gap={4}>
        <HStack justify="between" align="center">
          <Stack gap={1}>
            <Heading size="h2">User Management</Heading>
            <Text muted as="p">
              Manage user accounts, profiles, and permissions
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
              <HugeiconsIcon icon={ExportIcon} className="size-4 mr-2" />
              Export
            </Button>
            <Button
              variant="outline"
              size="sm"
              onClick={() => {
                /* Import functionality */
              }}
            >
              <HugeiconsIcon icon={ImportIcon} className="size-4 mr-2" />
              Import
            </Button>
            <Button size="sm" onClick={() => setIsCreateUserOpen(true)}>
              <HugeiconsIcon icon={UserPlusIcon} className="size-4 mr-2" />
              Add User
            </Button>
          </HStack>
        </HStack>

        {/* Search Bar */}
        <HStack gap={4}>
          <InputGroup className="max-w-sm">
            <InputGroupInput
              placeholder="Search users..."
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
              icon={User01Icon}
              className="size-4 text-muted-foreground"
            />
          }
          label="Total Users"
          value={totalUsers}
          description="All user accounts"
        />
        <StatsCard
          icon={
            <HugeiconsIcon
              icon={UserCheckIcon}
              className="size-4 text-green-500"
            />
          }
          label="Verified"
          value={verifiedCount ?? 0}
          description="Verified accounts"
        />
        <StatsCard
          icon={
            <HugeiconsIcon
              icon={UserXIcon}
              className="size-4 text-yellow-500"
            />
          }
          label="Unverified"
          value={unverifiedCount ?? 0}
          description="Pending verification"
        />
        <StatsCard
          icon={
            <HugeiconsIcon icon={LockIcon} className="size-4 text-red-500" />
          }
          label="Locked"
          value={lockedCount ?? 0}
          description="Account locked"
        />
      </div>

      {/* Main Content */}
      <Card className="flex-1 flex flex-col min-h-0">
        <CardHeader className="border-b">
          <HStack justify="between" align="center">
            <Stack gap={1}>
              <CardTitle>User Accounts</CardTitle>
              <CardDescription>
                Manage user accounts and their access permissions
              </CardDescription>
            </Stack>
            <HStack gap={2}>
              <Tabs
                value={activeTab}
                onValueChange={(v) => {
                  setActiveTab(
                    v as 'all' | 'verified' | 'unverified' | 'locked',
                  )
                }}
              >
                <TabsList>
                  <TabsTrigger value="all">All</TabsTrigger>
                  <TabsTrigger value="verified">Verified</TabsTrigger>
                  <TabsTrigger value="unverified">Unverified</TabsTrigger>
                  <TabsTrigger value="locked">Locked</TabsTrigger>
                </TabsList>
              </Tabs>
              <DropdownMenu>
                <DropdownMenuTrigger asChild>
                  <Button variant="outline" size="sm">
                    <HugeiconsIcon
                      icon={TablePropertiesIcon}
                      className="size-4 mr-2"
                    />
                    View
                  </Button>
                </DropdownMenuTrigger>
                <DropdownMenuContent align="end">
                  <DropdownMenuItem
                    onClick={() => setViewMode('table')}
                    className={viewMode === 'table' ? 'bg-muted' : ''}
                  >
                    <HugeiconsIcon icon={List01Icon} className="size-4 mr-2" />
                    List
                  </DropdownMenuItem>
                  <DropdownMenuItem
                    onClick={() => setViewMode('grid')}
                    className={viewMode === 'grid' ? 'bg-muted' : ''}
                  >
                    <HugeiconsIcon icon={Grid01Icon} className="size-4 mr-2" />
                    Grid
                  </DropdownMenuItem>
                </DropdownMenuContent>
              </DropdownMenu>
            </HStack>
          </HStack>
        </CardHeader>

        <Box className="flex-1 min-h-0 overflow-y-auto">
          {viewMode === 'table' ? (
            <UsersListContainer
              usersQuery={usersQuery}
              limit={limit ?? 10}
              columns={columns}
              rowSelection={rowSelection}
              setRowSelection={setRowSelection}
              setUserToEdit={setUserToEdit}
              setUserToDelete={setUserToDelete}
              setUserToLock={setUserToLock}
              setUserToManagePermissions={setUserToManagePermissions}
              onCreateUser={() => setIsCreateUserOpen(true)}
              onAdd={() => setIsCreateUserOpen(true)}
              onAddLabel="Add User"
              facetedFilters={facetedFilters}
              onFetchFullData={fetchFullData}
              onImportCSV={(rows) => bulkImportUsers.mutate(rows)}
              onImportJSON={(rows) => bulkImportUsers.mutate(rows)}
              bulkActions={({ selectedRows }) => {
                return (
                  <UserToolbar
                    selectedUsers={new Set(selectedRows.map((r) => r.id))}
                    floating={false}
                    onBulkDelete={() => setIsBulkDeleteOpen(true)}
                    onBulkVerify={() => {}}
                    onBulkEdit={() => {}}
                  />
                )
              }}
              contextMenuItems={(row) => {
                return (
                  <UserContextMenuItems
                    user={row}
                    onToggleLock={(user) => {
                      if (user.lockout_until) {
                        setUserToLock(null)
                      } else {
                        setUserToLock(user)
                      }
                    }}
                    setUserToEdit={setUserToEdit}
                    setUserToDelete={setUserToDelete}
                    setUserToManagePermissions={setUserToManagePermissions}
                  />
                )
              }}
            />
          ) : (
            <Box p={4}>
              <UserGridView
                users={filteredUsers}
                isLoading={usersQuery.isFetching}
                onEdit={setUserToEdit}
                onDelete={setUserToDelete}
                onToggleLock={setUserToLock}
                onManagePermissions={setUserToManagePermissions}
                showProfilePictures={showProfilePictures}
              />
            </Box>
          )}
        </Box>
      </Card>

      {/* Modals */}
      <UserModals
        userToDelete={userToDelete}
        setUserToDelete={setUserToDelete}
        onDeleteConfirm={(id: string) =>
          deleteUser.mutate(
            { path: { id } },
            {
              onSuccess: () => {
                setUserToDelete(null)
              },
            },
          )
        }
        isBulkDeleteOpen={isBulkDeleteOpen}
        setIsBulkDeleteOpen={setIsBulkDeleteOpen}
        onBulkDeleteConfirm={() =>
          bulkDeleteUsers.mutate(
            {},
            {
              onSuccess: () => {
                setRowSelection({})
                setIsBulkDeleteOpen(false)
              },
            },
          )
        }
        userToEdit={userToEdit}
        setUserToEdit={setUserToEdit}
        onEditConfirm={() => {
          setUserToEdit(null)
        }}
        isBulkEditOpen={false}
        setIsBulkEditOpen={() => {}}
        onBulkEditConfirm={() => {}}
        selectedCount={Object.keys(rowSelection).length}
        userToLock={userToLock}
        setUserToLock={setUserToLock}
        onLockConfirm={() => {
          setUserToLock(null)
        }}
        userToManagePermissions={userToManagePermissions}
        setUserToManagePermissions={setUserToManagePermissions}
      />

      <UserCreateDialog
        open={isCreateUserOpen}
        onOpenChange={setIsCreateUserOpen}
        onConfirm={(data) =>
          createUser.mutate(
            {
              body: { email: data.email, password: data.password },
            },
            {
              onSuccess: () => {
                setIsCreateUserOpen(false)
              },
            },
          )
        }
        isSubmitting={createUser.isPending}
      />
    </Stack>
  )
}
