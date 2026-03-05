import { createFileRoute } from '@tanstack/react-router'
import { useQuery } from '@tanstack/react-query'
import * as React from 'react'
import { z } from 'zod'
import { HugeiconsIcon } from '@hugeicons/react'
import {
  LayoutGridIcon,
  TableIcon,
  Calendar02Icon,
} from '@hugeicons/core-free-icons'

import type { GradePeriodResponse } from '@/lib/api/types.gen'
import {
  getGradePeriodsByGradeQueryOptions,
  useCreateGradePeriod,
  useDeleteGradePeriod,
  useUpdateGradePeriod,
} from '@/features/academics/grade-periods/api'
import { GradePeriodsHeader } from '@/features/academics/grade-periods/components/grade-periods-header'
import { getGradePeriodsColumns } from '@/features/academics/grade-periods/components/grade-periods-table-columns'
import { GradePeriodDialog } from '@/features/academics/grade-periods/components/grade-period-dialog'
import { GradePeriodsVisualView } from '@/features/academics/grade-periods/components/grade-periods-visual-view'
import { GradePeriodsGridView } from '@/features/academics/grade-periods/components/grade-periods-grid-view'
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
import { Box, HStack, Stack, Text } from '@/components/primitives'
import { Tabs, TabsContent, TabsList, TabsTrigger } from '@/components/ui/tabs'
import { DataTable } from '@/components/data-table'
import { authClient } from '@/lib/clients'
import { getAllGradeLevelsOptions } from '@/lib/api/@tanstack/react-query.gen'
import {
  Select,
  SelectContent,
  SelectItem,
  SelectTrigger,
  SelectValue,
} from '@/components/ui/select'
import { Empty } from '@/components/empty'

const gradePeriodsSearchSchema = z.object({
  gradeId: z.string().optional(),
  view: z.enum(['table', 'grid', 'visual']).default('table'),
})

export const Route = createFileRoute('/admin/academics/grade-periods')({
  validateSearch: gradePeriodsSearchSchema,
  component: GradePeriodsPage,
})

