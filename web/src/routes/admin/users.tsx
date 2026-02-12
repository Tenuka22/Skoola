import * as React from 'react'
import {
  Add01Icon,
  ArrowRight01Icon,
  Download01Icon,
  FilterIcon,
  LayoutGridIcon,
  ListViewIcon,
  Search01Icon,
  Settings01Icon,
  TableIcon,
  ViewOffSlashIcon,
} from '@hugeicons/core-free-icons'
import { HugeiconsIcon } from '@hugeicons/react'
import {
  keepPreviousData,
  useMutation,
  useQuery,
  useQueryClient,
} from '@tanstack/react-query'
import { createFileRoute } from '@tanstack/react-router'
import { toast } from 'sonner'

import { Badge } from '@/components/ui/badge'
import { Button } from '@/components/ui/button'
import { DataTable } from '@/components/ui/data-table'
import {
  DropdownMenu,
  DropdownMenuCheckboxItem,
  DropdownMenuContent,
  DropdownMenuTrigger,
} from '@/components/ui/dropdown-menu'
import { Input } from '@/components/ui/input'
import { Tabs, TabsContent, TabsList, TabsTrigger } from '@/components/ui/tabs'
import { authClient } from '@/lib/clients'

import { UserBulkPermissionsDialog } from '../../features/permissions/components/user-bulk-permissions-dialog'
import { UserPermissionsDialog } from '../../features/permissions/components/user-permissions-dialog'
import { UserCreateDialog } from '../../features/users/components/user-create-dialog'
import { UserComparisonOverlay } from '../../features/users/components/user-comparison-overlay'
import { getUserColumns } from '../../features/users/components/user-table-columns'
import { UserModals } from '../../features/users/components/user-modals'
import { UserBoardView } from '../../features/users/components/user-board-view'
import { UserListView } from '../../features/users/components/user-list-view'

import type { UserResponse } from '@/lib/api/types.gen'
import type { SortingState } from '@tanstack/react-table'
import type {
  BulkUpdateValues,
  UpdateUserValues,
} from '../../features/users/schemas'

import {
  deleteUsers5D3C91131F7D9Efc5999C92Dbfac75DaMutation,
  deleteUsersBulk6B8Be22247333C35E8A37A5Db37Fbfa8Mutation,
  getUsers06Bdcf95Aafda840B1D04322636De293Options,
  getUsersStatsBf304B57E4A0115F8280C4Bed2Fd9FbaOptions,
  patchUsers5D3C91131F7D9Efc5999C92Dbfac75DaMutation,
  patchUsersBulk6B8Be22247333C35E8A37A5Db37Fbfa8Mutation,
  postAuthRegisterD7296Dbacc4Fd751Aeb142Bbb8A63Fd9Mutation,
} from '@/lib/api/@tanstack/react-query.gen'

export const Route = createFileRoute('/admin/users')({
  component: Users,
})

