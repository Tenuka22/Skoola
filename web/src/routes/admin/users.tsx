import { createFileRoute } from '@tanstack/react-router'
import * as React from 'react'

import { UserBulkPermissionsDialog } from '../../features/permissions/components/user-bulk-permissions-dialog'
import { UserPermissionsDialog } from '../../features/permissions/components/user-permissions-dialog'
import { UserToolbar } from '../../features/users/components/user-toolbar'
import { UserCreateDialog } from '../../features/users/components/user-create-dialog'
import { UserModals } from '../../features/users/components/user-modals'
import { getUserColumns } from '../../features/users/components/user-table-columns'
import { UsersFilters } from '../../features/users/components/users-filters'
import { UsersHeader } from '../../features/users/components/users-header'
import { UsersListContainer } from '../../features/users/components/users-list-container'
import { UsersToolbar } from '../../features/users/components/users-toolbar'
import { useUsersStore } from '../../features/users/store'

import { handleExportCSV } from '../../lib/export'
import { useUserMutations } from '../../features/users/hooks/use-user-mutations'
import { useUsers } from '../../features/users/hooks/use-users'
import {
  BulkUpdateValues,
  UpdateUserValues,
} from '../../features/users/schemas'

export const Route = createFileRoute('/admin/users')({
  component: Users,
})

function Users() {
  const store = useUsersStore()
  const { search, setDebouncedSearch } = store

  const limit = 10

  React.useEffect(() => {
    const handler = setTimeout(() => {
      setDebouncedSearch(search)
    }, 400)
    return () => clearTimeout(handler)
  }, [search, setDebouncedSearch])

  const usersQuery = useUsers()
  const {
    deleteUser,
    bulkDeleteUsers,
    updateUser,
    bulkUpdateUsers,
    createUser,
  } = useUserMutations()

  const [rowSelection, setRowSelection] = React.useState({})

  const selectedUsers = React.useMemo(() => {
    return new Set(Object.keys(rowSelection))
  }, [rowSelection])

  const columns = getUserColumns({
    users: usersQuery.data?.data,
    onToggleVerify: (user) =>
      updateUser.mutate({
        path: { user_id: user.id },
        body: { is_verified: !user.is_verified },
      }),
    onToggleLock: (user) => {
      if (user.lockout_until) {
        updateUser.mutate({
          path: { user_id: user.id },
          body: { lockout_until: null },
        })
      } else {
        store.setUserToLock(user)
      }
    },
    setUserToDelete: store.setUserToDelete,
    setUserToEdit: store.setUserToEdit,
    setUserToManagePermissions: store.setUserToManagePermissions,
  })

  return (
    <div className="flex h-full flex-col bg-background">
      <UsersHeader />
      <UsersToolbar
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
      />
      <UsersFilters />
      <UsersListContainer
        usersQuery={usersQuery}
        limit={limit}
        columns={columns}
        updateMutation={updateUser}
        rowSelection={rowSelection}
        setRowSelection={setRowSelection}
      />

      <UserToolbar
        selectedUsers={selectedUsers}
        onClear={() => setRowSelection({})}
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
        onBulkDelete={() => store.setIsBulkDeleteOpen(true)}
        onBulkEdit={() => store.setIsBulkEditOpen(true)}
        onBulkManagePermissions={() => store.setIsBulkPermissionsOpen(true)}
        users={usersQuery.data?.data}
      />

      <UserModals
        userToDelete={store.userToDelete}
        setUserToDelete={store.setUserToDelete}
        onDeleteConfirm={(id: string) =>
          deleteUser.mutate({ path: { user_id: id } })
        }
        isBulkDeleteOpen={store.isBulkDeleteOpen}
        setIsBulkDeleteOpen={store.setIsBulkDeleteOpen}
        onBulkDeleteConfirm={() =>
          bulkDeleteUsers.mutate(
            {
              body: { user_ids: Array.from(selectedUsers) },
            },
            {
              onSuccess: () => setRowSelection({}),
            },
          )
        }
        isBulkEditOpen={store.isBulkEditOpen}
        setIsBulkEditOpen={store.setIsBulkEditOpen}
        onBulkEditConfirm={(data: BulkUpdateValues) =>
          bulkUpdateUsers.mutate(
            {
              body: { user_ids: Array.from(selectedUsers), ...data },
            },
            {
              onSuccess: () => setRowSelection({}),
            },
          )
        }
        selectedCount={selectedUsers.size}
        isBulkUpdating={bulkUpdateUsers.isPending}
        userToEdit={store.userToEdit}
        setUserToEdit={store.setUserToEdit}
        onEditConfirm={(data: UpdateUserValues) =>
          store.userToEdit &&
          updateUser.mutate({
            path: { user_id: store.userToEdit.id },
            body: data,
          })
        }
        isUpdating={updateUser.isPending}
        userToLock={store.userToLock}
        setUserToLock={store.setUserToLock}
        onLockConfirm={(date) =>
          store.userToLock &&
          updateUser.mutate(
            {
              path: { user_id: store.userToLock.id },
              body: { lockout_until: date.toISOString().slice(0, 19) },
            },
            {
              onSuccess: () => store.setUserToLock(null),
            },
          )
        }
        isLocking={updateUser.isPending}
      />

      <UserPermissionsDialog
        user={store.userToManagePermissions}
        open={!!store.userToManagePermissions}
        onOpenChange={(open) => !open && store.setUserToManagePermissions(null)}
      />

      <UserBulkPermissionsDialog
        userIds={Array.from(selectedUsers)}
        open={store.isBulkPermissionsOpen}
        onOpenChange={store.setIsBulkPermissionsOpen}
      />

      <UserCreateDialog
        open={store.isCreateUserOpen}
        onOpenChange={store.setIsCreateUserOpen}
        onConfirm={(data) => createUser.mutate({ body: { ...data } })}
        isSubmitting={createUser.isPending}
      />
    </div>
  )
}
