import { createFileRoute } from '@tanstack/react-router'
import {
  keepPreviousData,
  useQuery,
  useQueryClient,
} from '@tanstack/react-query'
import * as React from 'react'

import { HugeiconsIcon } from '@hugeicons/react'
import {
  Delete02Icon,
  LayoutGridIcon,
  TableIcon,
} from '@hugeicons/core-free-icons'
import type { AcademicYearFormValues } from '@/features/academics/years/schemas'
import type { AcademicYearResponse } from '@/lib/api/types.gen'
import { AcademicYearsHeader } from '@/features/academics/years/components/academic-years-header'
import { AcademicYearsListContainer } from '@/features/academics/years/components/academic-years-list-container'
import { AcademicYearsGridView } from '@/features/academics/years/components/academic-years-grid-view'
import { getAcademicYearsColumns } from '@/features/academics/years/components/academic-years-table-columns'
import { AcademicYearAddDialog } from '@/features/academics/years/components/academic-year-add-dialog'
import { AcademicYearEditDialog } from '@/features/academics/years/components/academic-year-edit-dialog'
import {
  AlertDialog,
  AlertDialogAction,
  AlertDialogCancel,
  AlertDialogContent,
  AlertDialogDescription,
  AlertDialogFooter,
  AlertDialogHeader,
  AlertDialogTitle,
} from '@/components/ui/alert-dialog'
import { HStack, Stack } from '@/components/primitives'
import {
  getAllAcademicYearsQueryOptions,
  useBulkDeleteAcademicYears,
  useCreateAcademicYear,
  useDeleteAcademicYear,
  useSetCurrentAcademicYear,
  useUpdateAcademicYear,
} from '@/features/academics/years/api'
import { useAcademicYearsSearchParams } from '@/features/academics/years/search-params'
import { Button } from '@/components/ui/button'
import { Tabs, TabsContent, TabsList, TabsTrigger } from '@/components/ui/tabs'

export const Route = createFileRoute('/admin/years')({
  component: AcademicYearsPage,
})

