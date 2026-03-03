import { createFileRoute } from '@tanstack/react-router'
import { keepPreviousData, useQuery } from '@tanstack/react-query'
import * as React from 'react'

import type { GradeLevelFormValues } from '@/features/academics/grade-levels/schemas'
import type { GradeLevelResponse } from '@/lib/api/types.gen'
import { handleExportCSV } from '@/lib/export'
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
import { Stack } from '@/components/primitives'
import {
  getAllGradeLevelsQueryOptions,
  useBulkDeleteGradeLevels,
  useCreateGradeLevel,
  useDeleteGradeLevel,
  useUpdateGradeLevel,
} from '@/features/academics/grade-levels/api'
import { useGradeLevelsSearchParams } from '@/features/academics/grade-levels/search-params'

export const Route = createFileRoute('/admin/grades')({
  component: GradeLevelsPage,
})

function GradeLevelsPage() {
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
  const selectedGradeLevels = React.useMemo(() => {
    return new Set(Object.keys(rowSelection).filter((key) => rowSelection[key]))
  }, [rowSelection])

  const columns = getGradeLevelsColumns({
    onEdit: setGradeLevelToEdit,
    onDelete: setGradeLevelToDelete,
  })

  return (
    <Stack gap={0} className="h-full bg-background">
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
        setIsCreateGradeLevelOpen={setIsCreateGradeLevelOpen}
      />
      <GradeLevelsListContainer
        query={gradeLevelsQuery}
        columns={columns}
        rowSelection={rowSelection}
        setRowSelection={setRowSelection}
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
              {selectedGradeLevels.size} grade levels.
            </AlertDialogDescription>
          </AlertDialogHeader>
          <AlertDialogFooter>
            <AlertDialogCancel>Cancel</AlertDialogCancel>
            <AlertDialogAction
              onClick={() => {
                bulkDeleteGradeLevels.mutate(
                  {
                    body: { grade_level_ids: Array.from(selectedGradeLevels) },
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
