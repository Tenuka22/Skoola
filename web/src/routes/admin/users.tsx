import { createFileRoute } from '@tanstack/react-router'
import {
  keepPreviousData,
  useMutation,
  useQuery,
  useQueryClient,
} from '@tanstack/react-query'
import * as React from 'react'
import { toast } from 'sonner'

import { UserBulkPermissionsDialog } from '../../features/permissions/components/user-bulk-permissions-dialog'
import { UserPermissionsDialog } from '../../features/permissions/components/user-permissions-dialog'
import { UserCreateDialog } from '../../features/users/components/user-create-dialog'
import { UserModals } from '../../features/users/components/user-modals'
import { UserToolbar } from '../../features/users/components/user-toolbar'
import { getUserColumns } from '../../features/users/components/user-table-columns'
import { UsersFilters } from '../../features/users/components/users-filters'
import { UsersHeader } from '../../features/users/components/users-header'
import { UsersListContainer } from '../../features/users/components/users-list-container'
import { UsersToolbar } from '../../features/users/components/users-toolbar'
import { useUsersStore } from '../../features/users/store'
import { handleExportCSV } from '../../lib/export'
import type {
  BulkUpdateValues,
  UpdateUserValues,
} from '../../features/users/schemas'
import { authClient } from '@/lib/clients'
import {
  deleteUsers5D3C91131F7D9Efc5999C92Dbfac75DaMutation,
  deleteUsersBulk6B8Be22247333C35E8A37A5Db37Fbfa8Mutation,
  getUsers06Bdcf95Aafda840B1D04322636De293Options,
  getUsers06Bdcf95Aafda840B1D04322636De293QueryKey,
  getUsersStatsBf304B57E4A0115F8280C4Bed2Fd9FbaQueryKey,
  patchUsers5D3C91131F7D9Efc5999C92Dbfac75DaMutation,
  patchUsersBulk6B8Be22247333C35E8A37A5Db37Fbfa8Mutation,
  postAuthRegisterD7296Dbacc4Fd751Aeb142Bbb8A63Fd9Mutation,
} from '@/lib/api/@tanstack/react-query.gen'

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

  const {
    page,
    statusFilter,
    authFilter,
    createdAfter,
    createdBefore,
    sorting,
    debouncedSearch,
    setUserToDelete,
    setIsBulkDeleteOpen,
    setIsBulkEditOpen,
    setIsCreateUserOpen,
    setUserToLock,
    setUserToEdit,
  } = store

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
        created_after: createdAfter ?? undefined,
        created_before: createdBefore ?? undefined,
        sort_by: sortBy,
        sort_order: sortOrder,
      },
    }),
    placeholderData: keepPreviousData,
  })

  const queryClient = useQueryClient()
  const invalidateUsers = () => {
    queryClient.invalidateQueries({
      queryKey: getUsers06Bdcf95Aafda840B1D04322636De293QueryKey(),
    })
    queryClient.invalidateQueries({
      queryKey: getUsersStatsBf304B57E4A0115F8280C4Bed2Fd9FbaQueryKey(),
    })
  }

  const deleteUser = useMutation({
    ...deleteUsers5D3C91131F7D9Efc5999C92Dbfac75DaMutation({
      client: authClient,
    }),
    onSuccess: (_, variables) => {
      const userIdentifier = variables?.path.user_id || 'User'
      toast.success(`Successfully deleted ${userIdentifier}.`)
      invalidateUsers()
      setUserToDelete(null)
    },
    onError: (error, variables) => {
      const userIdentifier = variables?.path.user_id || 'User'
      toast.error(
        `Failed to delete ${userIdentifier}: ${error.message || 'Unknown error'}`,
      )
    },
  })

  const bulkDeleteUsers = useMutation({
    ...deleteUsersBulk6B8Be22247333C35E8A37A5Db37Fbfa8Mutation({
      client: authClient,
    }),
    onSuccess: (_, variables) => {
      const count = variables?.body.user_ids.length || 0
      toast.success(
        `Successfully deleted ${count} user${count !== 1 ? 's' : ''}.`,
      )
      invalidateUsers()
      setIsBulkDeleteOpen(false)
    },
    onError: (error) => {
      toast.error(`Failed to delete users: ${error.message || 'Unknown error'}`)
    },
  })

  const updateUser = useMutation({
    ...patchUsers5D3C91131F7D9Efc5999C92Dbfac75DaMutation({
      client: authClient,
    }),
    onSuccess: (_, variables) => {
      const userIdentifier = variables?.path.user_id || 'User'
      toast.success(`Successfully updated ${userIdentifier}.`)
      invalidateUsers()
      setUserToEdit(null)
      setUserToLock(null)
    },
    onError: (error, variables) => {
      const userIdentifier = variables?.path.user_id || 'User'
      toast.error(
        `Failed to update ${userIdentifier}: ${error.message || 'Unknown error'}`,
      )
    },
  })

  const bulkUpdateUsers = useMutation({
    ...patchUsersBulk6B8Be22247333C35E8A37A5Db37Fbfa8Mutation({
      client: authClient,
    }),
    onSuccess: (_, variables) => {
      const count = variables?.body.user_ids.length || 0
      toast.success(
        `Successfully updated ${count} user${count !== 1 ? 's' : ''}.`,
      )
      invalidateUsers()
      setIsBulkEditOpen(false)
    },
    onError: (error) => {
      toast.error(`Failed to update users: ${error.message || 'Unknown error'}`)
    },
  })

  const createUser = useMutation({
    ...postAuthRegisterD7296Dbacc4Fd751Aeb142Bbb8A63Fd9Mutation({
      client: authClient,
    }),
    onSuccess: (_, variables) => {
      const userIdentifier = variables?.body.email || 'New user'
      toast.success(`User ${userIdentifier} created successfully.`)
      invalidateUsers()
      setIsCreateUserOpen(false)
    },
    onError: (error, variables) => {
      const userIdentifier = variables?.body.email || 'User'
      toast.error(
        `Failed to create ${userIdentifier}: ${error.message || 'Unknown error'}`,
      )
    },
  })

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
          body: {
            lockout_until: null,
          },
        })
      } else {
        store.setUserToLock(user)
      }
    },
    setUserToDelete: store.setUserToDelete,
    setUserToEdit: store.setUserToEdit,
    setUserToManagePermissions: store.setUserToManagePermissions,
    isUpdating: updateUser.isPending,
    updatingUserId: updateUser.variables?.path.user_id,
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