function AcademicYearsPage() {
  const queryClient = useQueryClient()
  const { page, limit, search, sortBy, sortOrder } =
    useAcademicYearsSearchParams()

  const [yearToDelete, setYearToDelete] = React.useState<string | null>(null)
  const [isBulkDeleteOpen, setIsBulkDeleteOpen] = React.useState(false)
  const [isCreateYearOpen, setIsCreateYearOpen] = React.useState(false)
  const [yearToEdit, setYearToEdit] =
    React.useState<AcademicYearResponse | null>(null)

  const yearsQuery = useQuery({
    ...getAllAcademicYearsQueryOptions({
      query: {
        page: page ?? 1,
        limit: limit ?? 10,
        search: search ?? undefined,
        sort_by: sortBy ?? 'year_start',
        sort_order:
          sortOrder === 'asc' || sortOrder === 'desc' ? sortOrder : 'desc',
      },
    }),
    placeholderData: keepPreviousData,
  })

  const createYear = useCreateAcademicYear()
  const updateYear = useUpdateAcademicYear()
  const deleteYear = useDeleteAcademicYear()
  const setCurrentYear = useSetCurrentAcademicYear()
  const bulkDeleteYears = useBulkDeleteAcademicYears()

  const [rowSelection, setRowSelection] = React.useState<
    Record<string, boolean>
  >({})

  const fetchFullData = React.useCallback(async () => {
    const options = getAllAcademicYearsQueryOptions({
      query: {
        page: 1,
        limit: 1000,
        search: search ?? undefined,
        sort_by: sortBy ?? 'year_start',
        sort_order: sortOrder === 'desc' ? 'desc' : 'asc',
      },
    })

    if (!options.queryFn) return []
    const response = await options.queryFn({
      queryKey: options.queryKey,
      meta: undefined,
      client: queryClient,
      signal: new AbortSignal(),
    })
    return response.data || []
  }, [search, sortBy, sortOrder, queryClient])

  const columns = getAcademicYearsColumns({
    onEdit: setYearToEdit,
    onDelete: setYearToDelete,
    onSetCurrent: (id) => setCurrentYear.mutate({ path: { id } }),
  })

  return (
    <Stack gap={4} p={8} className="h-full">
      <AcademicYearsHeader />

      <Tabs
        defaultValue="table"
        className="flex flex-col flex-1 gap-4 overflow-hidden"
      >
        <HStack>
          <TabsList>
            <TabsTrigger value="table" className="gap-2">
              <HugeiconsIcon icon={TableIcon} className="size-4" />
              Table
            </TabsTrigger>
            <TabsTrigger value="grid" className="gap-2">
              <HugeiconsIcon icon={LayoutGridIcon} className="size-4" />
              Grid
            </TabsTrigger>
          </TabsList>
        </HStack>

        <TabsContent value="table" className="flex-1 w-full mt-0">
          <div className="overflow-y-auto w-0 flex-1 min-w-full h-full">
            <AcademicYearsListContainer
              query={yearsQuery}
              columns={columns}
              rowSelection={rowSelection}
              setRowSelection={setRowSelection}
              onFetchFullData={fetchFullData}
              onAdd={() => setIsCreateYearOpen(true)}
              onAddLabel="Add Year"
              bulkActions={({ selectedRows }) => (
                <Button
                  variant="destructive"
                  size="sm"
                  onClick={() => setIsBulkDeleteOpen(true)}
                >
                  <HugeiconsIcon icon={Delete02Icon} className="size-4 mr-2" />
                  Delete Selected ({selectedRows.length})
                </Button>
              )}
            />
          </div>
        </TabsContent>

        <TabsContent
          value="grid"
          className="flex-1 w-full mt-0 overflow-y-auto"
        >
          <AcademicYearsGridView
            data={yearsQuery.data?.data || []}
            isLoading={yearsQuery.isLoading}
            onEdit={setYearToEdit}
            onDelete={setYearToDelete}
            onSetCurrent={(id) => setCurrentYear.mutate({ path: { id } })}
          />
        </TabsContent>
      </Tabs>

      <AcademicYearAddDialog
        open={isCreateYearOpen}
        onOpenChange={setIsCreateYearOpen}
        onConfirm={(data: AcademicYearFormValues) =>
          createYear.mutate(
            {
              body: {
                id: data.id,
                name: data.name,
                year_start: new Date(data.start_date).getFullYear(),
                year_end: new Date(data.end_date).getFullYear(),
                current: !!data.current,
              },
            },
            { onSuccess: () => setIsCreateYearOpen(false) },
          )
        }
        isSubmitting={createYear.isPending}
      />

      <AcademicYearEditDialog
        year={yearToEdit}
        open={!!yearToEdit}
        onOpenChange={() => setYearToEdit(null)}
        onConfirm={(data: AcademicYearFormValues) =>
          yearToEdit &&
          updateYear.mutate(
            {
              path: { id: yearToEdit.id },
              body: {
                name: data.name,
                year_start: new Date(data.start_date).getFullYear(),
                year_end: new Date(data.end_date).getFullYear(),
                current: !!data.current,
              },
            },
            {
              onSuccess: () => {
                setRowSelection({})
                setYearToEdit(null)
              },
            },
          )
        }
        isSubmitting={updateYear.isPending}
      />

      <AlertDialog
        open={!!yearToDelete}
        onOpenChange={() => setYearToDelete(null)}
      >
        <AlertDialogContent>
          <AlertDialogHeader>
            <AlertDialogTitle>Are you absolutely sure?</AlertDialogTitle>
            <AlertDialogDescription>
              This action cannot be undone. This will permanently delete the
              academic year.
            </AlertDialogDescription>
          </AlertDialogHeader>
          <AlertDialogFooter>
            <AlertDialogCancel>Cancel</AlertDialogCancel>
            <AlertDialogAction
              onClick={() =>
                yearToDelete &&
                deleteYear.mutate(
                  { path: { id: yearToDelete } },
                  { onSuccess: () => setYearToDelete(null) },
                )
              }
            >
              Delete
            </AlertDialogAction>
          </AlertDialogFooter>
        </AlertDialogContent>
      </AlertDialog>

      <AlertDialog open={isBulkDeleteOpen} onOpenChange={setIsBulkDeleteOpen}>
        <AlertDialogContent>
          <AlertDialogHeader>
            <AlertDialogTitle>Bulk Delete Confirmation</AlertDialogTitle>
            <AlertDialogDescription>
              You are about to delete{' '}
              {Object.keys(rowSelection).filter((k) => rowSelection[k]).length}{' '}
              academic years.
            </AlertDialogDescription>
          </AlertDialogHeader>
          <AlertDialogFooter>
            <AlertDialogCancel>Cancel</AlertDialogCancel>
            <AlertDialogAction
              onClick={() => {
                const ids = Object.keys(rowSelection).filter(
                  (k) => rowSelection[k],
                )
                bulkDeleteYears.mutate(
                  { body: { academic_year_ids: ids } },
                  {
                    onSuccess: () => {
                      setIsBulkDeleteOpen(false)
                      setRowSelection({})
                    },
                  },
                )
              }}
            >
              Confirm
            </AlertDialogAction>
          </AlertDialogFooter>
        </AlertDialogContent>
      </AlertDialog>
    </Stack>
  )
}
