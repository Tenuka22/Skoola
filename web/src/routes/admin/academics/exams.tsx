import { createFileRoute } from '@tanstack/react-router'
import { useQuery } from '@tanstack/react-query'
import * as React from 'react'
import { HugeiconsIcon } from '@hugeicons/react'
import { PlusSignIcon } from '@hugeicons/core-free-icons'
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
import { Box, HStack, Stack } from '@/components/primitives'
import { DataTable } from '@/components/data-table'
import { Button } from '@/components/ui/button'
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

export const Route = createFileRoute('/admin/academics/exams')({
  component: ExamsPage,
})

function ExamsPage() {
  const [examToEdit, setExamToEdit] = React.useState<ExamResponse | null>(null)
  const [examToDelete, setExamToDelete] = React.useState<string | null>(null)
  const [isAddOpen, setIsAddOpen] = React.useState(false)

  const examsQuery = useQuery(getAllExamsQueryOptions())
  const createMutation = useCreateExam()
  const updateMutation = useUpdateExam()
  const deleteMutation = useDeleteExam()

  const columns = getExamsColumns({
    onEdit: setExamToEdit,
    onDelete: setExamToDelete,
  })

  return (
    <Stack gap={4} p={0} className="h-full">
      <ExamsHeader />

      <HStack className="justify-end">
        <Button onClick={() => setIsAddOpen(true)} size="sm" className="gap-2">
          <HugeiconsIcon icon={PlusSignIcon} className="size-4" />
          Add Exam
        </Button>
      </HStack>

      <Box className="flex-1 overflow-hidden border rounded-xl bg-card">
        <DataTable
          columns={columns}
          data={examsQuery.data?.data || []}
          isLoading={examsQuery.isLoading}
          searchPlaceholder="Filter exams..."
          pageIndex={0}
          pageSize={examsQuery.data?.data.length || 10}
          pageCount={1}
          canNextPage={false}
          canPreviousPage={false}
          fetchNextPage={() => {}}
          fetchPreviousPage={() => {}}
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
