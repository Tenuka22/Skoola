import { createFileRoute } from '@tanstack/react-router'
import { keepPreviousData, useQuery } from '@tanstack/react-query'
import * as React from 'react'

import type { AcademicYearFormValues } from '@/features/academics/years/schemas'
import type { AcademicYearResponse } from '@/lib/api/types.gen'
import { handleExportCSV } from '@/lib/export'
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
import { Stack } from '@/components/primitives'
import {
  getAllAcademicYearsQueryOptions,
  useBulkDeleteAcademicYears,
  useCreateAcademicYear,
  useDeleteAcademicYear,
  useSetCurrentAcademicYear,
  useUpdateAcademicYear,
} from '@/features/academics/years/api'
import { useAcademicYearsSearchParams } from '@/features/academics/years/search-params'

export const Route = createFileRoute('/admin/years')({
  component: AcademicYearsPage,
})

function AcademicYearsPage() {
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
  const selectedYears = React.useMemo(() => {
    return new Set(Object.keys(rowSelection).filter((key) => rowSelection[key]))
  }, [rowSelection])

  const columns = getAcademicYearsColumns({
    onEdit: setYearToEdit,
    onDelete: setYearToDelete,
    onSetCurrent: (id) => setCurrentYear.mutate({ path: { id } }),
  })

  return (
    <Stack gap={4} p={8} className="h-full bg-background">
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
                accessor: (year) => String(year.year_start),
              },
              {
                header: 'End Date',
                accessor: (year) => String(year.year_end),
              },
              { header: 'Is Current', accessor: 'current' },
            ],
          )
        }
        setIsCreateYearOpen={setIsCreateYearOpen}
      />
      <AcademicYearsListContainer
        query={yearsQuery}
        columns={columns}
        rowSelection={rowSelection}
        setRowSelection={setRowSelection}
      />

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
            {
              onSuccess: () => {
                setIsCreateYearOpen(false)
              },
            },
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
                  {
                    onSuccess: () => {
                      setYearToDelete(null)
                    },
                  },
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
                bulkDeleteYears.mutate(
                  {
                    body: { academic_year_ids: Array.from(selectedYears) },
                  },
                  {
                    onSuccess: () => {
                      setIsBulkDeleteOpen(false)
                      setRowSelection({})
                    },
                  },
                )
              }}
            >
              Delete All
            </AlertDialogAction>
          </AlertDialogFooter>
        </AlertDialogContent>
      </AlertDialog>
    </Stack>
  )
}
