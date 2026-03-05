import { createFileRoute } from '@tanstack/react-router'
import { useQuery } from '@tanstack/react-query'
import * as React from 'react'
import {
  getCoreRowModel,
  getFilteredRowModel,
  getPaginationRowModel,
  useReactTable,
} from '@tanstack/react-table'
import type {
  ColumnDef,
  OnChangeFn,
  PaginationState,
} from '@tanstack/react-table'
import type {
  CurriculumStandardResponse,
  GradeLevelResponse,
  SubjectResponse,
} from '@/lib/api/types.gen'
import {
  getAllCurriculumStandardsQueryOptions,
  useCreateCurriculumStandard,
  useDeleteCurriculumStandard,
  useUpdateCurriculumStandard,
} from '@/features/academics/curriculum/api'
import {
  getAllGradeLevelsOptions,
  getAllSubjectsOptions,
} from '@/lib/api/@tanstack/react-query.gen'
import { CurriculumHeader } from '@/features/academics/curriculum/components/curriculum-header'
import { useCurriculumColumns } from '@/features/academics/curriculum/components/curriculum-table-columns'
import { CurriculumDialog } from '@/features/academics/curriculum/components/curriculum-dialog'
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
import { Box, HStack, Stack } from '@/components/primitives'
import { Tabs, TabsContent, TabsList, TabsTrigger } from '@/components/ui/tabs'
import { DataTable } from '@/components/data-table'
import { CurriculumGrid } from '@/features/academics/curriculum/components/curriculum-grid'
import { Empty } from '@/components/empty'

export const Route = createFileRoute('/admin/academics/curriculum')({
  component: CurriculumPage,
})

type EnrichedCurriculumStandard = CurriculumStandardResponse & {
  subject_name?: string
  grade_level_name?: string
}

function CurriculumPage() {
  const [standardToEdit, setStandardToEdit] =
    React.useState<CurriculumStandardResponse | null>(null)
  const [standardToDelete, setStandardToDelete] = React.useState<string | null>(
    null,
  )
  const [isAddOpen, setIsAddOpen] = React.useState(false)
  const [globalFilter, setGlobalFilter] = React.useState('')
  const [pagination, setPagination] = React.useState<PaginationState>({
    pageIndex: 0,
    pageSize: 10,
  })

  const curriculumQuery = useQuery(getAllCurriculumStandardsQueryOptions())
  const subjectsQuery = useQuery(getAllSubjectsOptions())
  const gradeLevelsQuery = useQuery(getAllGradeLevelsOptions())

  const createMutation = useCreateCurriculumStandard()
  const updateMutation = useUpdateCurriculumStandard()
  const deleteMutation = useDeleteCurriculumStandard()

  const columns = useCurriculumColumns({
    onEdit: setStandardToEdit,
    onDelete: setStandardToDelete,
  })

  const { data, table } = useCurriculumTable({
    standards: curriculumQuery.data,
    subjects: subjectsQuery.data?.data,
    gradeLevels: gradeLevelsQuery.data?.data,
    columns,
    globalFilter,
    pagination,
    onPaginationChange: setPagination,
  })

  const isLoading =
    curriculumQuery.isLoading ||
    subjectsQuery.isLoading ||
    gradeLevelsQuery.isLoading

  const pageRows = table.getRowModel().rows

  return (
    <Stack gap={4} p={8} className="h-full">
      <CurriculumHeader total={data.length} onAdd={() => setIsAddOpen(true)} />

      <Tabs defaultValue="table" className="flex flex-col flex-1">
        <HStack justify="between" align="center">
          <TabsList>
            <TabsTrigger value="table">Table</TabsTrigger>
            <TabsTrigger value="grid">Grid</TabsTrigger>
          </TabsList>
        </HStack>

        <Box className="pt-4 flex-1 flex flex-col">
          {isLoading ? (
            <div className="flex justify-center items-center h-64">
              <p>Loading...</p>
            </div>
          ) : table.getFilteredRowModel().rows.length === 0 ? (
            <Empty
              title="No standards found"
              description={
                globalFilter
                  ? 'Try adjusting your search'
                  : 'Add a new standard to get started'
              }
              icon="empty"
              className="flex-1"
            />
          ) : (
            <Box className="flex-1">
              <TabsContent value="table" className="h-full">
                <Box className="flex-1 h-full overflow-hidden">
                  <DataTable
                    columns={columns}
                    data={data}
                    isLoading={isLoading}
                    pageCount={table.getPageCount()}
                    pageIndex={pagination.pageIndex}
                    pageSize={pagination.pageSize}
                    onPageIndexChange={table.setPageIndex}
                    onPageSizeChange={table.setPageSize}
                    canNextPage={table.getCanNextPage()}
                    canPreviousPage={table.getCanPreviousPage()}
                    fetchNextPage={table.nextPage}
                    fetchPreviousPage={table.previousPage}
                    rowSelection={{}}
                    onRowSelectionChange={() => {}}
                    search={globalFilter}
                    onSearchChange={setGlobalFilter}
                    searchPlaceholder="Search standards..."
                  />{' '}
                </Box>
              </TabsContent>
              <TabsContent value="grid">
                <Stack gap={4}>
                  <CurriculumGrid
                    standards={pageRows.map((r) => r.original)}
                    onEdit={setStandardToEdit}
                    onDelete={setStandardToDelete}
                  />
                </Stack>
              </TabsContent>
            </Box>
          )}
        </Box>
      </Tabs>

      <CurriculumDialog
        open={isAddOpen}
        onOpenChange={setIsAddOpen}
        onConfirm={(formData) =>
          createMutation.mutate(
            { body: formData },
            { onSuccess: () => setIsAddOpen(false) },
          )
        }
        isSubmitting={createMutation.isPending}
        title="Add Curriculum Standard"
      />

      <CurriculumDialog
        open={!!standardToEdit}
        onOpenChange={() => setStandardToEdit(null)}
        standard={standardToEdit}
        onConfirm={(formData) =>
          standardToEdit &&
          updateMutation.mutate(
            {
              path: { standard_id: standardToEdit.id },
              body: formData,
            },
            { onSuccess: () => setStandardToEdit(null) },
          )
        }
        isSubmitting={updateMutation.isPending}
        title="Edit Curriculum Standard"
      />

      <AlertDialog
        open={!!standardToDelete}
        onOpenChange={() => setStandardToDelete(null)}
      >
        <AlertDialogContent>
          <AlertDialogHeader>
            <AlertDialogTitle>Are you absolutely sure?</AlertDialogTitle>
            <AlertDialogDescription>
              This action cannot be undone. This will permanently delete the
              curriculum standard.
            </AlertDialogDescription>
          </AlertDialogHeader>
          <AlertDialogFooter>
            <AlertDialogCancel>Cancel</AlertDialogCancel>
            <AlertDialogAction
              onClick={() =>
                standardToDelete &&
                deleteMutation.mutate(
                  { path: { standard_id: standardToDelete } },
                  { onSuccess: () => setStandardToDelete(null) },
                )
              }
            >
              Delete
            </AlertDialogAction>
          </AlertDialogFooter>
        </AlertDialogContent>
      </AlertDialog>
    </Stack>
  )
}

