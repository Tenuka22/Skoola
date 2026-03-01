import { useMutation, useQuery } from '@tanstack/react-query'
import { Suspense, useMemo, useState } from 'react'
import { format } from 'date-fns'
import { useClasses, useStaffList } from '../api'
import type { StaffResponse } from '@/lib/api/types.gen'
import { Box, HStack, Heading, Stack, Text } from '@/components/primitives'
import {
  Card,
  CardContent,
  CardDescription,
  CardHeader,
  CardTitle,
} from '@/components/ui/card'
import { Button } from '@/components/ui/button'
import { Input } from '@/components/ui/input'
import {
  Select,
  SelectContent,
  SelectItem,
  SelectTrigger,
  SelectValue,
} from '@/components/ui/select'
import {
  createSubstitutionMutation,
  initiateEmergencyRollCallMutation,
  runDiscrepancyCheckOptions,
  sendAbsenceNotificationsMutation,
  suggestSubstituteMutation,
  syncPreApprovedAbsencesMutation,
  syncSchoolBusinessMutation,
} from '@/lib/api/@tanstack/react-query.gen'
import { CalendarInput } from '@/components/ui/calendar-input'
import { FullPageSpinner } from '@/components/ui/full-page-spinner'
import { authClient } from '@/lib/clients'

function StudentActions() {
  const { data: classes } = useClasses()
  const [selectedClassId, setSelectedClassId] = useState<string | undefined>()
  const [notificationDate, setNotificationDate] = useState<Date>(new Date())
  const [syncAbsenceDate, setSyncAbsenceDate] = useState<Date>(new Date())
  const [syncBusinessDate, setSyncBusinessDate] = useState<Date>(new Date())
  const [discrepancyDate, setDiscrepancyDate] = useState<Date>(new Date())

  const { mutate: sendNotifications, isPending: isSending } = useMutation(
    sendAbsenceNotificationsMutation({ client: authClient }),
  )
  const { mutate: syncAbsences, isPending: isSyncingAbsences } = useMutation(
    syncPreApprovedAbsencesMutation({ client: authClient }),
  )
  const { mutate: syncBusiness, isPending: isSyncingBusiness } = useMutation(
    syncSchoolBusinessMutation({ client: authClient }),
  )

  const {
    refetch: refetchDiscrepancyCheck,
    isFetching: isRunningCheck,
    error: discrepancyCheckError,
  } = useQuery({
    ...runDiscrepancyCheckOptions({
      client: authClient,
      path: { date: format(discrepancyDate, 'yyyy-MM-dd') },
    }),
    enabled: false,
  })

  return (
    <Card className="border-none shadow-xl bg-card overflow-hidden">
      <CardHeader className="bg-muted/20 border-b px-6 py-4">
        <CardTitle className="text-sm font-bold">Student Actions</CardTitle>
        <CardDescription className="text-xs">
          Actions related to student attendance.
        </CardDescription>
      </CardHeader>
      <CardContent className="p-6">
        <Stack gap={4}>
          <HStack
            justify="between"
            align="center"
            className="p-4 rounded-xl border bg-muted/5 border-border/40"
          >
            <Text className="font-bold text-sm">Send Absence Notifications</Text>
            <HStack gap={3}>
              <Select
                value={selectedClassId}
                onValueChange={(value) => setSelectedClassId(value ?? undefined)}
              >
                <SelectTrigger className="w-[240px] rounded-xl h-10 ring-1 ring-border">
                  <SelectValue placeholder="Select a class" />
                </SelectTrigger>
                <SelectContent>
                  {classes?.data?.map((c) => (
                    <SelectItem key={c.id} value={c.id}>
                      {c.section_name} - {c.id}{' '}
                    </SelectItem>
                  ))}
                </SelectContent>
              </Select>
              <CalendarInput
                value={notificationDate}
                onChange={setNotificationDate}
              />
              <Button
                className="rounded-xl font-bold h-10"
                onClick={() => {
                  if (selectedClassId) {
                    sendNotifications({
                      body: {
                        class_id: selectedClassId,
                        date: format(notificationDate, 'yyyy-MM-dd'),
                      },
                    })
                  }
                }}
                disabled={!selectedClassId || isSending}
              >
                {isSending ? 'Sending...' : 'Send'}
              </Button>
            </HStack>
          </HStack>
          
          <HStack
            justify="between"
            align="center"
            className="p-4 rounded-xl border bg-muted/5 border-border/40"
          >
            <Text className="font-bold text-sm">Sync Pre-approved Absences</Text>
            <HStack gap={3}>
              <CalendarInput
                value={syncAbsenceDate}
                onChange={setSyncAbsenceDate}
              />
              <Button
                className="rounded-xl font-bold h-10"
                onClick={() =>
                  syncAbsences({
                    path: { date: format(syncAbsenceDate, 'yyyy-MM-dd') },
                  })
                }
                disabled={isSyncingAbsences}
              >
                {isSyncingAbsences ? 'Syncing...' : 'Sync'}
              </Button>
            </HStack>
          </HStack>
          
          <HStack
            justify="between"
            align="center"
            className="p-4 rounded-xl border bg-muted/5 border-border/40"
          >
            <Text className="font-bold text-sm">Sync School Business</Text>
            <HStack gap={3}>
              <CalendarInput
                value={syncBusinessDate}
                onChange={setSyncBusinessDate}
              />
              <Button
                className="rounded-xl font-bold h-10"
                onClick={() =>
                  syncBusiness({
                    path: { date: format(syncBusinessDate, 'yyyy-MM-dd') },
                  })
                }
                disabled={isSyncingBusiness}
              >
                {isSyncingBusiness ? 'Syncing...' : 'Sync'}
              </Button>
            </HStack>
          </HStack>
          
          <HStack
            justify="between"
            align="center"
            className="p-4 rounded-xl border bg-muted/5 border-border/40"
          >
            <Text className="font-bold text-sm">Run Discrepancy Check</Text>
            <HStack gap={3}>
              <CalendarInput
                value={discrepancyDate}
                onChange={setDiscrepancyDate}
              />
              <Button
                className="rounded-xl font-bold h-10"
                onClick={() => void refetchDiscrepancyCheck()}
                disabled={isRunningCheck}
              >
                {isRunningCheck ? 'Running...' : 'Run'}
              </Button>
            </HStack>
          </HStack>
          {discrepancyCheckError && (
            <Text className="text-red-500 text-xs font-bold px-2">
              Error running discrepancy check: {discrepancyCheckError.message}
            </Text>
          )}
        </Stack>
      </CardContent>
    </Card>
  )
}

