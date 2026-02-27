import { createFileRoute } from '@tanstack/react-router'
import {
  keepPreviousData,
  useMutation,
  useQuery,
  useQueryClient,
} from '@tanstack/react-query'
import * as React from 'react'
import { toast } from 'sonner'
import { format } from 'date-fns'

import type { AcademicYearFormValues } from '@/features/academics/years/schemas'
import { authClient } from '@/lib/clients'
import { handleExportCSV } from '@/lib/export'
import {
  bulkDeleteAcademicYearsMutation,
  createAcademicYearMutation,
  deleteAcademicYearMutation,
  getAllAcademicYearsOptions,
  getAllAcademicYearsQueryKey,
  setCurrentAcademicYearMutation,
  updateAcademicYearMutation,
} from '@/lib/api/@tanstack/react-query.gen'
import { useAcademicYearsStore } from '@/features/academics/years/store'
import { AcademicYearsHeader } from '@/features/academics/years/components/academic-years-header'
import { AcademicYearsToolbar } from '@/features/academics/years/components/academic-years-toolbar'
import { AcademicYearsListContainer } from '@/features/academics/years/components/academic-years-list-container'
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

export const Route = createFileRoute('/admin/academics/years')({
  component: AcademicYearsPage,
})

function AcademicYearsPage() {
  const store = useAcademicYearsStore()
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
    sorting,
    debouncedSearch,
    setYearToEdit,
    setYearToDelete,
    setIsCreateYearOpen,
    isBulkDeleteOpen,
    setIsBulkDeleteOpen,
  } = store

  const sortBy = sorting[0]?.id
  const sortOrder = sorting[0]?.desc ? 'desc' : 'asc'

  const yearsQuery = useQuery({
    ...getAllAcademicYearsOptions({
      client: authClient,
      query: {
        page,
        limit,
        search: debouncedSearch,
        sort_by: sortBy,
        sort_order: sortOrder,
      },
    }),
    placeholderData: keepPreviousData,
  })

  const queryClient = useQueryClient()
  const invalidateQueries = () => {
    queryClient.invalidateQueries({
      queryKey: getAllAcademicYearsQueryKey(),
    })
  }

  const createYear = useMutation({
    ...createAcademicYearMutation({
      client: authClient,
    }),
    onSuccess: () => {
      toast.success('Academic year created successfully.')
      invalidateQueries()
      setIsCreateYearOpen(false)
    },
    onError: (error) => {
      toast.error(
        `Failed to create academic year: ${error.message || 'Unknown error'}`,
      )
    },
  })

  const updateYear = useMutation({
    ...updateAcademicYearMutation({
      client: authClient,
    }),
    onSuccess: () => {
      toast.success('Academic year updated successfully.')
      invalidateQueries()
      setYearToEdit(null)
    },
    onError: (error) => {
      toast.error(
        `Failed to update academic year: ${error.message || 'Unknown error'}`,
      )
    },
  })

  const deleteYear = useMutation({
    ...deleteAcademicYearMutation({
      client: authClient,
    }),
    onSuccess: () => {
      toast.success('Academic year deleted successfully.')
      invalidateQueries()
      setYearToDelete(null)
    },
    onError: (error) => {
      toast.error(
        `Failed to delete academic year: ${error.message || 'Unknown error'}`,
      )
    },
  })

  const setCurrentYear = useMutation({
    ...setCurrentAcademicYearMutation({
      client: authClient,
    }),
    onSuccess: () => {
      toast.success('Academic year set as current.')
      invalidateQueries()
    },
    onError: (error) => {
      toast.error(
        `Failed to set academic year: ${error.message || 'Unknown error'}`,
      )
    },
  })

  const bulkDeleteYears = useMutation({
    ...bulkDeleteAcademicYearsMutation({
      client: authClient,
    }),
    onSuccess: (_, variables) => {
      const count = variables.body?.academic_year_ids?.length ?? 0
      toast.success(`Successfully deleted ${count} academic years.`)
      invalidateQueries()
      setIsBulkDeleteOpen(false)
      setRowSelection({})
    },
    onError: (error) => {
      toast.error(
        `Failed to delete academic years: ${error.message || 'Unknown error'}`,
      )
    },
  })

  const [rowSelection, setRowSelection] = React.useState<
    Record<string, boolean>
  >({})
  const selectedYears = React.useMemo(() => {
    return new Set(Object.keys(rowSelection).filter((key) => rowSelection[key]))
  }, [rowSelection])

  const columns = getAcademicYearsColumns({
    onEdit: setYearToEdit,
    onDelete: setYearToDelete,
    onSetCurrent: (id) => setCurrentYear.mutate({ path: { id } }),
  })

  return (
    <div className="flex h-full flex-col bg-background">
      <AcademicYearsHeader />
      <AcademicYearsToolbar
        onExport={() =>
          handleExportCSV(
            yearsQuery.data?.data || [],
            'academic_years_export.csv',
            [
              { header: 'ID', accessor: 'id' },
              { header: 'Name', accessor: 'name' },
              {
                header: 'Start Date',
                accessor: (year) =>
                  format(new Date(String(year.year_start)), 'yyyy-MM-dd'),
              },
              {
                header: 'End Date',
                accessor: (year) =>
                  format(new Date(String(year.year_end)), 'yyyy-MM-dd'),
              },
              { header: 'Is Current', accessor: 'current' },
            ],
          )
        }
      />
      <AcademicYearsListContainer
        query={yearsQuery}
        columns={columns}
        rowSelection={rowSelection}
        setRowSelection={setRowSelection}
      />

      <AcademicYearAddDialog
        open={store.isCreateYearOpen}
        onOpenChange={setIsCreateYearOpen}
        onConfirm={(data: AcademicYearFormValues) =>
          createYear.mutate({
            body: {
              ...data,
              year_start: new Date(data.start_date).getTime(),
              year_end: new Date(data.end_date).getTime(),
            } as any,
          })
        }
        isSubmitting={createYear.isPending}
      />

      <AcademicYearEditDialog
        year={store.yearToEdit}
        open={!!store.yearToEdit}
        onOpenChange={() => setYearToEdit(null)}
        onConfirm={(data: AcademicYearFormValues) =>
          store.yearToEdit &&
          updateYear.mutate({
            path: { id: store.yearToEdit.id },
            body: {
              ...data,
              year_start: new Date(data.start_date).getTime(),
              year_end: new Date(data.end_date).getTime(),
            } as any,
          })
        }
        isSubmitting={updateYear.isPending}
      />

      <AlertDialog
        open={!!store.yearToDelete}
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
                store.yearToDelete &&
                deleteYear.mutate({ path: { id: store.yearToDelete } })
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
            <AlertDialogTitle>Are you absolutely sure?</AlertDialogTitle>
            <AlertDialogDescription>
              This action cannot be undone. This will permanently delete{' '}
              {selectedYears.size} academic years.
            </AlertDialogDescription>
          </AlertDialogHeader>
          <AlertDialogFooter>
            <AlertDialogCancel>Cancel</AlertDialogCancel>
            <AlertDialogAction
              onClick={() => {
                bulkDeleteYears.mutate({
                  body: { academic_year_ids: Array.from(selectedYears) },
                })
              }}
            >
              Delete All
            </AlertDialogAction>
          </AlertDialogFooter>
        </AlertDialogContent>
      </AlertDialog>
    </div>
  )
}
