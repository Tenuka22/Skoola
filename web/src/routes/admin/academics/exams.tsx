import { createFileRoute } from '@tanstack/react-router'
import { useQuery } from '@tanstack/react-query'
import * as React from 'react'
import type { ExamResponse } from '@/lib/api/types.gen'
import type {
  ExamFormValues,
  UpdateExamFormValues,
} from '@/features/academics/exams/schemas'
import {
  getAllExamsQueryOptions,
  useCreateExam,
  useDeleteExam,
  useUpdateExam,
} from '@/features/academics/exams/api'
import { ExamsHeader } from '@/features/academics/exams/components/exams-header'
import { getExamsColumns } from '@/features/academics/exams/components/exams-table-columns'
import { Box, Stack } from '@/components/primitives'
import { DataTable } from '@/components/data-table'
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
import { ExamAddDialog } from '@/features/academics/exams/components/exam-add-dialog'
import { ExamEditDialog } from '@/features/academics/exams/components/exam-edit-dialog'
import { Empty } from '@/components/empty'

export const Route = createFileRoute('/admin/academics/exams')({
  component: ExamsPage,
})

function ExamsPage() {
  const [examToEdit, setExamToEdit] = React.useState<ExamResponse | null>(null)
  const [examToDelete, setExamToDelete] = React.useState<string | null>(null)
  const [isAddOpen, setIsAddOpen] = React.useState(false)
  const [search, setSearch] = React.useState('')

  const examsQuery = useQuery(getAllExamsQueryOptions())
  const createMutation = useCreateExam()
  const updateMutation = useUpdateExam()
  const deleteMutation = useDeleteExam()

  const columns = getExamsColumns({
    onEdit: setExamToEdit,
    onDelete: setExamToDelete,
  })

  const rawData = examsQuery.data?.data || []
  const filteredData = React.useMemo(() => {
    if (!search) return rawData
    const s = search.toLowerCase()
    return rawData.filter((exam) => exam.name.toLowerCase().includes(s))
  }, [rawData, search])

  return (
    <Stack gap={6} p={8} className="h-full overflow-hidden">
      <ExamsHeader />

      <Box className="flex-1 flex flex-col overflow-hidden min-h-0">
        <DataTable
          columns={columns}
          data={filteredData}
          isLoading={examsQuery.isLoading}
          search={search}
          onSearchChange={setSearch}
          searchPlaceholder="Filter exams..."
          onAdd={() => setIsAddOpen(true)}
          onAddLabel="Add Exam"
          pageIndex={0}
          pageSize={filteredData.length || 10}
          pageCount={1}
          canNextPage={false}
          canPreviousPage={false}
          fetchNextPage={() => {}}
          fetchPreviousPage={() => {}}
          emptyState={
            <Empty
              title="No Exams Found"
              description={
                search
                  ? 'No exams match your search criteria.'
                  : 'Get started by creating your first exam.'
              }
              icon="empty"
              className="py-12"
            />
          }
        />
      </Box>

      <ExamAddDialog
        open={isAddOpen}
        onOpenChange={setIsAddOpen}
        onConfirm={(data: ExamFormValues) =>
          createMutation.mutate(
            { body: data },
            {
              onSuccess: () => {
                setIsAddOpen(false)
              },
            },
          )
        }
        isSubmitting={createMutation.isPending}
      />

      <ExamEditDialog
        exam={examToEdit}
        open={!!examToEdit}
        onOpenChange={() => setExamToEdit(null)}
        onConfirm={(data: UpdateExamFormValues) =>
          examToEdit &&
          updateMutation.mutate(
            {
              path: { id: examToEdit.id },
              body: data,
            },
            {
              onSuccess: () => {
                setExamToEdit(null)
              },
            },
          )
        }
        isSubmitting={updateMutation.isPending}
      />

      <AlertDialog
        open={!!examToDelete}
        onOpenChange={() => setExamToDelete(null)}
      >
        <AlertDialogContent>
          <AlertDialogHeader>
            <AlertDialogTitle>Are you absolutely sure?</AlertDialogTitle>
            <AlertDialogDescription>
              This action cannot be undone. This will permanently delete the
              exam.
            </AlertDialogDescription>
          </AlertDialogHeader>
          <AlertDialogFooter>
            <AlertDialogCancel>Cancel</AlertDialogCancel>
            <AlertDialogAction
              onClick={() =>
                examToDelete &&
                deleteMutation.mutate(
                  { path: { id: examToDelete } },
                  { onSuccess: () => setExamToDelete(null) },
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
