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
import type { ClassFormValues } from '@/features/academics/classes/schemas'
import type { ClassResponse } from '@/lib/api/types.gen'
import { ClassesHeader } from '@/features/academics/classes/components/classes-header'
import { ClassesListContainer } from '@/features/academics/classes/components/classes-list-container'
import { ClassesGridView } from '@/features/academics/classes/components/classes-grid-view'
import { useClassesColumns } from '@/features/academics/classes/components/classes-table-columns'
import { ClassAddDialog } from '@/features/academics/classes/components/class-add-dialog'
import { ClassEditDialog } from '@/features/academics/classes/components/class-edit-dialog'
import { ClassAssignStudentsDialog } from '@/features/academics/classes/components/class-assign-students-dialog'
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
  getAllClassesQueryOptions,
  useBulkDeleteClasses,
  useCreateClass,
  useDeleteClass,
  useUpdateClass,
} from '@/features/academics/classes/api'
import { useClassesSearchParams } from '@/features/academics/classes/search-params'
import { authClient } from '@/lib/clients'
import {
  getAllAcademicYearsOptions,
  getAllGradeLevelsOptions,
} from '@/lib/api/@tanstack/react-query.gen'
import { Button } from '@/components/ui/button'
import { Tabs, TabsContent, TabsList, TabsTrigger } from '@/components/ui/tabs'

export const Route = createFileRoute('/admin/classes')({
  component: ClassesPage,
})

