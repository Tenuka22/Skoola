import * as React from 'react'
import { useQuery } from '@tanstack/react-query'
import { format } from 'date-fns'
import {
  Add01Icon,
  AlertCircleIcon,
  Delete02Icon,
  PencilEdit01Icon,
} from '@hugeicons/core-free-icons'
import { HugeiconsIcon } from '@hugeicons/react'

import {
  getAllBehaviorIncidentTypesQueryOptions,
  getStudentBehaviorIncidentsQueryOptions,
  useDeleteBehaviorIncident,
  useRecordBehaviorIncident,
  useUpdateBehaviorIncident,
} from '../api'
import { BehaviorIncidentDialog } from './behavior-incident-dialog'
import type {
  BehaviorIncidentResponse,
  StudentResponse,
} from '@/lib/api/types.gen'
import type { BehaviorIncidentFormValues } from '../schemas'
import {
  Dialog,
  DialogContent,
  DialogDescription,
  DialogHeader,
  DialogTitle,
} from '@/components/ui/dialog'
import { Button } from '@/components/ui/button'
import { Badge } from '@/components/ui/badge'
import { ScrollArea } from '@/components/ui/scroll-area'
import { Spinner } from '@/components/ui/spinner'
import { HStack, Stack, Text } from '@/components/primitives'

interface StudentBehaviorDialogProps {
  student: StudentResponse | null
  open: boolean
  onOpenChange: (open: boolean) => void
}

export function StudentBehaviorDialog({
  student,
  open,
  onOpenChange,
}: StudentBehaviorDialogProps) {
  const [isRecordOpen, setIsRecordOpen] = React.useState(false)
  const [incidentToEdit, setIncidentToEdit] =
    React.useState<BehaviorIncidentResponse | null>(null)

  const { data: incidents, isLoading } = useQuery({
    ...getStudentBehaviorIncidentsQueryOptions({
      path: { student_id: student?.id ?? '' },
    }),
    enabled: !!student && open,
  })

  const { data: typesData } = useQuery({
    ...getAllBehaviorIncidentTypesQueryOptions(),
    enabled: open,
  })

  const types = typesData || []

  const recordMutation = useRecordBehaviorIncident()

  const updateMutation = useUpdateBehaviorIncident()

  const deleteMutation = useDeleteBehaviorIncident()

  const getTypeName = (typeId: string) => {
    return types.find((t) => t.id === typeId)?.type_name || 'Unknown Type'
  }

  return (
    <Dialog open={open} onOpenChange={onOpenChange}>
      <DialogContent className="sm:max-w-[600px] h-[80vh] flex flex-col p-0">
        <DialogHeader className="p-6 pb-0">
          <HStack className="justify-between items-start">
            <Stack gap={1}>
              <DialogTitle>Behavior History</DialogTitle>
              <DialogDescription>
                {student?.name_english} ({student?.admission_number})
              </DialogDescription>
            </Stack>
            <Button size="sm" onClick={() => setIsRecordOpen(true)}>
              <HugeiconsIcon icon={Add01Icon} className="mr-2 size-4" />
              Record Incident
            </Button>
          </HStack>
        </DialogHeader>

        <div className="flex-1 overflow-hidden p-6">
          {isLoading ? (
            <div className="flex h-full items-center justify-center">
              <Spinner />
            </div>
          ) : !incidents || incidents.length === 0 ? (
            <div className="flex h-full flex-col items-center justify-center text-center">
              <HugeiconsIcon
                icon={AlertCircleIcon}
                className="size-12 text-muted-foreground opacity-20"
              />
              <Text muted className="mt-4">
                No behavior incidents recorded for this student.
              </Text>
            </div>
          ) : (
            <ScrollArea className="h-full pr-4">
              <Stack gap={4}>
                {incidents.map((incident) => (
                  <div
                    key={incident.id}
                    className="group relative rounded-lg border p-4 hover:bg-muted/50 transition-colors"
                  >
                    <HStack className="justify-between items-start mb-2">
                      <Stack gap={1}>
                        <HStack gap={2}>
                          <Text className="font-semibold">
                            {getTypeName(incident.incident_type_id)}
                          </Text>
                          <Badge
                            variant={
                              incident.points_awarded < 0
                                ? 'destructive'
                                : 'secondary'
                            }
                            className="text-[10px] px-1.5 py-0"
                          >
                            {incident.points_awarded > 0 ? '+' : ''}
                            {incident.points_awarded} points
                          </Badge>
                        </HStack>
                        <Text size="xs" muted>
                          {format(new Date(incident.incident_date), 'PPP')}
                        </Text>
                      </Stack>
                      <HStack
                        gap={2}
                        className="opacity-0 group-hover:opacity-100 transition-opacity"
                      >
                        <Button
                          variant="ghost"
                          size="icon"
                          className="size-8"
                          onClick={() => setIncidentToEdit(incident)}
                        >
                          <HugeiconsIcon
                            icon={PencilEdit01Icon}
                            className="size-4"
                          />
                        </Button>
                        <Button
                          variant="ghost"
                          size="icon"
                          className="size-8 text-destructive hover:text-destructive"
                          onClick={() =>
                            deleteMutation.mutate({
                              path: { incident_id: incident.id },
                            })
                          }
                        >
                          <HugeiconsIcon
                            icon={Delete02Icon}
                            className="size-4"
                          />
                        </Button>
                      </HStack>
                    </HStack>
                    <Text size="sm">{incident.description}</Text>
                  </div>
                ))}
              </Stack>
            </ScrollArea>
          )}
        </div>

        <BehaviorIncidentDialog
          student={student}
          open={isRecordOpen}
          onOpenChange={setIsRecordOpen}
          onConfirm={(data: BehaviorIncidentFormValues) =>
            recordMutation.mutate(
              {
                body: { ...data, student_id: student?.id ?? '' },
              },
              {
                onSuccess: () => {
                  setIsRecordOpen(false)
                },
              },
            )
          }
          isSubmitting={recordMutation.isPending}
        />

        <BehaviorIncidentDialog
          student={student}
          incident={incidentToEdit}
          open={!!incidentToEdit}
          onOpenChange={() => setIncidentToEdit(null)}
          onConfirm={(data: BehaviorIncidentFormValues) =>
            incidentToEdit &&
            updateMutation.mutate(
              {
                path: { incident_id: incidentToEdit.id },
                body: data,
              },
              {
                onSuccess: () => {
                  setIncidentToEdit(null)
                },
              },
            )
          }
          isSubmitting={updateMutation.isPending}
        />
      </DialogContent>
    </Dialog>
  )
}
