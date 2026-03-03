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
import { UsersFilters } from '../../features/users/components/users-filters'
import { UsersHeader } from '../../features/users/components/users-header'
import { UsersListContainer } from '../../features/users/components/users-list-container'
import { UsersToolbar as UsersToolbarComponent } from '../../features/users/components/users-toolbar'
import { handleExportCSV } from '../../lib/export'
import { isAuthMethod } from '../../features/users/utils/user-guards'
import type {
  BulkUpdateValues,
  UpdateUserValues,
} from '../../features/users/schemas'
import type { UserResponse } from '@/lib/api'
import { Stack } from '@/components/primitives'
import {
  getUsersQueryOptions,
  useBulkDeleteUsers,
  useBulkUpdateUsers,
  useDeleteUser,
  useRegisterUser,
  useUpdateUser,
} from '@/features/users/api'
import { useUsersSearchParams } from '@/features/users/search-params'

export const Route = createFileRoute('/admin/users')({
  component: Users,
})

function Users() {
  const {
    page,
    limit,
    search,
    statusFilter,
    authFilter,
    createdAfter,
    createdBefore,
    sortBy,
    sortOrder,
  } = useUsersSearchParams()

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
        created_after: createdAfter ?? undefined,
        created_before: createdBefore ?? undefined,
        sort_by: sortBy ?? 'created_at',
        sort_order:
          sortOrder === 'asc' || sortOrder === 'desc' ? sortOrder : 'desc',
      },
    }),
    placeholderData: keepPreviousData,
  })

  const deleteUser = useDeleteUser()

  const bulkDeleteUsers = useBulkDeleteUsers()

  const updateUser = useUpdateUser()

  const bulkUpdateUsers = useBulkUpdateUsers()

  const createUser = useRegisterUser()

  const [rowSelection, setRowSelection] = React.useState({})

  const selectedUsers = React.useMemo(() => {
    return new Set(Object.keys(rowSelection))
  }, [rowSelection])

  const columns = getUserColumns({
    users: usersQuery.data?.data || [],
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
      <UsersToolbarComponent
        handleExportCSV={() =>
          handleExportCSV(usersQuery.data?.data || [], 'users_export.csv', [
            { header: 'ID', accessor: 'id' },
            { header: 'Email', accessor: 'email' },
            {
              header: 'Verified',
              accessor: (u) => (u.is_verified ? 'Yes' : 'No'),
            },
            {
              header: 'Created At',
              accessor: (u) => new Date(u.created_at).toLocaleString(),
            },
          ])
        }
        setIsCreateUserOpen={setIsCreateUserOpen}
      />
      <UsersFilters />
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

      <UserToolbar
        selectedUsers={selectedUsers}
        onBulkVerify={(v: boolean) =>
          bulkUpdateUsers.mutate(
            {
              body: {
                user_ids: Array.from(selectedUsers),
                is_verified: v,
              },
            },
            {
              onSuccess: () => setRowSelection({}),
            },
          )
        }
        onBulkDelete={() => setIsBulkDeleteOpen(true)}
        onBulkEdit={() => setIsBulkEditOpen(true)}
        users={usersQuery.data?.data}
      />

      <UserModals
        userToDelete={userToDelete}
        setUserToDelete={setUserToDelete}
        onDeleteConfirm={(id: string) =>
          deleteUser.mutate(
            { path: { user_id: id } },
            {
              onSuccess: () => setUserToDelete(null),
            },
          )
        }
        isBulkDeleteOpen={isBulkDeleteOpen}
        setIsBulkDeleteOpen={setIsBulkDeleteOpen}
        onBulkDeleteConfirm={() =>
          bulkDeleteUsers.mutate(
            {
              body: { userIds: Array.from(selectedUsers) },
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
              body: { user_ids: Array.from(selectedUsers), ...data },
            },
            {
              onSuccess: () => {
                setRowSelection({})
                setIsBulkEditOpen(false)
              },
            },
          )
        }
        selectedCount={selectedUsers.size}
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
              onSuccess: () => setIsCreateUserOpen(false),
            },
          )
        }
        isSubmitting={createUser.isPending}
      />
    </Stack>
  )
}