function StaffActions() {
  const mockTimetables = useMemo(
    () => [
      {
        id: 'tt1',
        class_name: 'Grade 10',
        subject_name: 'Math',
        day_of_week: 'Monday',
      },
      {
        id: 'tt2',
        class_name: 'Grade 11',
        subject_name: 'Science',
        day_of_week: 'Tuesday',
      },
      {
        id: 'tt3',
        class_name: 'Grade 10',
        subject_name: 'English',
        day_of_week: 'Wednesday',
      },
    ],
    [],
  )

  const { data: teachers } = useStaffList({ staff_type: 'Teaching' })

  const [suggestDate, setSuggestDate] = useState<Date>(new Date())
  const [selectedTimetableId, setSelectedTimetableId] = useState<
    string | undefined
  >()
  const [suggestedTeacher, setSuggestedTeacher] =
    useState<StaffResponse | null>(null)

  const [createDate, setCreateDate] = useState<Date>(new Date())
  const [createTimetableId, setCreateTimetableId] = useState<
    string | undefined
  >()
  const [originalTeacherId, setOriginalTeacherId] = useState<
    string | undefined
  >()

  const { mutate: suggest, isPending: isSuggesting } = useMutation(
    suggestSubstituteMutation({ client: authClient }),
  )
  const { mutate: create, isPending: isCreating } = useMutation(
    createSubstitutionMutation({ client: authClient }),
  )

  const handleSuggest = () => {
    if (!selectedTimetableId) return
    suggest(
      {
        body: {
          timetable_id: selectedTimetableId,
          date: format(suggestDate, 'yyyy-MM-dd'),
        },
      },
      {
        onSuccess: (data) => {
          setSuggestedTeacher(data)
        },
      },
    )
  }

  const handleCreate = () => {
    if (!createTimetableId || !originalTeacherId) return
    create({
      body: {
        original_teacher_id: originalTeacherId,
        timetable_id: createTimetableId,
        date: format(createDate, 'yyyy-MM-dd'),
      },
    })
  }

  return (
    <Card className="border-none shadow-xl bg-card overflow-hidden">
      <CardHeader className="bg-muted/20 border-b px-6 py-4">
        <CardTitle className="text-sm font-bold">Staff Actions</CardTitle>
        <CardDescription className="text-xs">
          Manage substitutions for absent teachers.
        </CardDescription>
      </CardHeader>
      <CardContent className="p-6">
        <Stack gap={4}>
          <HStack
            justify="between"
            align="center"
            className="p-4 rounded-xl border bg-muted/5 border-border/40"
          >
            <Text className="font-bold text-sm">Suggest Substitute</Text>
            <HStack gap={3}>
              <Select
                value={selectedTimetableId}
                onValueChange={(value) =>
                  setSelectedTimetableId(value ?? undefined)
                }
              >
                <SelectTrigger className="w-[200px] rounded-xl h-10 ring-1 ring-border">
                  <SelectValue placeholder="Select Timetable" />
                </SelectTrigger>
                <SelectContent>
                  {mockTimetables.map((t) => (
                    <SelectItem key={t.id} value={t.id}>
                      {t.class_name} - {t.subject_name} ({t.day_of_week})
                    </SelectItem>
                  ))}
                </SelectContent>
              </Select>
              <CalendarInput value={suggestDate} onChange={setSuggestDate} />
              <Button
                className="rounded-xl font-bold h-10"
                onClick={handleSuggest}
                disabled={!selectedTimetableId || isSuggesting}
              >
                {isSuggesting ? 'Suggesting...' : 'Suggest'}
              </Button>
            </HStack>
          </HStack>
          {suggestedTeacher && (
            <Box className="px-4 py-2 bg-primary/10 rounded-lg border border-primary/20">
              <Text size="xs" className="font-bold text-primary">Suggested: {suggestedTeacher.name}</Text>
            </Box>
          )}

          <HStack
            justify="between"
            align="center"
            className="p-4 rounded-xl border bg-muted/5 border-border/40"
          >
            <Text className="font-bold text-sm">Create Substitution</Text>
            <HStack gap={3}>
              <Select
                value={originalTeacherId}
                onValueChange={(value) =>
                  setOriginalTeacherId(value ?? undefined)
                }
              >
                <SelectTrigger className="w-[180px] rounded-xl h-10 ring-1 ring-border">
                  <SelectValue placeholder="Original Teacher" />
                </SelectTrigger>
                <SelectContent>
                  {teachers?.data?.map((t) => (
                    <SelectItem key={t.id} value={t.id}>
                      {t.name}
                    </SelectItem>
                  ))}
                </SelectContent>
              </Select>
              <Select
                value={createTimetableId}
                onValueChange={(value) =>
                  setCreateTimetableId(value ?? undefined)
                }
              >
                <SelectTrigger className="w-[180px] rounded-xl h-10 ring-1 ring-border">
                  <SelectValue placeholder="Select Timetable" />
                </SelectTrigger>
                <SelectContent>
                  {mockTimetables.map((t) => (
                    <SelectItem key={t.id} value={t.id}>
                      {t.class_name} - {t.subject_name} ({t.day_of_week})
                    </SelectItem>
                  ))}
                </SelectContent>
              </Select>
              <CalendarInput value={createDate} onChange={setCreateDate} />
              <Button
                className="rounded-xl font-bold h-10"
                onClick={handleCreate}
                disabled={!createTimetableId || !originalTeacherId || isCreating}
              >
                {isCreating ? 'Creating...' : 'Create'}
              </Button>
            </HStack>
          </HStack>
        </Stack>
      </CardContent>
    </Card>
  )
}

