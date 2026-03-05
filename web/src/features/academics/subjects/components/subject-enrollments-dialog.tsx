import { HugeiconsIcon } from '@hugeicons/react'
import { AlertCircleIcon } from '@hugeicons/core-free-icons'
import { useQuery } from '@tanstack/react-query'
import * as React from 'react'
import type {
  AcademicYearResponse,
  StudentResponse,
  SubjectResponse,
} from '@/lib/api/types.gen'
import type { DataTableColumnDef } from '@/components/data-table'
import { Button } from '@/components/ui/button'
import {
  Dialog,
  DialogContent,
  DialogDescription,
  DialogHeader,
  DialogTitle,
} from '@/components/ui/dialog'
import { authClient } from '@/lib/clients'
import {
  getAllAcademicYearsOptions,
  getStudentsBySubjectOptions,
} from '@/lib/api/@tanstack/react-query.gen'
import { Badge } from '@/components/ui/badge'
import {
  Select,
  SelectContent,
  SelectItem,
  SelectTrigger,
  SelectValue,
} from '@/components/ui/select'
import { Label } from '@/components/ui/label'
import { HStack, Stack, Text } from '@/components/primitives'
import { Avatar, AvatarFallback, AvatarImage } from '@/components/ui/avatar'
import { DataTable } from '@/components/data-table'

interface SubjectEnrollmentsDialogProps {
  subject: SubjectResponse | null
  open: boolean
  onOpenChange: (open: boolean) => void
  onEnrollStudent?: () => void
}

export function SubjectEnrollmentsDialog({
  subject,
  open,
  onOpenChange,
  onEnrollStudent,
}: SubjectEnrollmentsDialogProps) {
  const [selectedAcademicYearId, setSelectedAcademicYearId] = React.useState<
    string | undefined
  >(undefined)
  const [search, setSearch] = React.useState('')
  const [page, setPage] = React.useState(1)

  const { data: academicYearsData } = useQuery(
    getAllAcademicYearsOptions({ client: authClient }),
  )
  const academicYears = React.useMemo(
    () => academicYearsData?.data || [],
    [academicYearsData?.data],
  )

  React.useEffect(() => {
    if (academicYears.length > 0 && !selectedAcademicYearId) {
      const currentYear = academicYears.find(
        (ay: AcademicYearResponse) => ay.current,
      )
      setSelectedAcademicYearId(currentYear?.id || academicYears[0]?.id)
    }
  }, [academicYears, selectedAcademicYearId])

  const studentsQuery = useQuery({
    ...getStudentsBySubjectOptions({
      client: authClient,
      path: {
        id: subject?.id || '',
        academic_year_id: selectedAcademicYearId || '',
      },
      query: {
        limit: 50,
        page: page,
      },
    }),
    enabled: open && !!subject?.id && !!selectedAcademicYearId,
  })

  // Reset page when search or year changes
  React.useEffect(() => {
    setPage(1)
  }, [search, selectedAcademicYearId])

  const totalPages = studentsQuery.data?.total_pages || 1

  // Filter students locally if search is provided
  const filteredEnrollments = React.useMemo(() => {
    const enrollments = studentsQuery.data?.data || []
    if (!search) return enrollments
    const lowerSearch = search.toLowerCase()
    return enrollments.filter(
      (e: StudentResponse) =>
        e.name_english.toLowerCase().includes(lowerSearch) ||
        e.admission_number.toLowerCase().includes(lowerSearch),
    )
  }, [studentsQuery.data?.data, search])

  const columns = React.useMemo(
    (): Array<DataTableColumnDef<StudentResponse>> => [
      {
        accessorKey: 'name_english',
        header: 'Student',
        cell: ({ row }) => {
          const student = row.original
          return (
            <HStack gap={3}>
              <Avatar className="size-8">
                <AvatarImage src={student.profile_photo_url || ''} />
                <AvatarFallback>
                  {student.name_english[0].toUpperCase()}
                </AvatarFallback>
              </Avatar>
              <Stack gap={0}>
                <Text size="sm" className="font-medium">
                  {student.name_english}
                </Text>
                <Text size="xs" muted>
                  {student.admission_number}
                </Text>
              </Stack>
            </HStack>
          )
        },
      },
      {
        id: 'status',
        header: 'Status',
        cell: () => (
          <Badge variant="secondary" className="text-[10px]">
            Enrolled
          </Badge>
        ),
      },
    ],
    [],
  )

  return (
    <Dialog open={open} onOpenChange={onOpenChange}>
      <DialogContent className="max-w-4xl flex flex-col h-[80vh]">
        <DialogHeader>
          <DialogTitle>Enrollments for {subject?.subject_name_en}</DialogTitle>
          <DialogDescription>
            View students enrolled in this subject.
          </DialogDescription>
        </DialogHeader>

        <Stack gap={4} className="flex-1 overflow-hidden py-4">
          <HStack gap={3} align="end">
            <Stack gap={1} className="w-64">
              <Label htmlFor="academic_year_select">Academic Year</Label>
              <Select
                onValueChange={(value) =>
                  setSelectedAcademicYearId(value || undefined)
                }
                value={selectedAcademicYearId}
              >
                <SelectTrigger id="academic_year_select">
                  <SelectValue placeholder="Select Year" />
                </SelectTrigger>
                <SelectContent>
                  {academicYears.map((ay: AcademicYearResponse) => (
                    <SelectItem key={ay.id} value={ay.id}>
                      {ay.name}
                    </SelectItem>
                  ))}
                </SelectContent>
              </Select>
            </Stack>
          </HStack>

          <div className="flex-1 overflow-hidden">
            {studentsQuery.isError ? (
              <div className="flex flex-col items-center justify-center py-20 text-center px-4">
                <HugeiconsIcon
                  icon={AlertCircleIcon}
                  className="size-10 text-destructive mb-2"
                />
                <Text size="sm" muted>
                  Failed to load enrollments.
                </Text>
              </div>
            ) : (
              <DataTable
                columns={columns}
                data={filteredEnrollments}
                isLoading={studentsQuery.isLoading}
                pageCount={totalPages}
                pageIndex={page - 1}
                pageSize={50}
                onPageIndexChange={(idx) => setPage(idx + 1)}
                canNextPage={page < totalPages}
                canPreviousPage={page > 1}
                fetchNextPage={() => setPage((p) => p + 1)}
                fetchPreviousPage={() => setPage((p) => p - 1)}
                search={search}
                onSearchChange={setSearch}
                onAdd={onEnrollStudent}
                onAddLabel="Add Student"
                searchPlaceholder="Search students..."
                showDefaultToolbar={true}
              />
            )}
          </div>
        </Stack>

        <HStack justify="end">
          <Button onClick={() => onOpenChange(false)} variant="outline">
            Close
          </Button>
        </HStack>
      </DialogContent>
    </Dialog>
  )
}
