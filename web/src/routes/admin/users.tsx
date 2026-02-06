'use client'

import * as React from 'react'
import { createFileRoute } from '@tanstack/react-router'
import {
  keepPreviousData,
  useMutation,
  useQuery,
  useQueryClient,
} from '@tanstack/react-query'
import {
  Card,
  CardContent,
  CardDescription,
  CardHeader,
  CardTitle,
} from '@/components/ui/card'
import { Button } from '@/components/ui/button'
import { HugeiconsIcon } from '@hugeicons/react'
import {
  RefreshIcon,
  Search01Icon,
  FilterIcon,
  FingerPrintIcon,
  ViewIcon,
} from '@hugeicons/core-free-icons'
import { toast } from 'sonner'
import { DataTable } from '@/components/ui/data-table'
import type { SortingState } from '@tanstack/react-table'
import { Input } from '@/components/ui/input'
import {
  Select,
  SelectContent,
  SelectItem,
  SelectTrigger,
  SelectValue,
} from '@/components/ui/select'
import {
  DropdownMenu,
  DropdownMenuCheckboxItem,
  DropdownMenuContent,
  DropdownMenuTrigger,
} from '@/components/ui/dropdown-menu'

import { 
  getUsers, 
  getUserStats, 
  deleteUser, 
  bulkDeleteUsers, 
  updateUser, 
  bulkUpdateUsers 
} from '@/features/users/api'
import type { PaginatedUserResponse, UserStatsResponse } from '@/features/users/types'
import { UserAnalytics } from '@/features/users/components/user-analytics'
import { getUserColumns } from '@/features/users/components/user-table-columns'
import { UserComparisonOverlay } from '@/features/users/components/user-comparison-overlay'
import { UserModals } from '@/features/users/components/user-modals'

export const Route = createFileRoute('/admin/users')({
  component: Users,
})