function useCurriculumTable({
  standards,
  subjects,
  gradeLevels,
  columns,
  globalFilter,
  pagination,
  onPaginationChange,
}: {
  standards: Array<CurriculumStandardResponse> | undefined
  subjects: Array<SubjectResponse> | undefined
  gradeLevels: Array<GradeLevelResponse> | undefined
  columns: Array<ColumnDef<EnrichedCurriculumStandard>>
  globalFilter: string
  pagination: PaginationState
  onPaginationChange: OnChangeFn<PaginationState>
}) {
  const data = React.useMemo(() => {
    const standardsData = standards || []
    const subjectsData = subjects || []
    const gradeLevelsData = gradeLevels || []

    const subjectMap = new Map(
      subjectsData.map((s) => [s.id, s.subject_name_en]),
    )
    const gradeLevelMap = new Map(
      gradeLevelsData.map((gl) => [gl.id, gl.grade_name]),
    )

    return standardsData.map((standard) => ({
      ...standard,
      subject_name: subjectMap.get(standard.subject_id),
      grade_level_name: gradeLevelMap.get(standard.grade_level_id),
    }))
  }, [standards, subjects, gradeLevels])

  const table = useReactTable({
    data,
    columns,
    state: {
      globalFilter,
      pagination,
    },
    onPaginationChange: onPaginationChange,
    getCoreRowModel: getCoreRowModel(),
    getPaginationRowModel: getPaginationRowModel(),
    getFilteredRowModel: getFilteredRowModel(),
    globalFilterFn: (row, _, filterValue: string) => {
      const standard = row.original
      const s = filterValue.toLowerCase()
      return (
        standard.standard_code.toLowerCase().includes(s) ||
        (standard.version_name?.toLowerCase().includes(s) ?? false) ||
        (standard.description?.toLowerCase().includes(s) ?? false) ||
        (standard.subject_name?.toLowerCase().includes(s) ?? false) ||
        (standard.grade_level_name?.toLowerCase().includes(s) ?? false)
      )
    },
  })

  return { data, table }
}