function Users() {
  const [page, setPage] = React.useState(1)
  const [search, setSearch] = React.useState('')
  const [debouncedSearch, setDebouncedSearch] = React.useState('')
  const [statusFilter] = React.useState<string>('all')
  const [authFilter] = React.useState<string>('all')
  const [sorting, setSorting] = React.useState<SortingState>([])
  const [columnVisibility, setColumnVisibility] = React.useState({})

  const [selectedUsers, setSelectedUsers] = React.useState<Set<string>>(
    new Set(),
  )
  const [userToDelete, setUserToDelete] = React.useState<string | null>(null)
  const [isBulkDeleteOpen, setIsBulkDeleteOpen] = React.useState(false)
  const [isBulkEditOpen, setIsBulkEditOpen] = React.useState(false)
  const [isBulkPermissionsOpen, setIsBulkPermissionsOpen] = React.useState(false)
  const [isCreateUserOpen, setIsCreateUserOpen] = React.useState(false)
  const [userToEdit, setUserToEdit] = React.useState<UserResponse | null>(null)
  const [userToManagePermissions, setUserToManagePermissions] = React.useState<UserResponse | null>(null)

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

  const usersQuery = useQuery({
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

  const { data: stats } = useQuery(
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

  const createUserMutation = useMutation({
    ...postAuthRegisterD7296Dbacc4Fd751Aeb142Bbb8A63Fd9Mutation({
      client: authClient,
    }),
    onSuccess: () => {
      toast.success('User created successfully')
      queryClient.invalidateQueries({
        queryKey: [{ _id: 'getUsers06Bdcf95Aafda840B1D04322636De293' }],
      })
      queryClient.invalidateQueries({
        queryKey: [{ _id: 'getUsersStatsBf304B57E4A0115F8280C4Bed2Fd9Fba' }],
      })
      setIsCreateUserOpen(false)
    },
    onError: (error: any) => {
      toast.error(error.message || 'Failed to create user')
    },
  })

  const handleExportCSV = () => {
    if (!usersQuery.data?.data) return
    const headers = ['ID', 'Email', 'Verified', 'Created At']
    const rows = usersQuery.data.data.map((u) => [
      u.id,
      u.email,
      u.is_verified ? 'Yes' : 'No',
      new Date(u.created_at).toLocaleString(),
    ])
    const csvContent =
      'data:text/csv;charset=utf-8,' +
      [headers.join(','), ...rows.map((e) => e.join(','))].join('\n')
    const encodedUri = encodeURI(csvContent)
    const link = document.createElement('a')
    link.setAttribute('href', encodedUri)
    link.setAttribute('download', 'users_export.csv')
    document.body.appendChild(link)
    link.click()
    document.body.removeChild(link)
  }

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
        onManagePermissions: (user: UserResponse) => setUserToManagePermissions(user),
        users: usersQuery.data?.data,
      }),
    [selectedUsers, usersQuery.data?.data, updateMutation],
  )

  return (
    <div className="flex h-full flex-col bg-background">
      {/* Header */}
      <div className="px-8 py-6 pb-2">
        <div className="mb-1 flex items-center gap-3">
          <h1 className="text-2xl font-semibold tracking-tight">User management</h1>
          <Badge variant="secondary" className="rounded-md bg-muted px-2 py-0.5 text-xs font-normal text-muted-foreground hover:bg-muted">
            {stats?.total_users || 0}
          </Badge>
        </div>
        <p className="text-sm text-muted-foreground">
          Manage your team members and their account permissions here.
        </p>
      </div>

      <div className="flex-1 px-8 py-4 space-y-4">
        <Tabs defaultValue="table" className="w-full">
          <div className="mb-4 flex flex-col gap-4 sm:flex-row sm:items-center sm:justify-between">
            <TabsList className="h-9 w-fit rounded-lg bg-muted/50 p-1">
              <TabsTrigger value="table" className="flex h-7 gap-2 rounded-md px-3 text-xs data-[state=active]:bg-background data-[state=active]:shadow-sm">
                <HugeiconsIcon icon={TableIcon} className="size-3.5" />
                Table
              </TabsTrigger>
              <TabsTrigger value="board" className="flex h-7 gap-2 rounded-md px-3 text-xs data-[state=active]:bg-background data-[state=active]:shadow-sm">
                <HugeiconsIcon icon={LayoutGridIcon} className="size-3.5" />
                Board
              </TabsTrigger>
              <TabsTrigger value="list" className="flex h-7 gap-2 rounded-md px-3 text-xs data-[state=active]:bg-background data-[state=active]:shadow-sm">
                <HugeiconsIcon icon={ListViewIcon} className="size-3.5" />
                List
              </TabsTrigger>
            </TabsList>

            <div className="flex items-center gap-2 overflow-x-auto pb-2 sm:w-auto sm:pb-0">
              <div className="group relative min-w-[200px]">
                <HugeiconsIcon
                  icon={Search01Icon}
                  className="absolute left-3 top-1/2 size-4 -translate-y-1/2 text-muted-foreground transition-colors group-focus-within:text-foreground"
                />
                <Input
                  placeholder="Search"
                  className="h-9 w-full border-border/60 bg-background pl-9 transition-all focus-visible:border-primary focus-visible:ring-1 focus-visible:ring-primary sm:w-64"
                  value={search}
                  onChange={(e) => setSearch(e.target.value)}
                />
              </div>
              
              <Button variant="outline" size="sm" className="hidden h-9 gap-2 text-muted-foreground hover:text-foreground md:flex">
                <HugeiconsIcon icon={ViewOffSlashIcon} className="size-4" />
                Hide
              </Button>
              
              <DropdownMenu>
                <DropdownMenuTrigger
                  render={
                    <Button variant="outline" size="sm" className="hidden h-9 gap-2 text-muted-foreground hover:text-foreground md:flex">
                      <HugeiconsIcon icon={Settings01Icon} className="size-4" />
                      Customize
                      <HugeiconsIcon icon={ArrowRight01Icon} className="size-3 rotate-90 opacity-50" />
                    </Button>
                  }
                />
                <DropdownMenuContent align="end">
                {['email', 'is_verified', 'created_at']
                  .filter(col => col !== 'email')
                  .map((columnId) => (
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
                size="sm" 
                className="h-9 gap-2 text-muted-foreground hover:text-foreground"
                onClick={handleExportCSV}
              >
                 <HugeiconsIcon icon={Download01Icon} className="size-4" />
                Export
              </Button>
              
              <Button 
                size="sm" 
                className="h-9 gap-2 bg-primary text-primary-foreground shadow-sm hover:bg-primary/90"
                onClick={() => setIsCreateUserOpen(true)}
              >
                 Add User
                 <HugeiconsIcon icon={Add01Icon} className="size-4" />
              </Button>
            </div>
          </div>

          <div className="mb-6 flex items-center gap-3 overflow-x-auto pb-2">
            <div className="flex cursor-pointer items-center gap-1 rounded-lg border border-border/60 bg-background px-3 py-1.5 shadow-sm transition-colors hover:bg-muted/50">
              <HugeiconsIcon icon={FilterIcon} className="size-3.5 text-muted-foreground" />
              <span className="text-xs font-medium">Role</span>
              <HugeiconsIcon icon={ArrowRight01Icon} className="ml-1 size-3 rotate-90 text-muted-foreground" />
            </div>

            <Button variant="ghost" size="sm" className="h-8 gap-1.5 px-2 text-muted-foreground hover:text-foreground">
              <HugeiconsIcon icon={Add01Icon} className="size-3.5" />
              <span className="text-xs font-medium">Add filter</span>
            </Button>
          </div>

          <TabsContent value="table" className="mt-0">
             <div className="overflow-hidden rounded-xl border border-border/60 bg-background shadow-sm">
                <DataTable
                  columns={columns}
                  data={usersQuery.data?.data || []}
                  pageIndex={page - 1}
                  pageSize={limit}
                  pageCount={usersQuery.data?.total_pages || 0}
                  canPreviousPage={page > 1}
                  canNextPage={page < (usersQuery.data?.total_pages || 0)}
                  fetchPreviousPage={() => setPage((p) => p - 1)}
                  fetchNextPage={() => setPage((p) => p + 1)}
                  sorting={sorting}
                  onSortingChange={setSorting}
                  columnVisibility={columnVisibility}
                  onColumnVisibilityChange={setColumnVisibility}
                  isLoading={usersQuery.isFetching}
                />
             </div>
          </TabsContent>
          
          <TabsContent value="board">
             <UserBoardView 
               users={usersQuery.data?.data}
               isLoading={usersQuery.isFetching}
               onEdit={(user) => setUserToEdit(user)}
               onDelete={(id) => setUserToDelete(id)}
               onToggleVerify={(user) =>
                 updateMutation.mutate({
                   path: { user_id: user.id },
                   body: { is_verified: !user.is_verified },
                 })
               }
               onManagePermissions={(user) => setUserToManagePermissions(user)}
             />
          </TabsContent>
          
          <TabsContent value="list">
             <UserListView 
               users={usersQuery.data?.data}
               isLoading={usersQuery.isFetching}
               onEdit={(user) => setUserToEdit(user)}
               onDelete={(id) => setUserToDelete(id)}
               onToggleVerify={(user) =>
                 updateMutation.mutate({
                   path: { user_id: user.id },
                   body: { is_verified: !user.is_verified },
                 })
               }
               onManagePermissions={(user) => setUserToManagePermissions(user)}
             />
          </TabsContent>
        </Tabs>
      </div>

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
        onBulkManagePermissions={() => setIsBulkPermissionsOpen(true)}
        users={usersQuery.data?.data as any}
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

      <UserPermissionsDialog
        user={userToManagePermissions}
        open={!!userToManagePermissions}
        onOpenChange={(open) => !open && setUserToManagePermissions(null)}
      />

      <UserBulkPermissionsDialog
        userIds={Array.from(selectedUsers)}
        open={isBulkPermissionsOpen}
        onOpenChange={setIsBulkPermissionsOpen}
      />

      <UserCreateDialog
        open={isCreateUserOpen}
        onOpenChange={setIsCreateUserOpen}
        onConfirm={(data) => createUserMutation.mutate({ body: { ...data } })}
        isSubmitting={createUserMutation.isPending}
      />
    </div>
  )
}
