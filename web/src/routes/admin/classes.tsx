import { createFileRoute } from '@tanstack/react-router'
import { keepPreviousData, useQuery } from '@tanstack/react-query'
import * as React from 'react'

import type { ClassFormValues } from '@/features/academics/classes/schemas'
import type { ClassResponse } from '@/lib/api/types.gen'
import { handleExportCSV } from '@/lib/export'
import { ClassesHeader } from '@/features/academics/classes/components/classes-header'
import { ClassesToolbar } from '@/features/academics/classes/components/classes-toolbar'
import { ClassesListContainer } from '@/features/academics/classes/components/classes-list-container'
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
import { Stack } from '@/components/primitives'
import {
  getAllClassesQueryOptions,
  useBulkDeleteClasses,
  useCreateClass,
  useDeleteClass,
  useUpdateClass,
} from '@/features/academics/classes/api'
import { useClassesSearchParams } from '@/features/academics/classes/search-params'

export const Route = createFileRoute('/admin/classes')({
  component: ClassesPage,
})

function ClassesPage() {
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

  const createClass = useCreateClass()

  const updateClass = useUpdateClass()

  const deleteClass = useDeleteClass()

  const bulkDeleteClasses = useBulkDeleteClasses()

  const [rowSelection, setRowSelection] = React.useState<
    Record<string, boolean>
  >({})
  const selectedClasses = React.useMemo(() => {
    return new Set(Object.keys(rowSelection).filter((key) => rowSelection[key]))
  }, [rowSelection])

  const columns = useClassesColumns({
    onEdit: setClassToEdit,
    onDelete: setClassToDelete,
    onAssignStudents: (classItem) => {
      setClassToAssignStudentsFor(classItem)
      setIsAssignStudentsOpen(true)
    },
  })

  return (
    <Stack gap={4} p={8} className="h-full bg-background">
      <ClassesHeader />
      <ClassesToolbar
        onExport={() =>
          handleExportCSV(classesQuery.data?.data || [], 'classes_export.csv', [
            { header: 'ID', accessor: 'id' },
            { header: 'Name', accessor: 'section_name' },
            { header: 'Grade Level', accessor: 'grade_id' },
            { header: 'Academic Year', accessor: 'academic_year_id' },
          ])
        }
        setIsCreateClassOpen={setIsCreateClassOpen}
      />
      <ClassesListContainer
        query={classesQuery}
        columns={columns}
        rowSelection={rowSelection}
        setRowSelection={setRowSelection}
      />

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
              {selectedClasses.size} classes.
            </AlertDialogDescription>
          </AlertDialogHeader>
          <AlertDialogFooter>
            <AlertDialogCancel>Cancel</AlertDialogCancel>
            <AlertDialogAction
              onClick={() => {
                bulkDeleteClasses.mutate(
                  {
                    body: { class_ids: Array.from(selectedClasses) },
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
