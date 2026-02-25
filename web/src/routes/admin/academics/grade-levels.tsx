import { createFileRoute } from '@tanstack/react-router'
import {
  keepPreviousData,
  useMutation,
  useQuery,
  useQueryClient,
} from '@tanstack/react-query'
import * as React from 'react'
import { toast } from 'sonner'

import type { GradeLevelFormValues } from '@/features/academics/grade-levels/schemas'
import { authClient } from '@/lib/clients'
import { handleExportCSV } from '@/lib/export'
import {
  bulkDeleteGradeLevelsMutation,
  createGradeLevelMutation,
  deleteGradeLevelMutation,
  getAllGradeLevelsOptions,
  getAllGradeLevelsQueryKey,
  updateGradeLevelMutation,
} from '@/lib/api/@tanstack/react-query.gen'
import { useGradeLevelsStore } from '@/features/academics/grade-levels/store'
import { GradeLevelsHeader } from '@/features/academics/grade-levels/components/grade-levels-header'
import { GradeLevelsToolbar } from '@/features/academics/grade-levels/components/grade-levels-toolbar'
import { GradeLevelsListContainer } from '@/features/academics/grade-levels/components/grade-levels-list-container'
import { getGradeLevelsColumns } from '@/features/academics/grade-levels/components/grade-levels-table-columns'
import { GradeLevelAddDialog } from '@/features/academics/grade-levels/components/grade-level-add-dialog'
import { GradeLevelEditDialog } from '@/features/academics/grade-levels/components/grade-level-edit-dialog'
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

export const Route = createFileRoute('/admin/academics/grade-levels')({
  component: GradeLevelsPage,
})

function GradeLevelsPage() {
  const store = useGradeLevelsStore()
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
    setGradeLevelToEdit,
    setGradeLevelToDelete,
    setIsCreateGradeLevelOpen,
    isBulkDeleteOpen,
    setIsBulkDeleteOpen,
  } = store

  const sortBy = sorting[0]?.id
  const sortOrder = sorting[0]?.desc ? 'desc' : 'asc'

  const gradeLevelsQuery = useQuery({
    ...getAllGradeLevelsOptions({
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
      queryKey: getAllGradeLevelsQueryKey(),
    })
  }

  const createGradeLevel = useMutation({
    ...createGradeLevelMutation({
      client: authClient,
    }),
    onSuccess: () => {
      toast.success('Grade level created successfully.')
      invalidateQueries()
      setIsCreateGradeLevelOpen(false)
    },
    onError: (error) => {
      toast.error(
        `Failed to create grade level: ${error.message || 'Unknown error'}`,
      )
    },
  })

  const updateGradeLevel = useMutation({
    ...updateGradeLevelMutation({
      client: authClient,
    }),
    onSuccess: () => {
      toast.success('Grade level updated successfully.')
      invalidateQueries()
      setGradeLevelToEdit(null)
    },
    onError: (error) => {
      toast.error(
        `Failed to update grade level: ${error.message || 'Unknown error'}`,
      )
    },
  })

  const deleteGradeLevel = useMutation({
    ...deleteGradeLevelMutation({
      client: authClient,
    }),
    onSuccess: () => {
      toast.success('Grade level deleted successfully.')
      invalidateQueries()
      setGradeLevelToDelete(null)
    },
    onError: (error) => {
      toast.error(
        `Failed to delete grade level: ${error.message || 'Unknown error'}`,
      )
    },
  })

  const bulkDeleteGradeLevels = useMutation({
    ...bulkDeleteGradeLevelsMutation({
      client: authClient,
    }),
    onSuccess: (_, variables) => {
      const count = variables.body?.grade_level_ids?.length ?? 0
      toast.success(`Successfully deleted ${count} grade levels.`)
      invalidateQueries()
      setIsBulkDeleteOpen(false)
      setRowSelection({})
    },
    onError: (error) => {
      toast.error(
        `Failed to delete grade levels: ${error.message || 'Unknown error'}`,
      )
    },
  })

  const [rowSelection, setRowSelection] = React.useState<Record<string, boolean>>({})
  const selectedGradeLevels = React.useMemo(() => {
    return new Set(
      Object.keys(rowSelection).filter((key) => rowSelection[key]),
    )
  }, [rowSelection])

  const columns = getGradeLevelsColumns({
    onEdit: setGradeLevelToEdit,
    onDelete: setGradeLevelToDelete,
  })

  return (
    <div className="flex h-full flex-col bg-background">
      <GradeLevelsHeader />
      <GradeLevelsToolbar
        onExport={() =>
          handleExportCSV(
            gradeLevelsQuery.data?.data || [],
            'grade_levels_export.csv',
            [
              { header: 'ID', accessor: 'id' },
              { header: 'Name', accessor: 'grade_name' },
              { header: 'Number', accessor: 'grade_number' },
              { header: 'Education Level', accessor: 'education_level' },
            ],
          )
        }
      />
      <GradeLevelsListContainer
        query={gradeLevelsQuery as any}
        columns={columns}
        rowSelection={rowSelection}
        setRowSelection={setRowSelection}
      />

      <GradeLevelAddDialog
        open={store.isCreateGradeLevelOpen}
        onOpenChange={setIsCreateGradeLevelOpen}
        onConfirm={(data: GradeLevelFormValues) =>
          createGradeLevel.mutate({ body: data as any })
        }
        isSubmitting={createGradeLevel.isPending}
      />

      <GradeLevelEditDialog
        gradeLevel={store.gradeLevelToEdit}
        open={!!store.gradeLevelToEdit}
        onOpenChange={() => setGradeLevelToEdit(null)}
        onConfirm={(data: GradeLevelFormValues) =>
          store.gradeLevelToEdit &&
          updateGradeLevel.mutate({
            path: { id: store.gradeLevelToEdit.id },
            body: data as any,
          })
        }
        isSubmitting={updateGradeLevel.isPending}
      />

      <AlertDialog
        open={!!store.gradeLevelToDelete}
        onOpenChange={() => setGradeLevelToDelete(null)}
      >
        <AlertDialogContent>
          <AlertDialogHeader>
            <AlertDialogTitle>Are you absolutely sure?</AlertDialogTitle>
            <AlertDialogDescription>
              This action cannot be undone. This will permanently delete the
              grade level.
            </AlertDialogDescription>
          </AlertDialogHeader>
          <AlertDialogFooter>
            <AlertDialogCancel>Cancel</AlertDialogCancel>
            <AlertDialogAction
              onClick={() =>
                store.gradeLevelToDelete &&
                deleteGradeLevel.mutate({ path: { id: store.gradeLevelToDelete } })
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
              {selectedGradeLevels.size} grade levels.
            </AlertDialogDescription>
          </AlertDialogHeader>
          <AlertDialogFooter>
            <AlertDialogCancel>Cancel</AlertDialogCancel>
            <AlertDialogAction
              onClick={() => {
                bulkDeleteGradeLevels.mutate({
                  body: { grade_level_ids: Array.from(selectedGradeLevels) },
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
