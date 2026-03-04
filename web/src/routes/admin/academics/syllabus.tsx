import { Link, createFileRoute } from '@tanstack/react-router'
import { useQuery } from '@tanstack/react-query'
import * as React from 'react'
import { z } from 'zod'
import { HugeiconsIcon } from '@hugeicons/react'
import { PlusSignIcon } from '@hugeicons/core-free-icons'
import type { SyllabusResponse } from '@/lib/api/types.gen'
import {
  getSyllabusTopicsForStandardQueryOptions,
  useCreateSyllabusTopic,
  useDeleteSyllabusTopic,
  useUpdateSyllabusTopic,
} from '@/features/academics/syllabus/api'
import { SyllabusHeader } from '@/features/academics/syllabus/components/syllabus-header'
import { getSyllabusColumns } from '@/features/academics/syllabus/components/syllabus-table-columns'
import { SyllabusDialog } from '@/features/academics/syllabus/components/syllabus-dialog'
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
import { DataTable } from '@/components/data-table'
import { authClient } from '@/lib/clients'
import { getCurriculumStandardByIdOptions } from '@/lib/api/@tanstack/react-query.gen'
import { Button } from '@/components/ui/button'

const syllabusSearchSchema = z.object({
  standardId: z.string().optional(),
})

export const Route = createFileRoute('/admin/academics/syllabus')({
  validateSearch: syllabusSearchSchema,
  component: SyllabusPage,
})

function SyllabusPage() {
  const { standardId } = Route.useSearch()
  const [topicToEdit, setTopicToEdit] = React.useState<SyllabusResponse | null>(
    null,
  )
  const [topicToDelete, setTopicToDelete] = React.useState<string | null>(null)
  const [parentTopic, setParentTopic] = React.useState<SyllabusResponse | null>(
    null,
  )
  const [isAddOpen, setIsAddOpen] = React.useState(false)

  const syllabusQuery = useQuery({
    ...getSyllabusTopicsForStandardQueryOptions(standardId ?? ''),
    enabled: !!standardId,
  })

  const standardQuery = useQuery({
    ...getCurriculumStandardByIdOptions({
      client: authClient,
      path: { standard_id: standardId ?? '' },
    }),
    enabled: !!standardId,
  })

  const createMutation = useCreateSyllabusTopic()
  const updateMutation = useUpdateSyllabusTopic()
  const deleteMutation = useDeleteSyllabusTopic()

  const columns = getSyllabusColumns({
    onEdit: setTopicToEdit,
    onDelete: setTopicToDelete,
    onAddSubTopic: (parent) => {
      setParentTopic(parent)
      setIsAddOpen(true)
    },
  })

  // Basic tree processing for display - in a real app, use a proper tree component
  const topics = React.useMemo(() => {
    const data = syllabusQuery.data || []
    return data
  }, [syllabusQuery.data])

  if (!standardId) {
    return (
      <Stack gap={4} p={8} className="h-full items-center justify-center">
        <SyllabusHeader />
        <Box className="text-muted-foreground text-center">
          Please select a curriculum standard first to view its syllabus.
        </Box>
        <div>
          <Link to="/admin/academics/curriculum">
            <Button variant="outline">Go to Curriculum Standards</Button>
          </Link>
        </div>
      </Stack>
    )
  }

  return (
    <Stack gap={4} p={0} className="h-full">
      <SyllabusHeader standard={standardQuery.data} />

      <HStack className="justify-end">
        <Button
          onClick={() => {
            setParentTopic(null)
            setIsAddOpen(true)
          }}
          size="sm"
          className="gap-2"
        >
          <HugeiconsIcon icon={PlusSignIcon} className="size-4" />
          Add Root Topic
        </Button>
      </HStack>

      <Box className="flex-1 overflow-hidden border rounded-xl bg-card">
        <DataTable
          columns={columns}
          data={topics}
          isLoading={syllabusQuery.isLoading}
          searchPlaceholder="Filter topics..."
          pageIndex={0}
          pageSize={topics.length || 10}
          pageCount={1}
          canNextPage={false}
          canPreviousPage={false}
          fetchNextPage={() => {}}
          fetchPreviousPage={() => {}}
        />
      </Box>

      <SyllabusDialog
        open={isAddOpen}
        onOpenChange={(open) => {
          setIsAddOpen(open)
          if (!open) setParentTopic(null)
        }}
        onConfirm={(data) =>
          createMutation.mutate(
            { body: data },
            {
              onSuccess: () => setIsAddOpen(false),
            },
          )
        }
        isSubmitting={createMutation.isPending}
        parentTopic={parentTopic}
        standardId={standardId}
        title={
          parentTopic
            ? `Add Sub-topic to ${parentTopic.topic_name}`
            : 'Add Root Topic'
        }
      />

      <SyllabusDialog
        open={!!topicToEdit}
        onOpenChange={() => setTopicToEdit(null)}
        topic={topicToEdit}
        onConfirm={(data) =>
          topicToEdit &&
          updateMutation.mutate(
            {
              path: { syllabus_id: topicToEdit.id },
              body: data,
            },
            { onSuccess: () => setTopicToEdit(null) },
          )
        }
        isSubmitting={updateMutation.isPending}
        standardId={standardId}
        title="Edit Topic"
      />

      <AlertDialog
        open={!!topicToDelete}
        onOpenChange={() => setTopicToDelete(null)}
      >
        <AlertDialogContent>
          <AlertDialogHeader>
            <AlertDialogTitle>Are you absolutely sure?</AlertDialogTitle>
            <AlertDialogDescription>
              This action cannot be undone. This will permanently delete the
              topic and all its sub-topics.
            </AlertDialogDescription>
          </AlertDialogHeader>
          <AlertDialogFooter>
            <AlertDialogCancel>Cancel</AlertDialogCancel>
            <AlertDialogAction
              onClick={() =>
                topicToDelete &&
                deleteMutation.mutate(
                  {
                    path: { syllabus_id: topicToDelete },
                  },
                  { onSuccess: () => setTopicToDelete(null) },
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
