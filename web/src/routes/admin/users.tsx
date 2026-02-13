import {
    keepPreviousData,
    useMutation,
    useQuery,
    useQueryClient,
} from '@tanstack/react-query'
import { createFileRoute } from '@tanstack/react-router'
import * as React from 'react'
import { toast } from 'sonner'


import { UserBulkPermissionsDialog } from '../../features/permissions/components/user-bulk-permissions-dialog'
import { UserPermissionsDialog } from '../../features/permissions/components/user-permissions-dialog'
import { UserComparisonOverlay } from '../../features/users/components/user-comparison-overlay'
import { UserCreateDialog } from '../../features/users/components/user-create-dialog'
import { UserModals } from '../../features/users/components/user-modals'
import { getUserColumns } from '../../features/users/components/user-table-columns'
import { UsersFilters } from '../../features/users/components/users-filters'
import { UsersHeader } from '../../features/users/components/users-header'
import { UsersListContainer } from '../../features/users/components/users-list-container'
import { UsersToolbar } from '../../features/users/components/users-toolbar'
import { useUsersStore } from '../../features/users/store'

import type {
    BulkUpdateValues,
    UpdateUserValues,
} from '../../features/users/schemas'
import { authClient } from '@/lib/clients'

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
  const {
    page,
    search,
    debouncedSearch,
    statusFilter,
    authFilter,
    createdAfter,
    createdBefore,
    sorting,
    selectedUsers,
    userToDelete,
    isBulkDeleteOpen,
    isBulkEditOpen,
    isBulkPermissionsOpen,
    isCreateUserOpen,
    userToEdit,
    userToLock,
    userToManagePermissions,

    setDebouncedSearch,
    setUserToDelete,
    setIsBulkDeleteOpen,
    setIsBulkEditOpen,
    setIsBulkPermissionsOpen,
    setIsCreateUserOpen,
    setUserToEdit,
    setUserToLock,
    setUserToManagePermissions,
    setSelectedUsers,
    resetSelection,
  } = useUsersStore()

  const limit = 10

  const queryClient = useQueryClient()

  // Debounce search
  React.useEffect(() => {
    const handler = setTimeout(() => {
      setDebouncedSearch(search)
    }, 400)
    return () => clearTimeout(handler)
  }, [search, setDebouncedSearch])

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
        sort_order: sortOrder as any,
      },
    }),
    placeholderData: keepPreviousData,
  })

  const deleteMutation = useMutation({
    ...deleteUsers5D3C91131F7D9Efc5999C92Dbfac75DaMutation({
      client: authClient,
    }),
    onSuccess: () => {
      toast.success('User deleted')
      queryClient.invalidateQueries({
        queryKey: deleteUsers5D3C91131F7D9Efc5999C92Dbfac75DaMutation().mutationKey,
      })
      queryClient.invalidateQueries({
        queryKey: getUsersStatsBf304B57E4A0115F8280C4Bed2Fd9FbaOptions().queryKey,
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
        queryKey: deleteUsersBulk6B8Be22247333C35E8A37A5Db37Fbfa8Mutation().mutationKey,
      })
      queryClient.invalidateQueries({
        queryKey: getUsersStatsBf304B57E4A0115F8280C4Bed2Fd9FbaOptions().queryKey,
      })
      resetSelection()
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
        queryKey: patchUsers5D3C91131F7D9Efc5999C92Dbfac75DaMutation().mutationKey,
      })
    },
  })

  const columns = React.useMemo(
    () =>
      getUserColumns({
        users: usersQuery.data?.data,
        onToggleVerify: (user) =>
          updateMutation.mutate({
            path: { user_id: user.id },
            body: { is_verified: !user.is_verified },
          }),
        onToggleLock: (user) => {
          if (user.lockout_until) {
            updateMutation.mutate({
              path: { user_id: user.id },
              body: { lockout_until: null },
            })
          } else {
            setUserToLock(user)
          }
        },
        selectedUsers,
        setSelectedUsers,
        setUserToDelete,
        setUserToEdit,
        setUserToManagePermissions,
      }),
    [
      usersQuery.data?.data,
      updateMutation,
      selectedUsers,
      setSelectedUsers,
      setUserToDelete,
      setUserToEdit,
      setUserToLock,
      setUserToManagePermissions,
    ],
  )


  const bulkUpdateMutation = useMutation({
    ...patchUsersBulk6B8Be22247333C35E8A37A5Db37Fbfa8Mutation({
      client: authClient,
    }),
    onSuccess: () => {
      toast.success('Batch update complete')
      queryClient.invalidateQueries({
        queryKey: patchUsersBulk6B8Be22247333C35E8A37A5Db37Fbfa8Mutation().mutationKey,
      })
      resetSelection()
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
        queryKey: postAuthRegisterD7296Dbacc4Fd751Aeb142Bbb8A63Fd9Mutation().mutationKey,
      })
      queryClient.invalidateQueries({
        queryKey: getUsersStatsBf304B57E4A0115F8280C4Bed2Fd9FbaOptions().queryKey,
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

  return (
    <div className="flex h-full flex-col bg-background">
      <UsersHeader />
      <UsersToolbar handleExportCSV={handleExportCSV} />
      <UsersFilters />
      <UsersListContainer
        usersQuery={usersQuery}
        limit={limit}
        columns={columns}
        updateMutation={updateMutation}
      />

      <UserComparisonOverlay
        selectedUsers={selectedUsers}
        onClear={resetSelection}
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
        userToLock={userToLock}
        setUserToLock={setUserToLock}
        onLockConfirm={(date) =>
          userToLock &&
          updateMutation.mutate(
            {
              path: { user_id: userToLock.id },
              body: { lockout_until: date.toISOString() },
            },
            {
              onSuccess: () => setUserToLock(null),
            },
          )
        }
        isLocking={updateMutation.isPending}
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