function ClassesPage() {
  const queryClient = useQueryClient()
  const { page, limit, search, gradeId, academicYearId, sortBy, sortOrder } =
    useClassesSearchParams()

  const [classToDelete, setClassToDelete] = React.useState<string | null>(null)
  const [isBulkDeleteOpen, setIsBulkDeleteOpen] = React.useState(false)
  const [isCreateClassOpen, setIsCreateClassOpen] = React.useState(false)
  const [classToEdit, setClassToEdit] = React.useState<ClassResponse | null>(
    null,
  )
  const [isAssignStudentsOpen, setIsAssignStudentsOpen] = React.useState(false)
  const [classToAssignStudentsFor, setClassToAssignStudentsFor] =
    React.useState<ClassResponse | null>(null)

  const classesQuery = useQuery({
    ...getAllClassesQueryOptions({
      query: {
        page: page ?? 1,
        limit: limit ?? 10,
        search: search ?? undefined,
        sort_by: sortBy ?? 'created_at',
        sort_order:
          sortOrder === 'asc' || sortOrder === 'desc' ? sortOrder : 'desc',
        grade_id: gradeId ?? undefined,
        academic_year_id: academicYearId ?? undefined,
      },
    }),
    placeholderData: keepPreviousData,
  })

  const { data: gradeLevelsData } = useQuery(
    getAllGradeLevelsOptions({ client: authClient }),
  )
  const gradeLevels = React.useMemo(
    () => gradeLevelsData?.data || [],
    [gradeLevelsData],
  )

  const { data: academicYearsData } = useQuery(
    getAllAcademicYearsOptions({ client: authClient }),
  )
  const academicYears = React.useMemo(
    () => academicYearsData?.data || [],
    [academicYearsData],
  )

  const createClass = useCreateClass()
  const updateClass = useUpdateClass()
  const deleteClass = useDeleteClass()
  const bulkDeleteClasses = useBulkDeleteClasses()

  const [rowSelection, setRowSelection] = React.useState<
    Record<string, boolean>
  >({})

  const fetchFullData = React.useCallback(async () => {
    const options = getAllClassesQueryOptions({
      query: {
        page: 1,
        limit: 1000,
        search: search ?? undefined,
        grade_id: gradeId ?? undefined,
        academic_year_id: academicYearId ?? undefined,
        sort_by: sortBy ?? 'created_at',
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
  }, [search, gradeId, academicYearId, sortBy, sortOrder, queryClient])

  const facetedFilters = React.useMemo(
    () => [
      {
        columnId: 'grade_id',
        title: 'Grade Level',
        options: gradeLevels.map((gl) => ({
          label: gl.grade_name,
          value: gl.id,
        })),
      },
      {
        columnId: 'academic_year_id',
        title: 'Academic Year',
        options: academicYears.map((ay) => ({
          label: ay.name,
          value: ay.id,
        })),
      },
    ],
    [gradeLevels, academicYears],
  )

  const columns = useClassesColumns({
    onEdit: setClassToEdit,
    onDelete: setClassToDelete,
    onAssignStudents: (classItem) => {
      setClassToAssignStudentsFor(classItem)
      setIsAssignStudentsOpen(true)
    },
  })

  return (
    <Stack gap={4} p={8} className="h-full">
      <ClassesHeader />

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
            <ClassesListContainer
              query={classesQuery}
              columns={columns}
              rowSelection={rowSelection}
              setRowSelection={setRowSelection}
              onFetchFullData={fetchFullData}
              facetedFilters={facetedFilters}
              onAdd={() => setIsCreateClassOpen(true)}
              onAddLabel="Add Class"
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
          <ClassesGridView
            data={classesQuery.data?.data || []}
            isLoading={classesQuery.isLoading}
            onEdit={setClassToEdit}
            onDelete={setClassToDelete}
            onAssignStudents={(classItem) => {
              setClassToAssignStudentsFor(classItem)
              setIsAssignStudentsOpen(true)
            }}
          />
        </TabsContent>
      </Tabs>

      <ClassAddDialog
        open={isCreateClassOpen}
        onOpenChange={setIsCreateClassOpen}
        onConfirm={(data: ClassFormValues) =>
          createClass.mutate(
            { body: data },
            {
              onSuccess: () => {
                setIsCreateClassOpen(false)
              },
            },
          )
        }
        isSubmitting={createClass.isPending}
      />

      <ClassEditDialog
        classItem={classToEdit}
        open={!!classToEdit}
        onOpenChange={() => setClassToEdit(null)}
        onConfirm={(data: ClassFormValues) =>
          classToEdit &&
          updateClass.mutate(
            {
              path: { id: classToEdit.id },
              body: data,
            },
            {
              onSuccess: () => {
                setClassToEdit(null)
              },
            },
          )
        }
        isSubmitting={updateClass.isPending}
      />

      <ClassAssignStudentsDialog
        classItem={classToAssignStudentsFor}
        open={isAssignStudentsOpen}
        onOpenChange={setIsAssignStudentsOpen}
      />

      <AlertDialog
        open={!!classToDelete}
        onOpenChange={() => setClassToDelete(null)}
      >
        <AlertDialogContent>
          <AlertDialogHeader>
            <AlertDialogTitle>Are you absolutely sure?</AlertDialogTitle>
            <AlertDialogDescription>
              This action cannot be undone. This will permanently delete the
              class.
            </AlertDialogDescription>
          </AlertDialogHeader>
          <AlertDialogFooter>
            <AlertDialogCancel>Cancel</AlertDialogCancel>
            <AlertDialogAction
              onClick={() =>
                classToDelete &&
                deleteClass.mutate(
                  { path: { id: classToDelete } },
                  {
                    onSuccess: () => {
                      setClassToDelete(null)
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
              {Object.keys(rowSelection).filter((k) => rowSelection[k]).length}{' '}
              classes.
            </AlertDialogDescription>
          </AlertDialogHeader>
          <AlertDialogFooter>
            <AlertDialogCancel>Cancel</AlertDialogCancel>
            <AlertDialogAction
              onClick={() => {
                const ids = Object.keys(rowSelection).filter(
                  (k) => rowSelection[k],
                )
                bulkDeleteClasses.mutate(
                  {
                    body: { class_ids: ids },
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
              Confirm
            </AlertDialogAction>
          </AlertDialogFooter>
        </AlertDialogContent>
      </AlertDialog>
    </Stack>
  )
}
