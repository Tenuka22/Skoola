import { createFileRoute } from '@tanstack/react-router'
import { useQuery } from '@tanstack/react-query'

import { z } from 'zod'
import { getLessonProgressQueryOptions } from '@/features/academics/lesson-progress/api'
import { LessonProgressHeader } from '@/features/academics/lesson-progress/components/lesson-progress-header'
import { useLessonProgressColumns } from '@/features/academics/lesson-progress/components/lesson-progress-table-columns'
import { Box, HStack, Stack } from '@/components/primitives'
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

  const classesQuery = useQuery(getAllClassesOptions({ client: authClient }))
  const subjectsQuery = useQuery(getAllSubjectsOptions({ client: authClient }))

  const lessonProgressQuery = useQuery({
    ...getLessonProgressQueryOptions(classId ?? '', subjectId ?? ''),
    enabled: !!classId && !!subjectId,
  })

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

  return (
    <Stack gap={4} p={0} className="h-full">
      <LessonProgressHeader />

      <HStack gap={4}>
        <div className="w-64">
          <Select
            value={classId}
            onValueChange={(val: string | null | undefined) =>
              handleClassChange(val)
            }
          >
            <SelectTrigger>
              <SelectValue placeholder="Select Class" />
            </SelectTrigger>
            <SelectContent>
              {classesQuery.data?.data.map((c) => (
                <SelectItem key={c.id} value={c.id}>
                  {c.section_name} ({c.id})
                </SelectItem>
              ))}
            </SelectContent>
          </Select>
        </div>
        <div className="w-64">
          <Select
            value={subjectId}
            onValueChange={(val: string | null | undefined) =>
              handleSubjectChange(val)
            }
          >
            <SelectTrigger>
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
        </div>
      </HStack>

      <Box className="flex-1 overflow-hidden border rounded-xl bg-card">
        {classId && subjectId ? (
          <DataTable
            columns={columns}
            data={lessonProgressQuery.data || []}
            isLoading={lessonProgressQuery.isLoading}
            searchPlaceholder="Filter entries..."
            pageIndex={0}
            pageSize={lessonProgressQuery.data?.length || 10}
            pageCount={1}
            canNextPage={false}
            canPreviousPage={false}
            fetchNextPage={() => {}}
            fetchPreviousPage={() => {}}
          />
        ) : (
          <Box className="h-full flex items-center justify-center text-muted-foreground">
            Please select a class and subject to view lesson progress.
          </Box>
        )}
      </Box>
    </Stack>
  )
}
