import { createFileRoute } from '@tanstack/react-router'
import {
  keepPreviousData,
  useMutation,
  useQuery,
  useQueryClient,
} from '@tanstack/react-query'
import * as React from 'react'
import { toast } from 'sonner'

import type { ClassFormValues } from '@/features/academics/classes/schemas'
import { authClient } from '@/lib/clients'
import { handleExportCSV } from '@/lib/export'
import {
  bulkDeleteClassesMutation,
  createClassMutation,
  deleteClassMutation,
  getAllClassesOptions,
  getAllClassesQueryKey,
  updateClassMutation,
} from '@/lib/api/@tanstack/react-query.gen'
import { useClassesStore } from '@/features/academics/classes/store'
import { ClassesHeader } from '@/features/academics/classes/components/classes-header'
import { ClassesToolbar } from '@/features/academics/classes/components/classes-toolbar'
import { ClassesListContainer } from '@/features/academics/classes/components/classes-list-container'
import { useClassesColumns } from '@/features/academics/classes/components/classes-table-columns'
import { ClassAddDialog } from '@/features/academics/classes/components/class-add-dialog'
import { ClassEditDialog } from '@/features/academics/classes/components/class-edit-dialog'
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

export const Route = createFileRoute('/admin/academics/classes')({
  component: ClassesPage,
})

function ClassesPage() {
  const store = useClassesStore()
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
    setClassToEdit,
    setClassToDelete,
    setIsCreateClassOpen,
    isBulkDeleteOpen,
    setIsBulkDeleteOpen,
  } = store

  const sortBy = sorting[0]?.id
  const sortOrder = sorting[0]?.desc ? 'desc' : 'asc'

  const classesQuery = useQuery({
    ...getAllClassesOptions({
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
      queryKey: getAllClassesQueryKey(),
    })
  }

  const createClass = useMutation({
    ...createClassMutation({
      client: authClient,
    }),
    onSuccess: () => {
      toast.success('Class created successfully.')
      invalidateQueries()
      setIsCreateClassOpen(false)
    },
    onError: (error) => {
      toast.error(
        `Failed to create class: ${error.message || 'Unknown error'}`,
      )
    },
  })

  const updateClass = useMutation({
    ...updateClassMutation({
      client: authClient,
    }),
    onSuccess: () => {
      toast.success('Class updated successfully.')
      invalidateQueries()
      setClassToEdit(null)
    },
    onError: (error) => {
      toast.error(
        `Failed to update class: ${error.message || 'Unknown error'}`,
      )
    },
  })

  const deleteClass = useMutation({
    ...deleteClassMutation({
      client: authClient,
    }),
    onSuccess: () => {
      toast.success('Class deleted successfully.')
      invalidateQueries()
      setClassToDelete(null)
    },
    onError: (error) => {
      toast.error(
        `Failed to delete class: ${error.message || 'Unknown error'}`,
      )
    },
  })

  const bulkDeleteClasses = useMutation({
    ...bulkDeleteClassesMutation({
      client: authClient,
    }),
    onSuccess: (_, variables) => {
      const count = variables.body?.class_ids?.length ?? 0
      toast.success(`Successfully deleted ${count} classes.`)
      invalidateQueries()
      setIsBulkDeleteOpen(false)
      setRowSelection({})
    },
    onError: (error) => {
      toast.error(
        `Failed to delete classes: ${error.message || 'Unknown error'}`,
      )
    },
  })

  const [rowSelection, setRowSelection] = React.useState<Record<string, boolean>>({})
  const selectedClasses = React.useMemo(() => {
    return new Set(
      Object.keys(rowSelection).filter((key) => rowSelection[key]),
    )
  }, [rowSelection])

  const columns = useClassesColumns({
    onEdit: setClassToEdit,
    onDelete: setClassToDelete,
  })

  return (
    <div className="flex h-full flex-col bg-background">
      <ClassesHeader />
      <ClassesToolbar
        onExport={() =>
          handleExportCSV(
            classesQuery.data?.data || [],
            'classes_export.csv',
            [
              { header: 'ID', accessor: 'id' },
              { header: 'Name', accessor: 'section_name' },
              { header: 'Grade Level', accessor: 'grade_id' },
              { header: 'Academic Year', accessor: 'academic_year_id' },
            ],
          )
        }
      />
      <ClassesListContainer
        query={classesQuery}
        columns={columns}
        rowSelection={rowSelection}
        setRowSelection={setRowSelection}
      />

      <ClassAddDialog
        open={store.isCreateClassOpen}
        onOpenChange={setIsCreateClassOpen}
        onConfirm={(data: ClassFormValues) =>
          createClass.mutate({ body: data })
        }
        isSubmitting={createClass.isPending}
      />

      <ClassEditDialog
        classItem={store.classToEdit}
        open={!!store.classToEdit}
        onOpenChange={() => setClassToEdit(null)}
        onConfirm={(data: ClassFormValues) =>
          store.classToEdit &&
          updateClass.mutate({
            path: { id: store.classToEdit.id },
            body: data,
          })
        }
        isSubmitting={updateClass.isPending}
      />

      <AlertDialog
        open={!!store.classToDelete}
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
                store.classToDelete &&
                deleteClass.mutate({ path: { id: store.classToDelete } })
              }
            >
              Delete
            </AlertDialogAction>
          </AlertDialogFooter>
        </AlertDialogContent>
      </AlertDialog>

      <AlertDialog
        open={isBulkDeleteOpen}
        onOpenChange={setIsBulkDeleteOpen}
      >
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
                bulkDeleteClasses.mutate({
                  body: { class_ids: Array.from(selectedClasses) },
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
