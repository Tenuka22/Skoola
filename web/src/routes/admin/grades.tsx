import { createFileRoute } from '@tanstack/react-router'
import {
  keepPreviousData,
  useQuery,
  useQueryClient,
} from '@tanstack/react-query'
import * as React from 'react'

import { HugeiconsIcon } from '@hugeicons/react'
import { Delete02Icon } from '@hugeicons/core-free-icons'
import type { GradeLevelFormValues } from '@/features/academics/grade-levels/schemas'
import type { GradeLevelResponse } from '@/lib/api/types.gen'
import { GradeLevelsHeader } from '@/features/academics/grade-levels/components/grade-levels-header'
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
import { Stack } from '@/components/primitives'
import {
  getAllGradeLevelsQueryOptions,
  useBulkDeleteGradeLevels,
  useCreateGradeLevel,
  useDeleteGradeLevel,
  useUpdateGradeLevel,
} from '@/features/academics/grade-levels/api'
import { useGradeLevelsSearchParams } from '@/features/academics/grade-levels/search-params'
import { Button } from '@/components/ui/button'

export const Route = createFileRoute('/admin/grades')({
  component: GradeLevelsPage,
})

function GradeLevelsPage() {
  const queryClient = useQueryClient()
  const { page, limit, search, sortBy, sortOrder } =
    useGradeLevelsSearchParams()

  const [gradeLevelToDelete, setGradeLevelToDelete] = React.useState<
    string | null
  >(null)
  const [isBulkDeleteOpen, setIsBulkDeleteOpen] = React.useState(false)
  const [isCreateGradeLevelOpen, setIsCreateGradeLevelOpen] =
    React.useState(false)
  const [gradeLevelToEdit, setGradeLevelToEdit] =
    React.useState<GradeLevelResponse | null>(null)

  const gradeLevelsQuery = useQuery({
    ...getAllGradeLevelsQueryOptions({
      query: {
        page: page ?? 1,
        limit: limit ?? 10,
        search: search ?? undefined,
        sort_by: sortBy ?? 'grade_number',
        sort_order:
          sortOrder === 'asc' || sortOrder === 'desc' ? sortOrder : 'asc',
      },
    }),
    placeholderData: keepPreviousData,
  })

  const createGradeLevel = useCreateGradeLevel()
  const updateGradeLevel = useUpdateGradeLevel()
  const deleteGradeLevel = useDeleteGradeLevel()
  const bulkDeleteGradeLevels = useBulkDeleteGradeLevels()

  const [rowSelection, setRowSelection] = React.useState<
    Record<string, boolean>
  >({})

  const fetchFullData = React.useCallback(async () => {
    const options = getAllGradeLevelsQueryOptions({
      query: {
        page: 1,
        limit: 1000,
        search: search ?? undefined,
        sort_by: sortBy ?? 'grade_number',
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

  const columns = getGradeLevelsColumns({
    onEdit: setGradeLevelToEdit,
    onDelete: setGradeLevelToDelete,
  })

  return (
    <Stack gap={4} p={8} className="h-full bg-background">
      <GradeLevelsHeader />
      <GradeLevelsListContainer
        query={gradeLevelsQuery}
        columns={columns}
        rowSelection={rowSelection}
        setRowSelection={setRowSelection}
        onFetchFullData={fetchFullData}
        onAdd={() => setIsCreateGradeLevelOpen(true)}
        onAddLabel="Add Grade Level"
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

      <GradeLevelAddDialog
        open={isCreateGradeLevelOpen}
        onOpenChange={setIsCreateGradeLevelOpen}
        onConfirm={(data: GradeLevelFormValues) =>
          createGradeLevel.mutate(
            { body: data },
            {
              onSuccess: () => {
                setIsCreateGradeLevelOpen(false)
              },
            },
          )
        }
        isSubmitting={createGradeLevel.isPending}
      />

      <GradeLevelEditDialog
        gradeLevel={gradeLevelToEdit}
        open={!!gradeLevelToEdit}
        onOpenChange={() => setGradeLevelToEdit(null)}
        onConfirm={(data: GradeLevelFormValues) =>
          gradeLevelToEdit &&
          updateGradeLevel.mutate(
            {
              path: { id: gradeLevelToEdit.id },
              body: data,
            },
            {
              onSuccess: () => {
                setGradeLevelToEdit(null)
              },
            },
          )
        }
        isSubmitting={updateGradeLevel.isPending}
      />

      <AlertDialog
        open={!!gradeLevelToDelete}
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
                gradeLevelToDelete &&
                deleteGradeLevel.mutate(
                  {
                    path: { id: gradeLevelToDelete },
                  },
                  {
                    onSuccess: () => {
                      setGradeLevelToDelete(null)
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
              grade levels.
            </AlertDialogDescription>
          </AlertDialogHeader>
          <AlertDialogFooter>
            <AlertDialogCancel>Cancel</AlertDialogCancel>
            <AlertDialogAction
              onClick={() => {
                const ids = Object.keys(rowSelection).filter(
                  (k) => rowSelection[k],
                )
                bulkDeleteGradeLevels.mutate(
                  {
                    body: { grade_level_ids: ids },
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
