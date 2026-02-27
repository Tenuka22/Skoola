import { useUsersStore } from '../store'
import { UserBoardView } from './user-board-view'
import type { UseMutationResult, UseQueryResult } from '@tanstack/react-query'
import type { ColumnDef } from '@tanstack/react-table'
import type {
  MessageResponse,
  Options,
  PaginatedUserResponse,
  UpdateUserData,
  UserResponse,
} from '@/lib/api'
import { Tabs, TabsContent } from '@/components/ui/tabs'
import { DataTable } from '@/components/ui/data-table'

interface UsersListContainerProps {
  usersQuery: UseQueryResult<PaginatedUserResponse, Error>
  limit: number
  columns: Array<ColumnDef<UserResponse>>
  updateMutation: UseMutationResult<
    MessageResponse,
    Error,
    Options<UpdateUserData>,
    unknown
  >
  rowSelection: Record<string, boolean>
  setRowSelection: (
    selection:
      | Record<string, boolean>
      | ((prev: Record<string, boolean>) => Record<string, boolean>),
  ) => void
}

export function UsersListContainer({
  usersQuery,
  limit,
  columns,
  updateMutation,
  rowSelection,
  setRowSelection,
}: UsersListContainerProps) {
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
    setUserToLock,
    setUserToManagePermissions,
  } = useUsersStore()

  const isUpdating = updateMutation.isPending
  const updatingUserId = updateMutation.variables?.path?.user_id

  return (
    <Tabs defaultValue="table" value={view}>
      <TabsContent value="table" className="flex w-full">
        <div className="overflow-y-auto w-0 flex-1">
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
            rowSelection={rowSelection}
            onRowSelectionChange={setRowSelection}
            isLoading={usersQuery.isFetching}
          />
        </div>
      </TabsContent>

      <TabsContent value="board">
        <UserBoardView
          users={usersQuery.data?.data ?? []}
          isLoading={usersQuery.isFetching}
          onEdit={(user) => setUserToEdit(user)}
          onDelete={(id) => setUserToDelete(id)}
          onToggleVerify={(user) =>
            updateMutation.mutate({
              path: { user_id: user.id },
              body: { is_verified: !user.is_verified },
            })
          }
          onToggleLock={(user) => setUserToLock(user)}
          onManagePermissions={(user) => setUserToManagePermissions(user)}
          isUpdating={isUpdating}
          updatingUserId={updatingUserId}
        />
      </TabsContent>
    </Tabs>
  )
}