function Users() {
  const [page, setPage] = React.useState(1)
  const [search, setSearch] = React.useState('')
  const [debouncedSearch, setDebouncedSearch] = React.useState('')
  const [statusFilter, setStatusFilter] = React.useState<string>('all')
  const [authFilter, setAuthFilter] = React.useState<string>('all')
  const [sorting, setSorting] = React.useState<SortingState>([])
  const [columnVisibility, setColumnVisibility] = React.useState({})
  
  const [selectedUsers, setSelectedUsers] = React.useState<Set<string>>(new Set())
  const [userToDelete, setUserToDelete] = React.useState<string | null>(null)
  const [isBulkDeleteOpen, setIsBulkDeleteOpen] = React.useState(false)

  const limit = 10
  const queryClient = useQueryClient()

  // Debounce search
  React.useEffect(() => {
    const handler = setTimeout(() => {
      setDebouncedSearch(search)
      setPage(1)
    }, 400)
    return () => clearTimeout(handler)
  }, [search])

  const sortBy = sorting[0]?.id
  const sortOrder = sorting[0]?.desc ? 'desc' : 'asc'

  const { data: usersData, refetch: refetchUsers } = useQuery<PaginatedUserResponse>({
    queryKey: ['users', page, limit, debouncedSearch, statusFilter, authFilter, sortBy, sortOrder],
    queryFn: () => getUsers({
      page,
      limit,
      search: debouncedSearch,
      is_verified: statusFilter === 'all' ? undefined : statusFilter === 'verified',
      auth_method: authFilter === 'all' ? undefined : authFilter,
      sort_by: sortBy,
      sort_order: sortOrder as any,
    }),
    placeholderData: keepPreviousData,
  })

  const { data: stats, refetch: refetchStats } = useQuery<UserStatsResponse>({
    queryKey: ['users-stats'],
    queryFn: getUserStats,
  })

  const deleteMutation = useMutation({
    mutationFn: deleteUser,
    onSuccess: () => {
      toast.success('User deleted')
      queryClient.invalidateQueries({ queryKey: ['users'] })
      queryClient.invalidateQueries({ queryKey: ['users-stats'] })
      setUserToDelete(null)
    },
  })

  const bulkDeleteMutation = useMutation({
    mutationFn: bulkDeleteUsers,
    onSuccess: () => {
      toast.success('Users deleted')
      queryClient.invalidateQueries({ queryKey: ['users'] })
      queryClient.invalidateQueries({ queryKey: ['users-stats'] })
      setSelectedUsers(new Set())
      setIsBulkDeleteOpen(false)
    },
  })

  const updateMutation = useMutation({
    mutationFn: ({ id, data }: { id: string; data: any }) => updateUser(id, data),
    onSuccess: () => {
      toast.success('User updated')
      queryClient.invalidateQueries({ queryKey: ['users'] })
    },
  })

  const bulkUpdateMutation = useMutation({
    mutationFn: (data: { is_verified?: boolean }) => bulkUpdateUsers(Array.from(selectedUsers), data),
    onSuccess: () => {
      toast.success('Batch update complete')
      queryClient.invalidateQueries({ queryKey: ['users'] })
      setSelectedUsers(new Set())
    },
  })

  const columns = React.useMemo(() => getUserColumns({
    selectedUsers,
    setSelectedUsers,
    setUserToDelete,
    onToggleVerify: (user) => updateMutation.mutate({ id: user.id, data: { is_verified: !user.is_verified } }),
    onToggleLock: (user) => updateMutation.mutate({ id: user.id, data: { is_locked: true } }), 
    users: usersData?.data,
  }), [selectedUsers, usersData?.data])

  return (
    <div className="space-y-6 p-6">
      <UserAnalytics stats={stats} />

      {/* Main Content Area */}
      <Card className="border-none shadow-xl overflow-hidden">
        <CardHeader className="flex flex-col space-y-4 border-b bg-muted/20 px-6 py-5 lg:flex-row lg:items-center lg:justify-between lg:space-y-0">
          <div className="space-y-1">
            <CardTitle className="text-2xl font-black">Directory</CardTitle>
            <CardDescription className="font-medium">Manage and filter system users with server-side precision.</CardDescription>
          </div>
          <div className="flex flex-wrap items-center gap-3">
            <div className="relative group">
              <HugeiconsIcon icon={Search01Icon} className="absolute left-3 top-1/2 size-4 -translate-y-1/2 text-muted-foreground transition-colors group-focus-within:text-primary" />
              <Input
                placeholder="Fuzzy search ID or Email..."
                className="w-72 border-none bg-background/50 pl-10 ring-1 ring-border focus-visible:ring-2 focus-visible:ring-primary shadow-sm"
                value={search}
                onChange={(e) => setSearch(e.target.value)}
              />
            </div>
            
            <div className="flex items-center gap-1 rounded-xl bg-background/50 p-1 ring-1 ring-border shadow-sm">
              <HugeiconsIcon icon={FilterIcon} className="ml-2 size-3 text-muted-foreground" />
              <Select 
                value={statusFilter} 
                onValueChange={(val) => setStatusFilter(val || 'all')}
              >
                <SelectTrigger className="h-8 border-none bg-transparent text-[11px] font-black uppercase tracking-wider focus:ring-0">
                  <SelectValue placeholder="Verification" />
                </SelectTrigger>
                <SelectContent>
                  <SelectItem value="all">Verified & Pending</SelectItem>
                  <SelectItem value="verified">Verified Only</SelectItem>
                  <SelectItem value="pending">Pending Only</SelectItem>
                </SelectContent>
              </Select>
            </div>

            <div className="flex items-center gap-1 rounded-xl bg-background/50 p-1 ring-1 ring-border shadow-sm">
              <HugeiconsIcon icon={FingerPrintIcon} className="ml-2 size-3 text-muted-foreground" />
              <Select 
                value={authFilter} 
                onValueChange={(val) => setAuthFilter(val || 'all')}
              >
                <SelectTrigger className="h-8 border-none bg-transparent text-[11px] font-black uppercase tracking-wider focus:ring-0">
                  <SelectValue placeholder="Provider" />
                </SelectTrigger>
                <SelectContent>
                  <SelectItem value="all">Any Provider</SelectItem>
                  <SelectItem value="google">Google Auth</SelectItem>
                  <SelectItem value="github">GitHub Auth</SelectItem>
                  <SelectItem value="password">Direct Sign-in</SelectItem>
                </SelectContent>
              </Select>
            </div>

            <DropdownMenu>
              <DropdownMenuTrigger render={
                <Button variant="outline" size="sm" className="h-10 rounded-xl">
                  <HugeiconsIcon icon={ViewIcon} className="mr-2 size-4" />
                  Columns
                </Button>
              } />
              <DropdownMenuContent align="end">
                {['email', 'is_verified', 'created_at'].map((columnId) => (
                  <DropdownMenuCheckboxItem
                    key={columnId}
                    className="capitalize"
                    checked={(columnVisibility as any)[columnId] !== false}
                    onCheckedChange={(value) =>
                      setColumnVisibility((prev) => ({
                        ...prev,
                        [columnId]: !!value,
                      }))
                    }
                  >
                    {columnId.replace('_', ' ')}
                  </DropdownMenuCheckboxItem>
                ))}
              </DropdownMenuContent>
            </DropdownMenu>

            <Button variant="outline" size="icon" className="h-10 w-10 rounded-xl shadow-sm transition-transform active:scale-95" onClick={() => { refetchUsers(); refetchStats(); }}>
              <HugeiconsIcon icon={RefreshIcon} className="size-4" />
            </Button>
          </div>
        </CardHeader>
        <CardContent className="p-0">
          <DataTable
            columns={columns}
            data={usersData?.data || []}
            pageIndex={page - 1}
            pageSize={limit}
            pageCount={usersData?.total_pages || 0}
            canPreviousPage={page > 1}
            canNextPage={page < (usersData?.total_pages || 0)}
            fetchPreviousPage={() => setPage(p => p - 1)}
            fetchNextPage={() => setPage(p => p + 1)}
            sorting={sorting}
            onSortingChange={setSorting}
          />
        </CardContent>
      </Card>

      <UserComparisonOverlay 
        selectedUsers={selectedUsers}
        onClear={() => setSelectedUsers(new Set())}
        onBulkVerify={(v) => bulkUpdateMutation.mutate({ is_verified: v })}
        onBulkDelete={() => setIsBulkDeleteOpen(true)}
        users={usersData?.data}
      />

      <UserModals 
        userToDelete={userToDelete}
        setUserToDelete={setUserToDelete}
        onDeleteConfirm={(id) => deleteMutation.mutate(id)}
        isBulkDeleteOpen={isBulkDeleteOpen}
        setIsBulkDeleteOpen={setIsBulkDeleteOpen}
        onBulkDeleteConfirm={() => bulkDeleteMutation.mutate(Array.from(selectedUsers))}
        selectedCount={selectedUsers.size}
      />
    </div>
  )
}