import * as React from 'react'
import { HugeiconsIcon } from '@hugeicons/react'
import {
  Clock01Icon,
  Location01Icon,
  SchoolIcon,
  User02Icon,
} from '@hugeicons/core-free-icons'
import { useTimetablesStore } from '../store'
import { DAYS_OF_WEEK } from '../constants'
import type { TimetableEntryRow } from './timetables-table-columns'
import { Badge } from '@/components/ui/badge'
import { Card } from '@/components/ui/card'
import { Box, Grid, HStack, Stack, Text } from '@/components/primitives'
import { cn } from '@/lib/utils'

const TIMETABLE_COLS = 8

interface TimetablesVisualViewProps {
  data: Array<TimetableEntryRow>
}

export function TimetablesVisualView({ data }: TimetablesVisualViewProps) {
  const { viewMode, setTimetableEntryToEdit } = useTimetablesStore()

  // Get unique periods from data to determine grid rows
  const periods = React.useMemo(() => {
    const p = new Set<number>()
    data.forEach((entry) => p.add(entry.periodNumber))
    // Fallback to at least 8 periods if none found
    if (p.size === 0) return Array.from({ length: 8 }, (_, i) => i + 1)
    return Array.from(p).sort((a, b) => a - b)
  }, [data])

  // Group entries by Day and Period for easy lookup
  const scheduleMap = React.useMemo(() => {
    const map: Record<string, Record<number, TimetableEntryRow>> = {}
    DAYS_OF_WEEK.forEach((day) => {
      map[day] = {}
    })
    data.forEach((entry) => {
      if (map[entry.dayOfWeek]) {
        map[entry.dayOfWeek][entry.periodNumber] = entry
      }
    })
    return map
  }, [data])

  return (
    <div className="flex-1 overflow-auto p-8 bg-background/50">
      <Stack gap={6}>
        <div className="rounded-xl border border-border bg-card overflow-hidden shadow-sm">
          {/* Grid Header */}
          <Grid cols={TIMETABLE_COLS} gap={0} className="border-b">
            <Box className="p-4 bg-muted/30 border-r flex items-center justify-center">
              <HugeiconsIcon
                icon={Clock01Icon}
                className="size-4 text-muted-foreground"
              />
            </Box>
            {DAYS_OF_WEEK.map((day) => (
              <Box
                key={day}
                className="p-4 bg-muted/30 border-r last:border-r-0 flex flex-col items-center justify-center gap-1"
              >
                <Text
                  size="xs"
                  className="font-semibold uppercase tracking-wider text-muted-foreground"
                >
                  {day.substring(0, 3)}
                </Text>
                <Text size="sm" className="font-medium">
                  {day}
                </Text>
              </Box>
            ))}
          </Grid>

          {/* Grid Body */}
          <Stack gap={0} className="divide-y">
            {periods.map((period) => (
              <Grid
                key={period}
                cols={TIMETABLE_COLS}
                gap={0}
                className="min-h-32"
              >
                {/* Period Label */}
                <Box className="p-4 border-r bg-muted/10 flex flex-col items-center justify-center gap-1">
                  <Badge
                    variant="outline"
                    className="rounded-full px-2 py-0 h-5 text-[10px] bg-background"
                  >
                    P{period}
                  </Badge>
                  {/* Find first entry for this period to show approximate time */}
                  {(() => {
                    const sample = data.find((e) => e.periodNumber === period)
                    if (sample) {
                      return (
                        <Text
                          size="xs"
                          muted
                          className="text-center tabular-nums"
                        >
                          {sample.startTime.substring(0, 5)}
                        </Text>
                      )
                    }
                    return null
                  })()}
                </Box>

                {/* Days */}
                {DAYS_OF_WEEK.map((day) => {
                  const entry = scheduleMap[day][period]
                  return (
                    <Box
                      key={`${day}-${period}`}
                      className="p-2 border-r last:border-r-0 relative group hover:bg-muted/20 transition-colors"
                    >
                      {entry ? (
                        <Card
                          className={cn(
                            'h-full p-3 flex flex-col justify-between cursor-pointer transition-all hover:ring-2 hover:ring-primary/50 hover:shadow-md border-l-4',
                            getEntryColor(entry.subjectName),
                          )}
                          onClick={() => setTimetableEntryToEdit(entry.raw)}
                        >
                          <Stack gap={2}>
                            <HStack align="start" justify="between">
                              <Text
                                size="sm"
                                className="font-bold leading-tight line-clamp-2"
                              >
                                {entry.subjectName}
                              </Text>
                            </HStack>

                            <Stack gap={1}>
                              <HStack gap={1} align="center">
                                <HugeiconsIcon
                                  icon={
                                    viewMode === 'teacher'
                                      ? SchoolIcon
                                      : User02Icon
                                  }
                                  className="size-3 text-muted-foreground shrink-0"
                                />
                                <Text size="xs" muted className="truncate">
                                  {viewMode === 'teacher'
                                    ? entry.className
                                    : entry.teacherName}
                                </Text>
                              </HStack>

                              <HStack gap={1} align="center">
                                <HugeiconsIcon
                                  icon={Location01Icon}
                                  className="size-3 text-muted-foreground shrink-0"
                                />
                                <Text size="xs" muted className="truncate">
                                  Room {entry.room}
                                </Text>
                              </HStack>
                            </Stack>
                          </Stack>

                          <HStack justify="end">
                            <Text
                              size="xs"
                              className="font-mono text-[10px] opacity-50"
                            >
                              {entry.startTime.substring(0, 5)} -{' '}
                              {entry.endTime.substring(0, 5)}
                            </Text>
                          </HStack>
                        </Card>
                      ) : (
                        <div className="h-full w-full flex items-center justify-center opacity-0 group-hover:opacity-100 transition-opacity">
                          <Badge
                            variant="secondary"
                            className="bg-muted text-[10px] font-normal"
                          >
                            Free Slot
                          </Badge>
                        </div>
                      )}
                    </Box>
                  )
                })}
              </Grid>
            ))}
          </Stack>
        </div>
      </Stack>
    </div>
  )
}

function getEntryColor(subject: string): string {
  const s = subject.toLowerCase()
  if (s.includes('math')) return 'border-l-blue-500 bg-blue-500/5'
  if (
    s.includes('science') ||
    s.includes('physic') ||
    s.includes('chem') ||
    s.includes('bio')
  )
    return 'border-l-green-500 bg-green-500/5'
  if (s.includes('english') || s.includes('lit'))
    return 'border-l-orange-500 bg-orange-500/5'
  if (s.includes('hist') || s.includes('geo'))
    return 'border-l-purple-500 bg-purple-500/5'
  if (s.includes('art') || s.includes('music'))
    return 'border-l-pink-500 bg-pink-500/5'
  if (s.includes('it') || s.includes('comp'))
    return 'border-l-indigo-500 bg-indigo-500/5'
  if (s.includes('p.e') || s.includes('sport'))
    return 'border-l-red-500 bg-red-500/5'
  return 'border-l-slate-500 bg-slate-500/5'
}
