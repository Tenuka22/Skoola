import { createFileRoute } from '@tanstack/react-router'
import { useQuery } from '@tanstack/react-query'
import * as React from 'react'
import { z } from 'zod'
import {
  getLessonProgressQueryOptions,
  useRecordLessonProgress,
} from '@/features/academics/lesson-progress/api'
import { LessonProgressHeader } from '@/features/academics/lesson-progress/components/lesson-progress-header'
import { useLessonProgressColumns } from '@/features/academics/lesson-progress/components/lesson-progress-table-columns'
import { LessonProgressDialog } from '@/features/academics/lesson-progress/components/lesson-progress-dialog'
import { Box, HStack, Stack, Text } from '@/components/primitives'
import { DataTable } from '@/components/data-table'
import { authClient } from '@/lib/clients'
import {
  getAllClassesOptions,
  getAllSubjectsOptions,
} from '@/lib/api/@tanstack/react-query.gen'
import {
  Select,
  SelectContent,
  SelectItem,
  SelectTrigger,
  SelectValue,
} from '@/components/ui/select'
import { Empty } from '@/components/empty'

const lessonProgressSearchSchema = z.object({
  classId: z.string().optional(),
  subjectId: z.string().optional(),
})

export const Route = createFileRoute('/admin/academics/lesson-progress')({
  validateSearch: lessonProgressSearchSchema,
  component: LessonProgressPage,
})

function LessonProgressPage() {
  const { classId, subjectId } = Route.useSearch()
  const navigate = Route.useNavigate()
  const [isAddOpen, setIsAddOpen] = React.useState(false)

  const classesQuery = useQuery(getAllClassesOptions({ client: authClient }))
  const subjectsQuery = useQuery(getAllSubjectsOptions({ client: authClient }))

  const lessonProgressQuery = useQuery({
    ...getLessonProgressQueryOptions(classId ?? '', subjectId ?? ''),
    enabled: !!classId && !!subjectId,
  })

  const recordMutation = useRecordLessonProgress()
  const columns = useLessonProgressColumns()

  const handleClassChange = (val: string | null | undefined) => {
    navigate({
      search: (prev) => ({ ...prev, classId: val === null ? undefined : val }),
    })
  }

  const handleSubjectChange = (val: string | null | undefined) => {
    navigate({
      search: (prev) => ({
        ...prev,
        subjectId: val === null ? undefined : val,
      }),
    })
  }

  const lessonProgressData = lessonProgressQuery.data || []

  return (
    <Stack gap={6} p={8} className="h-full overflow-hidden">
      <LessonProgressHeader />

      <Box bg="card" p={4} rounded="xl" className="border border-border/40 shadow-sm">
        <HStack gap={6} align="center">
          <HStack gap={3} align="center">
            <Text size="sm" className="font-semibold text-foreground/80 whitespace-nowrap">
              Class:
            </Text>
            <Box className="w-64">
              <Select
                value={classId || ''}
                onValueChange={(val) => handleClassChange(val || null)}
              >
                <SelectTrigger className="h-9">
                  <SelectValue placeholder="Select Class" />
                </SelectTrigger>
                <SelectContent>
                  {classesQuery.data?.data.map((c) => (
                    <SelectItem key={c.id} value={c.id}>
                      {c.section_name}
                    </SelectItem>
                  ))}
                </SelectContent>
              </Select>
            </Box>
          </HStack>

          <HStack gap={3} align="center">
            <Text size="sm" className="font-semibold text-foreground/80 whitespace-nowrap">
              Subject:
            </Text>
            <Box className="w-64">
              <Select
                value={subjectId || ''}
                onValueChange={(val) => handleSubjectChange(val || null)}
              >
                <SelectTrigger className="h-9">
                  <SelectValue placeholder="Select Subject" />
                </SelectTrigger>
                <SelectContent>
                  {subjectsQuery.data?.data.map((s) => (
                    <SelectItem key={s.id} value={s.id}>
                      {s.subject_name_en}
                    </SelectItem>
                  ))}
                </SelectContent>
              </Select>
            </Box>
          </HStack>
        </HStack>
      </Box>

      <Box className="flex-1 flex flex-col overflow-hidden min-h-0">
        {!classId || !subjectId ? (
          <Empty
            title="Selection Required"
            description="Please select both a class and a subject to view and record lesson progress."
            icon="empty"
            className="flex-1"
          />
        ) : (
          <Box className="flex-1 overflow-hidden relative min-h-0">
            <DataTable
              columns={columns}
              data={lessonProgressData}
              isLoading={lessonProgressQuery.isLoading}
              searchPlaceholder="Search topics..."
              onAdd={() => setIsAddOpen(true)}
              onAddLabel="Record Progress"
              pageIndex={0}
              pageSize={lessonProgressData.length || 10}
              pageCount={1}
              canNextPage={false}
              canPreviousPage={false}
              fetchNextPage={() => {}}
              fetchPreviousPage={() => {}}
              emptyState={
                <Empty
                  title="No Records Found"
                  description="No lesson progress has been recorded for this class and subject yet."
                  icon="empty"
                  className="py-12"
                />
              }
            />
          </Box>
        )}
      </Box>

      {classId && subjectId && (
        <LessonProgressDialog
          open={isAddOpen}
          onOpenChange={setIsAddOpen}
          classId={classId}
          subjectId={subjectId}
          onConfirm={(data) => {
            recordMutation.mutate(
              { body: data },
              { onSuccess: () => setIsAddOpen(false) }
            )
          }}
          isSubmitting={recordMutation.isPending}
        />
      )}
    </Stack>
  )
}
