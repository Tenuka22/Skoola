import * as React from 'react'
import { useUsersSearchParams } from '../search-params'
import { UserGridView } from './user-grid-view'
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
import {
  Pagination,
  PaginationContent,
  PaginationEllipsis,
  PaginationItem,
  PaginationLink,
  PaginationNext,
  PaginationPrevious,
} from '@/components/ui/pagination'
import {
  Select,
  SelectContent,
  SelectItem,
  SelectTrigger,
  SelectValue,
} from '@/components/ui/select'
import { cn } from '@/lib/utils'

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
  contextMenuItems?: (row: UserResponse) => React.ReactNode
  setUserToEdit: (user: UserResponse | null) => void
  setUserToDelete: (id: string | null) => void
  setUserToLock: (user: UserResponse | null) => void
  setUserToManagePermissions: (user: UserResponse | null) => void
  onCreateUser: () => void
}

export function UsersListContainer({
  usersQuery,
  limit,
  columns,
  updateMutation,
  rowSelection,
  setRowSelection,
  contextMenuItems,
  setUserToEdit,
  setUserToDelete,
  setUserToLock,
  setUserToManagePermissions,
  onCreateUser,
}: UsersListContainerProps) {
  const {
    page,
    view,
    setPage,
    setLimit,
    sortBy,
    setSortBy,
    sortOrder,
    setSortOrder,
  } = useUsersSearchParams()

  const [columnVisibility, setColumnVisibility] = React.useState({})

  const isUpdating = updateMutation.isPending
  const updatingUserId = updateMutation.variables?.path?.user_id

  return (
    <Tabs defaultValue="table" value={view ?? 'table'}>
      <TabsContent value="table" className="flex w-full">
        <div className="overflow-y-auto w-0 flex-1">
          <DataTable
            columns={columns}
            data={usersQuery.data?.data || []}
            pageIndex={(page || 1) - 1}
            pageSize={limit}
            pageCount={usersQuery.data?.total_pages || 0}
            canPreviousPage={(page || 1) > 1}
            canNextPage={(page || 1) < (usersQuery.data?.total_pages || 0)}
            fetchPreviousPage={() => setPage((page || 1) - 1)}
            fetchNextPage={() => setPage((page || 1) + 1)}
            sorting={[
              { id: sortBy ?? 'created_at', desc: sortOrder === 'desc' },
            ]}
            onSortingChange={(updaterOrValue) => {
              const newSorting =
                typeof updaterOrValue === 'function'
                  ? updaterOrValue([
                      {
                        id: sortBy ?? 'created_at',
                        desc: sortOrder === 'desc',
                      },
                    ])
                  : updaterOrValue
              const firstSort = newSorting[0]
              if (firstSort) {
                setSortBy(firstSort.id)
                setSortOrder(firstSort.desc ? 'desc' : 'asc')
              }
            }}
            columnVisibility={columnVisibility}
            onColumnVisibilityChange={setColumnVisibility}
            rowSelection={rowSelection}
            onRowSelectionChange={setRowSelection}
            isLoading={usersQuery.isFetching}
            onPageSizeChange={setLimit}
            onPageIndexChange={(index: number) => setPage(index + 1)}
            contextMenuItems={contextMenuItems}
          />
        </div>
      </TabsContent>

      <TabsContent value="grid" className="flex flex-col gap-4">
        <UserGridView
          users={usersQuery.data?.data ?? []}
          limit={limit}
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
          onCreateUser={onCreateUser}
        />
        {(usersQuery.data?.data?.length || 0) > 0 && (
          <div className="rounded-lg border border-border/40 bg-card shadow-sm mt-4">
            <div className="flex items-center justify-between py-4 px-4">
              <div className="flex items-center space-x-6 lg:space-x-8">
                <div className="text-xs font-medium text-muted-foreground whitespace-nowrap hidden sm:block">
                  Showing{' '}
                  <span className="text-foreground">
                    {((page || 1) - 1) * limit + 1}
                  </span>{' '}
                  to{' '}
                  <span className="text-foreground">
                    {Math.min((page || 1) * limit, usersQuery.data?.total || 0)}
                  </span>{' '}
                  of{' '}
                  <span className="text-foreground">
                    {usersQuery.data?.total || 0}
                  </span>{' '}
                  results
                </div>
                <div className="flex items-center space-x-2">
                  <p className="text-xs font-medium text-muted-foreground whitespace-nowrap hidden sm:block">
                    Cards per page
                  </p>
                  <Select
                    value={`${limit}`}
                    onValueChange={(value) => setLimit(Number(value))}
                  >
                    <SelectTrigger className="h-8 w-[70px]">
                      <SelectValue placeholder={limit} />
                    </SelectTrigger>
                    <SelectContent side="top">
                      {[10, 20, 30, 40, 50].map((pageSizeOption) => (
                        <SelectItem
                          key={pageSizeOption}
                          value={`${pageSizeOption}`}
                        >
                          {pageSizeOption}
                        </SelectItem>
                      ))}
                    </SelectContent>
                  </Select>
                </div>
              </div>

              <Pagination className="mx-0 w-auto justify-end">
                <PaginationContent>
                  <PaginationItem>
                    <PaginationPrevious
                      className={cn(
                        (page || 1) <= 1 || usersQuery.isFetching
                          ? 'pointer-events-none opacity-50'
                          : 'cursor-pointer',
                      )}
                      onClick={
                        (page || 1) > 1 && !usersQuery.isFetching
                          ? () => setPage((page || 1) - 1)
                          : undefined
                      }
                    />
                  </PaginationItem>

                  <div className="hidden sm:flex items-center">
                    {(() => {
                      const currentPage = page || 1
                      const pageCount = usersQuery.data?.total_pages || 1
                      const maxVisiblePages = 5

                      if (pageCount <= maxVisiblePages) {
                        return Array.from({ length: pageCount }).map((_, i) => (
                          <PaginationItem key={i}>
                            <PaginationLink
                              isActive={currentPage === i + 1}
                              onClick={() => setPage(i + 1)}
                              className="cursor-pointer"
                            >
                              {i + 1}
                            </PaginationLink>
                          </PaginationItem>
                        ))
                      }

                      const pages = []

                      pages.push(
                        <PaginationItem key={1}>
                          <PaginationLink
                            isActive={currentPage === 1}
                            onClick={() => setPage(1)}
                            className="cursor-pointer"
                          >
                            1
                          </PaginationLink>
                        </PaginationItem>,
                      )

                      if (currentPage > 3) {
                        pages.push(
                          <PaginationItem key="ellipsis-start">
                            <PaginationEllipsis />
                          </PaginationItem>,
                        )
                      }

                      const startPage = Math.max(2, currentPage - 1)
                      const endPage = Math.min(pageCount - 1, currentPage + 1)

                      for (let i = startPage; i <= endPage; i++) {
                        pages.push(
                          <PaginationItem key={i}>
                            <PaginationLink
                              isActive={currentPage === i}
                              onClick={() => setPage(i)}
                              className="cursor-pointer"
                            >
                              {i}
                            </PaginationLink>
                          </PaginationItem>,
                        )
                      }

                      if (currentPage < pageCount - 2) {
                        pages.push(
                          <PaginationItem key="ellipsis-end">
                            <PaginationEllipsis />
                          </PaginationItem>,
                        )
                      }

                      pages.push(
                        <PaginationItem key={pageCount}>
                          <PaginationLink
                            isActive={currentPage === pageCount}
                            onClick={() => setPage(pageCount)}
                            className="cursor-pointer"
                          >
                            {pageCount}
                          </PaginationLink>
                        </PaginationItem>,
                      )

                      return pages
                    })()}
                  </div>
                  <PaginationItem className="sm:hidden">
                    <span className="text-sm px-4">
                      Page {page || 1} of {usersQuery.data?.total_pages || 1}
                    </span>
                  </PaginationItem>

                  <PaginationItem>
                    <PaginationNext
                      className={cn(
                        (page || 1) >= (usersQuery.data?.total_pages || 1) ||
                          usersQuery.isFetching
                          ? 'pointer-events-none opacity-50'
                          : 'cursor-pointer',
                      )}
                      onClick={
                        (page || 1) < (usersQuery.data?.total_pages || 1) &&
                        !usersQuery.isFetching
                          ? () => setPage((page || 1) + 1)
                          : undefined
                      }
                    />
                  </PaginationItem>
                </PaginationContent>
              </Pagination>
            </div>
          </div>
        )}
      </TabsContent>
    </Tabs>
  )
}
