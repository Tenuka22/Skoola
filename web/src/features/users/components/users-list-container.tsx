import * as React from 'react'
import { TabsContent, Tabs } from '@/components/ui/tabs'
import { DataTable } from '@/components/ui/data-table'
import { UserBoardView } from './user-board-view'
import { UserListView } from './user-list-view'
import { useUsersStore } from '../store'
import { useMutation } from '@tanstack/react-query'
import { patchUsers5D3C91131F7D9Efc5999C92Dbfac75DaMutation } from '@/lib/api/@tanstack/react-query.gen'
import { authClient } from '@/lib/clients'
import { getUserColumns } from './user-table-columns'

interface UsersListContainerProps {
  usersQuery: any // TODO: Type this properly
  limit: number
}

export function UsersListContainer({ usersQuery, limit }: UsersListContainerProps) {
  const {
    page,
    view,
    sorting,
    columnVisibility,
    setPage,
    setSorting,
    setColumnVisibility,
    setUserToEdit,
    setUserToDelete,
    setUserToManagePermissions,
  } = useUsersStore()

  const updateMutation = useMutation({
    ...patchUsers5D3C91131F7D9Efc5999C92Dbfac75DaMutation({
      client: authClient,
    }),
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
        onToggleLock: (user) =>
          updateMutation.mutate({
            path: { user_id: user.id },
            body: { is_locked: true },
          }),
      }),
    [usersQuery.data?.data, updateMutation],
  )

  return (
    <div className="flex-1 px-8 py-4 space-y-4">
      <Tabs defaultValue="table" value={view}>
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
              fetchPreviousPage={() => setPage(page - 1)}
              fetchNextPage={() => setPage(page + 1)}
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
  )
}