function GradePeriodsPage() {
  const { gradeId, view } = Route.useSearch()
  const navigate = Route.useNavigate()

  const [periodToEdit, setPeriodToEdit] =
    React.useState<GradePeriodResponse | null>(null)
  const [periodToDelete, setPeriodToDelete] =
    React.useState<GradePeriodResponse | null>(null)
  const [isAddOpen, setIsAddOpen] = React.useState(false)
  const [search, setSearch] = React.useState('')

  const gradeLevelsQuery = useQuery(
    getAllGradeLevelsOptions({ client: authClient }),
  )

  const gradePeriodsQuery = useQuery({
    ...getGradePeriodsByGradeQueryOptions(gradeId ?? ''),
    enabled: !!gradeId,
  })

  const createMutation = useCreateGradePeriod()
  const updateMutation = useUpdateGradePeriod(gradeId ?? '')
  const deleteMutation = useDeleteGradePeriod(gradeId ?? '')

  const columns = getGradePeriodsColumns({
    onEdit: setPeriodToEdit,
    onDelete: setPeriodToDelete,
  })

  const selectedGrade = React.useMemo(() => {
    return gradeLevelsQuery.data?.data.find((g) => g.id === gradeId)
  }, [gradeLevelsQuery.data, gradeId])

  const handleGradeChange = (val: string | null) => {
    navigate({
      search: (prev) => ({ ...prev, gradeId: val ?? undefined }),
    })
  }

  const handleViewChange = (val: string) => {
    navigate({
      search: (prev) => ({
        ...prev,
        view: val as 'table' | 'grid' | 'visual',
      }),
    })
  }

  const rawData = Array.isArray(gradePeriodsQuery.data) ? gradePeriodsQuery.data : []
  
  const filteredData = React.useMemo(() => {
    if (!search) return rawData
    const s = search.toLowerCase()
    return rawData.filter(p => 
      p.period_number.toString().includes(s) || 
      p.start_time.toLowerCase().includes(s) || 
      p.end_time.toLowerCase().includes(s)
    )
  }, [rawData, search])

  return (
    <Stack gap={6} p={8} className="h-full overflow-hidden">
      <GradePeriodsHeader
        gradeName={selectedGrade?.grade_name}
        total={rawData.length}
      />

      <Box bg="card" p={4} rounded="xl" className="border border-border/40 shadow-sm">
        <HStack gap={4} align="center">
          <Text size="sm" className="font-semibold text-foreground/80 whitespace-nowrap">
            Current Grade Level:
          </Text>
          <Box className="w-72">
            <Select value={gradeId || ''} onValueChange={handleGradeChange}>
              <SelectTrigger className="h-9">
                <SelectValue placeholder="Select a grade level..." />
              </SelectTrigger>
              <SelectContent>
                {gradeLevelsQuery.data?.data.map((g) => (
                  <SelectItem key={g.id} value={g.id}>
                    {g.grade_name}
                  </SelectItem>
                ))}
              </SelectContent>
            </Select>
          </Box>
        </HStack>
      </Box>

      <Box className="flex-1 flex flex-col overflow-hidden min-h-0">
        {!gradeId ? (
          <Empty
            title="No Grade Selected"
            description="Please select a grade level from the dropdown above to manage its periods."
            icon="empty"
            className="flex-1"
          />
        ) : (
          <Tabs
            value={view ?? 'table'}
            onValueChange={handleViewChange}
            className="flex flex-col flex-1 gap-4 overflow-hidden min-h-0"
          >
            <HStack justify="between" align="center">
              <TabsList className="h-9 p-1 bg-muted/50 border border-border/40">
                <TabsTrigger value="table" className="gap-2 h-7 px-3 text-xs data-[state=active]:shadow-sm">
                  <HugeiconsIcon icon={TableIcon} className="size-3.5" />
                  Table
                </TabsTrigger>
                <TabsTrigger value="grid" className="gap-2 h-7 px-3 text-xs data-[state=active]:shadow-sm">
                  <HugeiconsIcon icon={LayoutGridIcon} className="size-3.5" />
                  Grid
                </TabsTrigger>
                <TabsTrigger value="visual" className="gap-2 h-7 px-3 text-xs data-[state=active]:shadow-sm">
                  <HugeiconsIcon icon={Calendar02Icon} className="size-3.5" />
                  Timeline
                </TabsTrigger>
              </TabsList>
            </HStack>

            <Box className="flex-1 overflow-hidden relative min-h-0">
              <TabsContent
                value="table"
                className="absolute inset-0 m-0 overflow-hidden flex flex-col data-[state=inactive]:hidden"
              >
                <DataTable
                  columns={columns}
                  data={filteredData}
                  isLoading={gradePeriodsQuery.isLoading}
                  search={search}
                  onSearchChange={setSearch}
                  searchPlaceholder="Filter periods..."
                  onAdd={() => setIsAddOpen(true)}
                  onAddLabel="Add Period"
                  pageIndex={0}
                  pageSize={filteredData.length || 10}
                  pageCount={1}
                  canNextPage={false}
                  canPreviousPage={false}
                  fetchNextPage={() => {}}
                  fetchPreviousPage={() => {}}
                  emptyState={
                    <Empty
                      title="No Periods Found"
                      description={search ? "Adjust your filter to see more results." : "No grade periods have been defined for this grade level yet."}
                      icon="empty"
                      className="py-12"
                    />
                  }
                />
              </TabsContent>

              <TabsContent
                value="grid"
                className="absolute inset-0 m-0 overflow-y-auto data-[state=inactive]:hidden"
              >
                <Stack gap={4}>
                  <GradePeriodsGridView
                    periods={filteredData}
                    onEdit={setPeriodToEdit}
                    onDelete={setPeriodToDelete}
                    isLoading={gradePeriodsQuery.isLoading}
                  />
                </Stack>
              </TabsContent>

              <TabsContent
                value="visual"
                className="absolute inset-0 m-0 overflow-y-auto data-[state=inactive]:hidden"
              >
                <GradePeriodsVisualView periods={rawData} />
              </TabsContent>
            </Box>
          </Tabs>
        )}
      </Box>

      {gradeId && (
        <GradePeriodDialog
          open={isAddOpen}
          onOpenChange={setIsAddOpen}
          gradeId={gradeId}
          title="Add Grade Period"
          onConfirm={(data) => {
            createMutation.mutate(
              { body: data },
              { onSuccess: () => setIsAddOpen(false) },
            )
          }}
          isSubmitting={createMutation.isPending}
        />
      )}

      {gradeId && (
        <GradePeriodDialog
          open={!!periodToEdit}
          onOpenChange={() => setPeriodToEdit(null)}
          gradeId={gradeId}
          period={periodToEdit}
          title="Edit Grade Period"
          onConfirm={(data) => {
            if (periodToEdit) {
              updateMutation.mutate(
                {
                  path: { id: periodToEdit.id },
                  body: data,
                },
                { onSuccess: () => setPeriodToEdit(null) },
              )
            }
          }}
          isSubmitting={updateMutation.isPending}
        />
      )}

      <AlertDialog
        open={!!periodToDelete}
        onOpenChange={() => setPeriodToDelete(null)}
      >
        <AlertDialogContent>
          <AlertDialogHeader>
            <AlertDialogTitle>Are you absolutely sure?</AlertDialogTitle>
            <AlertDialogDescription>
              This action cannot be undone. This will permanently delete the
              grade period. Timetable entries referencing this period will have
              their link removed but will remain in the system.
            </AlertDialogDescription>
          </AlertDialogHeader>
          <AlertDialogFooter>
            <AlertDialogCancel>Cancel</AlertDialogCancel>
            <AlertDialogAction
              className="bg-destructive text-destructive-foreground hover:bg-destructive/90"
              onClick={() =>
                periodToDelete &&
                deleteMutation.mutate(
                  { path: { id: periodToDelete.id } },
                  { onSuccess: () => setPeriodToDelete(null) },
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
