import { createFileRoute } from '@tanstack/react-router'
import {
  keepPreviousData,
  useQuery,
  useQueryClient,
} from '@tanstack/react-query'

import * as React from 'react'

import { UserCreateDialog } from '../../features/users/components/user-create-dialog'
import { UserModals } from '../../features/users/components/user-modals'
import { UserToolbar } from '../../features/users/components/user-toolbar'
import {
  UserContextMenuItems,
  getUserColumns,
} from '../../features/users/components/user-table-columns'
import { UsersHeader } from '../../features/users/components/users-header'
import { UsersListContainer } from '../../features/users/components/users-list-container'

import type { UserResponse } from '@/lib/api'
import { Stack } from '@/components/primitives'
import {
  getUsersQueryOptions,
  useBulkDeleteUsers,
  useBulkImportUsers,
  useDeleteUser,
  useRegisterUser,
} from '@/features/users/api'
import { useUsersSearchParams } from '@/features/users/search-params'
import { authClient } from '@/lib/clients'

export const Route = createFileRoute('/admin/users')({
  component: Users,
})

function Users() {
  const queryClient = useQueryClient()
  const { page, limit, search, sort } = useUsersSearchParams()

  const [userToDelete, setUserToDelete] = React.useState<string | null>(null)
  const [isBulkDeleteOpen, setIsBulkDeleteOpen] = React.useState(false)
  const [isCreateUserOpen, setIsCreateUserOpen] = React.useState(false)
  const [userToLock, setUserToLock] = React.useState<UserResponse | null>(null)
  const [userToEdit, setUserToEdit] = React.useState<UserResponse | null>(null)
  const [userToManagePermissions, setUserToManagePermissions] =
    React.useState<UserResponse | null>(null)
  const [showProfilePictures, setShowProfilePictures] = React.useState(true)

  const usersQuery = useQuery({
    ...getUsersQueryOptions({
      client: authClient,
      query: {
        page: page ?? 1,
        limit: limit ?? 10,
        search: search ?? undefined,
        sort_by: sort?.[0]?.id ?? 'created_at',
        sort_order: sort?.[0]?.desc ? 'desc' : 'asc',
      },
    }),
    placeholderData: keepPreviousData,
  })

  const deleteUser = useDeleteUser()
  const bulkDeleteUsers = useBulkDeleteUsers()
  const createUser = useRegisterUser()
  const bulkImportUsers = useBulkImportUsers()

  const [rowSelection, setRowSelection] = React.useState<
    Record<string, boolean>
  >({})

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
        search: search ?? undefined,
        sort_by: sort?.[0]?.id ?? 'created_at',
        sort_order: sort?.[0]?.desc ? 'desc' : 'asc',
      },
    })

    if (!options.queryFn) return []
    const response = await options.queryFn({
      client: queryClient,
      queryKey: options.queryKey,
      signal: new AbortController().signal,
      meta: undefined,
    })
    // eslint-disable-next-line @typescript-eslint/consistent-type-assertions
    const typedResponse = response as { data: Array<UserResponse> }
    return typedResponse.data
  }, [search, sort, queryClient])

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

  return (
    <Stack gap={4} p={8} className="h-full">
      <UsersHeader
        showProfilePictures={showProfilePictures}
        setShowProfilePictures={setShowProfilePictures}
      />
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
          // Note: User updates are not supported by the API
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