function EmergencyActions() {
  const [eventName, setEventName] = useState('')
  const { mutate: initiate, isPending } = useMutation(
    initiateEmergencyRollCallMutation({ client: authClient }),
  )

  return (
    <Card className="border-none shadow-xl bg-card overflow-hidden">
      <CardHeader className="bg-muted/20 border-b px-6 py-4">
        <CardTitle className="text-sm font-bold">Emergency Actions</CardTitle>
        <CardDescription className="text-xs">
          Initiate and manage emergency roll calls.
        </CardDescription>
      </CardHeader>
      <CardContent className="p-6">
        <HStack
          justify="between"
          align="center"
          className="p-4 rounded-xl border bg-muted/5 border-border/40"
        >
          <Text className="font-bold text-sm">Initiate Emergency Roll Call</Text>
          <HStack gap={3}>
            <Input
              placeholder="Event Name (e.g., Fire Drill)"
              value={eventName}
              onChange={(e) => setEventName(e.target.value)}
              className="w-[240px] rounded-xl h-10 ring-1 ring-border"
            />
            <Button
              className="rounded-xl font-bold h-10"
              onClick={() => initiate({ body: { event_name: eventName } })}
              disabled={!eventName || isPending}
            >
              {isPending ? 'Initiating...' : 'Initiate'}
            </Button>
          </HStack>
        </HStack>
      </CardContent>
    </Card>
  )
}

export function AttendanceActionsPage() {
  return (
    <Stack gap={6} p={8} className="h-full">
      <Stack gap={1}>
        <Heading size="h2" className="font-black">Attendance Actions</Heading>
        <Text muted as="p">
          Perform bulk actions and trigger automated processes for attendance.
        </Text>
      </Stack>
      <Suspense fallback={<FullPageSpinner />}>
        <Stack gap={6}>
          <StudentActions />
          <StaffActions />
          <EmergencyActions />
        </Stack>
      </Suspense>
    </Stack>
  )
}

