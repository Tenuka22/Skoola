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
import type { UserResponse } from '@/lib/api/types.gen' // Import UserResponse type
import { authClient } from '@/lib/clients' // Import authClient
import { getUserColumns } from '../../features/users/components/user-table-columns' // Import getUserColumns
import { UserAnalytics } from '../../features/users/components/user-analytics' // Import UserAnalytics
import { UserComparisonOverlay } from '../../features/users/components/user-comparison-overlay' // Import UserComparisonOverlay
import { UserModals } from '../../features/users/components/user-modals' // Import UserModals
import type {
  BulkUpdateValues,
  UpdateUserValues,
} from '../../features/users/schemas' // Import BulkUpdateValues and UpdateUserValues

import {
  getUsers06Bdcf95Aafda840B1D04322636De293Options,
  getUsersStatsBf304B57E4A0115F8280C4Bed2Fd9FbaOptions,
  patchUsers5D3C91131F7D9Efc5999C92Dbfac75DaMutation,
  deleteUsers5D3C91131F7D9Efc5999C92Dbfac75DaMutation,
  deleteUsersBulk6B8Be22247333C35E8A37A5Db37Fbfa8Mutation,
  patchUsersBulk6B8Be22247333C35E8A37A5Db37Fbfa8Mutation,
} from '@/lib/api/@tanstack/react-query.gen'
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

  const [selectedUsers, setSelectedUsers] = React.useState<Set<string>>(
    new Set(),
  )
  const [userToDelete, setUserToDelete] = React.useState<string | null>(null)
  const [isBulkDeleteOpen, setIsBulkDeleteOpen] = React.useState(false)
  const [isBulkEditOpen, setIsBulkEditOpen] = React.useState(false)
  const [userToEdit, setUserToEdit] = React.useState<UserResponse | null>(null)

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

  const { data: usersData, refetch: refetchUsers } = useQuery({
    ...getUsers06Bdcf95Aafda840B1D04322636De293Options({
      client: authClient,
      query: {
        page,
        limit,
        search: debouncedSearch,
        is_verified:
          statusFilter === 'all' ? undefined : statusFilter === 'verified',
        auth_method: authFilter === 'all' ? undefined : authFilter,
        sort_by: sortBy,
        sort_order: sortOrder as any,
      },
    }),
    placeholderData: keepPreviousData,
  })

  const { data: stats, refetch: refetchStats } = useQuery(
    getUsersStatsBf304B57E4A0115F8280C4Bed2Fd9FbaOptions({
      client: authClient,
    }),
  )

  const deleteMutation = useMutation({
    ...deleteUsers5D3C91131F7D9Efc5999C92Dbfac75DaMutation({
      client: authClient,
    }),
    onSuccess: () => {
      toast.success('User deleted')
      queryClient.invalidateQueries({
        queryKey: [{ _id: 'getUsers06Bdcf95Aafda840B1D04322636De293' }],
      })
      queryClient.invalidateQueries({
        queryKey: [{ _id: 'getUsersStatsBf304B57E4A0115F8280C4Bed2Fd9Fba' }],
      })
      setUserToDelete(null)
    },
  })

  const bulkDeleteMutation = useMutation({
    ...deleteUsersBulk6B8Be22247333C35E8A37A5Db37Fbfa8Mutation({
      client: authClient,
    }),
    onSuccess: () => {
      toast.success('Users deleted')
      queryClient.invalidateQueries({
        queryKey: [{ _id: 'getUsers06Bdcf95Aafda840B1D04322636De293' }],
      })
      queryClient.invalidateQueries({
        queryKey: [{ _id: 'getUsersStatsBf304B57E4A0115F8280C4Bed2Fd9Fba' }],
      })
      setSelectedUsers(new Set())
      setIsBulkDeleteOpen(false)
    },
  })

  const updateMutation = useMutation({
    ...patchUsers5D3C91131F7D9Efc5999C92Dbfac75DaMutation({
      client: authClient,
    }),
    onSuccess: () => {
      toast.success('User updated')
      queryClient.invalidateQueries({
        queryKey: [{ _id: 'getUsers06Bdcf95Aafda840B1D04322636De293' }],
      })
    },
  })

  const bulkUpdateMutation = useMutation({
    ...patchUsersBulk6B8Be22247333C35E8A37A5Db37Fbfa8Mutation({
      client: authClient,
    }),
    onSuccess: () => {
      toast.success('Batch update complete')
      queryClient.invalidateQueries({
        queryKey: [{ _id: 'getUsers06Bdcf95Aafda840B1D04322636De293' }],
      })
      setSelectedUsers(new Set())
      setIsBulkEditOpen(false)
    },
  })

  const columns = React.useMemo(
    () =>
      getUserColumns({
        selectedUsers,
        setSelectedUsers,
        setUserToDelete,
        onToggleVerify: (user: UserResponse) =>
          updateMutation.mutate({
            path: { user_id: user.id },
            body: { is_verified: !user.is_verified },
          }),
        onToggleLock: (user: UserResponse) =>
          updateMutation.mutate({
            path: { user_id: user.id },
            body: { is_locked: true },
          }),
        onEditUser: (user: UserResponse) => setUserToEdit(user),
        users: usersData?.data,
      }),
    [selectedUsers, usersData?.data],
  )

  return (
    <div className="space-y-6 p-6">
      <UserAnalytics stats={stats as any} />

      {/* Main Content Area */}
      <Card className="border-none shadow-xl overflow-hidden">
        <CardHeader className="flex flex-col space-y-4 border-b bg-muted/20 px-6 py-5 lg:flex-row lg:items-center lg:justify-between lg:space-y-0">
          <div className="space-y-1">
            <CardTitle className="text-2xl font-black">Directory</CardTitle>
            <CardDescription className="font-medium">
              Manage and filter system users with server-side precision.
            </CardDescription>
          </div>
          <div className="flex flex-wrap items-center gap-3">
            <div className="relative group">
              <HugeiconsIcon
                icon={Search01Icon}
                className="absolute left-3 top-1/2 size-4 -translate-y-1/2 text-muted-foreground transition-colors group-focus-within:text-primary"
              />
              <Input
                placeholder="Fuzzy search ID or Email..."
                className="w-72 border-none bg-background/50 pl-10 ring-1 ring-border focus-visible:ring-2 focus-visible:ring-primary shadow-sm"
                value={search}
                onChange={(e) => setSearch(e.target.value)}
              />
            </div>

            <div className="flex items-center gap-1 rounded-xl bg-background/50 p-1 ring-1 ring-border shadow-sm">
              <HugeiconsIcon
                icon={FilterIcon}
                className="ml-2 size-3 text-muted-foreground"
              />
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
              <HugeiconsIcon
                icon={FingerPrintIcon}
                className="ml-2 size-3 text-muted-foreground"
              />
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
              <DropdownMenuTrigger
                render={
                  <Button
                    variant="outline"
                    size="sm"
                    className="h-10 rounded-xl"
                  >
                    <HugeiconsIcon icon={ViewIcon} className="mr-2 size-4" />
                    Columns
                  </Button>
                }
              />
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

            <Button
              variant="outline"
              size="icon"
              className="h-10 w-10 rounded-xl shadow-sm transition-transform active:scale-95"
              onClick={() => {
                refetchUsers()
                refetchStats()
              }}
            >
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
            fetchPreviousPage={() => setPage((p) => p - 1)}
            fetchNextPage={() => setPage((p) => p + 1)}
            sorting={sorting}
            onSortingChange={setSorting}
          />
        </CardContent>
      </Card>

      <UserComparisonOverlay
        selectedUsers={selectedUsers}
        onClear={() => setSelectedUsers(new Set())}
        onBulkVerify={(v: boolean) =>
          bulkUpdateMutation.mutate({
            body: { user_ids: Array.from(selectedUsers), is_verified: v },
          })
        }
        onBulkDelete={() => setIsBulkDeleteOpen(true)}
        onBulkEdit={() => setIsBulkEditOpen(true)}
        users={usersData?.data as any}
      />

      <UserModals
        userToDelete={userToDelete}
        setUserToDelete={setUserToDelete}
        onDeleteConfirm={(id: string) =>
          deleteMutation.mutate({ path: { user_id: id } })
        }
        isBulkDeleteOpen={isBulkDeleteOpen}
        setIsBulkDeleteOpen={setIsBulkDeleteOpen}
        onBulkDeleteConfirm={() =>
          bulkDeleteMutation.mutate({
            body: { user_ids: Array.from(selectedUsers) },
          })
        }
        isBulkEditOpen={isBulkEditOpen}
        setIsBulkEditOpen={setIsBulkEditOpen}
        onBulkEditConfirm={(data: BulkUpdateValues) =>
          bulkUpdateMutation.mutate({
            body: { user_ids: Array.from(selectedUsers), ...data },
          })
        }
        selectedCount={selectedUsers.size}
        isBulkUpdating={bulkUpdateMutation.isPending}
        userToEdit={userToEdit}
        setUserToEdit={setUserToEdit}
        onEditConfirm={(data: UpdateUserValues) =>
          userToEdit &&
          updateMutation.mutate({
            path: { user_id: userToEdit.id },
            body: data,
          })
        }
        isUpdating={updateMutation.isPending}
      />
    </div>
  )
}
