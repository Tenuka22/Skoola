import { createFileRoute } from '@tanstack/react-router'
import { keepPreviousData, useQuery } from '@tanstack/react-query'
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
import { isAuthMethod } from '../../features/users/utils/user-guards'
import type { QueryClient } from '@tanstack/react-query'
import type {
  BulkUpdateValues,
  UpdateUserValues,
} from '../../features/users/schemas'
import type { UserResponse } from '@/lib/api'
import { Stack } from '@/components/primitives'
import {
  getUsersQueryOptions,
  useBulkDeleteUsers,
  useBulkImportUsers,
  useBulkUpdateUsers,
  useDeleteUser,
  useRegisterUser,
  useUpdateUser,
} from '@/features/users/api'
import { useUsersSearchParams } from '@/features/users/search-params'
import { authClient } from '@/lib/clients'

export const Route = createFileRoute('/admin/users')({
  component: Users,
})

function Users() {
  const { page, limit, search, statusFilter, authFilter, sort } =
    useUsersSearchParams()

  const [userToDelete, setUserToDelete] = React.useState<string | null>(null)
  const [isBulkDeleteOpen, setIsBulkDeleteOpen] = React.useState(false)
  const [isBulkEditOpen, setIsBulkEditOpen] = React.useState(false)
  const [isCreateUserOpen, setIsCreateUserOpen] = React.useState(false)
  const [userToLock, setUserToLock] = React.useState<UserResponse | null>(null)
  const [userToEdit, setUserToEdit] = React.useState<UserResponse | null>(null)
  const [userToManagePermissions, setUserToManagePermissions] =
    React.useState<UserResponse | null>(null)
  const [showProfilePictures, setShowProfilePictures] = React.useState(true)

  const usersQuery = useQuery({
    ...getUsersQueryOptions({
      query: {
        page: page ?? 1,
        limit: limit ?? 10,
        search: search ?? undefined,
        is_verified:
          statusFilter === 'all'
            ? undefined
            : statusFilter === 'verified'
              ? true
              : false,
        auth_method:
          authFilter === 'all'
            ? undefined
            : isAuthMethod(authFilter)
              ? authFilter
              : undefined,
        sort_by: sort?.[0]?.id ?? 'created_at',
        sort_order: sort?.[0]?.desc ? 'desc' : 'asc',
      },
    }),
    placeholderData: keepPreviousData,
  })

  const deleteUser = useDeleteUser()
  const bulkDeleteUsers = useBulkDeleteUsers()
  const updateUser = useUpdateUser()
  const bulkUpdateUsers = useBulkUpdateUsers()
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
      query: {
        page: 1,
        limit: 1000,
        search: search ?? undefined,
        is_verified:
          statusFilter === 'all'
            ? undefined
            : statusFilter === 'verified'
              ? true
              : false,
        auth_method:
          authFilter === 'all'
            ? undefined
            : isAuthMethod(authFilter)
              ? authFilter
              : undefined,
        sort_by: sort?.[0]?.id ?? 'created_at',
        sort_order: sort?.[0]?.desc ? 'desc' : 'asc',
      },
    })

    if (!options.queryFn) return []
    const response = await options.queryFn({
      queryKey: options.queryKey,
      signal: new AbortController().signal,
      meta: undefined,
    })
    return response.data
  }, [search, statusFilter, authFilter, sort])

  const columns = getUserColumns({
    onToggleVerify: (user: UserResponse) =>
      updateUser.mutate({
        path: { user_id: user.id },
        body: { is_verified: !user.is_verified },
      }),
    onToggleLock: (user: UserResponse) => {
      if (user.lockout_until) {
        updateUser.mutate({
          path: { user_id: user.id },
          body: {
            lockout_until: null,
          },
        })
      } else {
        setUserToLock(user)
      }
    },
    setUserToDelete,
    setUserToEdit,
    setUserToManagePermissions,
    isUpdating: updateUser.isPending,
    updatingUserId: updateUser.variables?.path?.user_id ?? null,
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
        updateMutation={updateUser}
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
          const handleBulkVerify = (verify: boolean) => {
            bulkUpdateUsers.mutate(
              {
                body: {
                  user_ids: selectedRows.map((r) => r.id),
                  is_verified: verify,
                },
              },
              {
                onSuccess: () => setRowSelection({}),
              },
            )
          }

          return (
            <UserToolbar
              selectedUsers={new Set(selectedRows.map((r) => r.id))}
              floating={false}
              onBulkVerify={handleBulkVerify}
              onBulkDelete={() => setIsBulkDeleteOpen(true)}
              onBulkEdit={() => setIsBulkEditOpen(true)}
            />
          )
        }}
        contextMenuItems={(row) => {
          return (
            <UserContextMenuItems
              user={row}
              onToggleVerify={(user) =>
                updateUser.mutate({
                  path: { user_id: user.id },
                  body: { is_verified: !user.is_verified },
                })
              }
              onToggleLock={(user) => {
                if (user.lockout_until) {
                  updateUser.mutate({
                    path: { user_id: user.id },
                    body: {
                      lockout_until: null,
                    },
                  })
                } else {
                  setUserToLock(user)
                }
              }}
              setUserToDelete={setUserToDelete}
              setUserToEdit={setUserToEdit}
              setUserToManagePermissions={setUserToManagePermissions}
              isUpdating={updateUser.isPending}
              updatingUserId={updateUser.variables?.path?.user_id ?? null}
            />
          )
        }}
      />

      <UserModals
        userToDelete={userToDelete}
        setUserToDelete={setUserToDelete}
        onDeleteConfirm={(id: string) =>
          deleteUser.mutate(
            { path: { user_id: id } },
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
            {
              body: { userIds: Object.keys(rowSelection) },
            },
            {
              onSuccess: () => {
                setRowSelection({})
                setIsBulkDeleteOpen(false)
              },
            },
          )
        }
        isBulkEditOpen={isBulkEditOpen}
        setIsBulkEditOpen={setIsBulkEditOpen}
        onBulkEditConfirm={(data: BulkUpdateValues) =>
          bulkUpdateUsers.mutate(
            {
              body: { user_ids: Object.keys(rowSelection), ...data },
            },
            {
              onSuccess: () => {
                setRowSelection({})
                setIsBulkEditOpen(false)
              },
            },
          )
        }
        selectedCount={Object.keys(rowSelection).length}
        isBulkUpdating={bulkUpdateUsers.isPending}
        userToEdit={userToEdit}
        setUserToEdit={setUserToEdit}
        onEditConfirm={(data: UpdateUserValues) =>
          userToEdit &&
          updateUser.mutate(
            {
              path: { user_id: userToEdit.id },
              body: data,
            },
            {
              onSuccess: () => {
                setUserToEdit(null)
                setUserToLock(null)
              },
            },
          )
        }
        isUpdating={updateUser.isPending}
        userToLock={userToLock}
        setUserToLock={setUserToLock}
        onLockConfirm={(date) =>
          userToLock &&
          updateUser.mutate(
            {
              path: { user_id: userToLock.id },
              body: { lockout_until: date.toISOString().slice(0, 19) },
            },
            {
              onSuccess: () => {
                setUserToLock(null)
                setUserToEdit(null)
              },
            },
          )
        }
        isLocking={updateUser.isPending}
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
