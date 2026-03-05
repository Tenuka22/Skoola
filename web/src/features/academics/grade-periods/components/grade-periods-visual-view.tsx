import type { GradePeriodResponse } from '@/lib/api/types.gen'
import { Card } from '@/components/ui/card'
import { Badge } from '@/components/ui/badge'
import { Box, HStack, Stack, Text } from '@/components/primitives'
import { cn } from '@/lib/utils'
import { Empty } from '@/components/empty'

interface GradePeriodsVisualViewProps {
  periods: GradePeriodResponse[]
}

export function GradePeriodsVisualView({ periods }: GradePeriodsVisualViewProps) {
  // Sort periods by time
  const sortedPeriods = [...periods].sort((a, b) => a.start_time.localeCompare(b.start_time))

  if (sortedPeriods.length === 0) {
    return (
      <Empty
        title="No periods defined yet"
        description="Select a grade level and add your first period to see the timeline view."
        icon="empty"
        className="py-12"
      />
    )
  }

  return (
    <Box py={4} px={2}>
      <Stack gap={8} className="relative border-l-2 border-primary/20 pl-8 ml-4">
        {sortedPeriods.map((period) => (
          <Box key={period.id} className="relative">
            {/* Timeline Dot */}
            <Box
              className={cn(
                "absolute -left-[41px] top-1.5 h-4 w-4 rounded-full border-2 border-background shadow-sm",
                period.is_break ? "bg-secondary" : "bg-primary"
              )}
            />
            
            <Card className={cn(
              "p-4 transition-all hover:shadow-md",
              period.is_break ? "bg-muted/50 border-dashed" : "bg-card shadow-sm border-border/40"
            )}>
              <HStack justify="between" align="start">
                <Stack gap={2}>
                  <HStack gap={2} align="center">
                    <Text className="font-bold">Period {period.period_number}</Text>
                    {period.is_break ? (
                      <Badge variant="secondary" className="text-[10px] px-1.5 py-0 uppercase tracking-wider font-semibold">
                        Break
                      </Badge>
                    ) : (
                      <Badge variant="outline" className="text-[10px] px-1.5 py-0 uppercase tracking-wider font-semibold text-primary border-primary/20">
                        Lesson
                      </Badge>
                    )}
                  </HStack>
                  <HStack gap={4}>
                    <HStack gap={2} align="center">
                      <Box className="h-2 w-2 rounded-full bg-emerald-500 shadow-[0_0_8px_rgba(16,185,129,0.3)]" />
                      <Text size="xs" className="font-medium text-muted-foreground/80">{period.start_time}</Text>
                    </HStack>
                    <HStack gap={2} align="center">
                      <Box className="h-2 w-2 rounded-full bg-rose-500 shadow-[0_0_8px_rgba(244,63,94,0.3)]" />
                      <Text size="xs" className="font-medium text-muted-foreground/80">{period.end_time}</Text>
                    </HStack>
                  </HStack>
                </Stack>
                
                <Box bg="muted" px={2} py={1} rounded="md" className="border border-border/20 shadow-sm">
                  <Text size="xs" muted className="font-mono font-semibold whitespace-nowrap">
                    {calculateDuration(period.start_time, period.end_time)}
                  </Text>
                </Box>
              </HStack>
            </Card>
          </Box>
        ))}
      </Stack>
    </Box>
  )
}

function calculateDuration(start: string, end: string): string {
  try {
    const [sH, sM, sS] = start.split(':').map(Number)
    const [eH, eM, eS] = end.split(':').map(Number)
    
    const startTotalSec = sH * 3600 + sM * 60 + (sS || 0)
    const endTotalSec = eH * 3600 + eM * 60 + (eS || 0)
    
    let diffSec = endTotalSec - startTotalSec
    if (diffSec < 0) diffSec += 24 * 3600 // Handle overnight periods if any
    
    const hours = Math.floor(diffSec / 3600)
    const minutes = Math.floor((diffSec % 3600) / 60)
    
    if (hours > 0) {
      return `${hours}h ${minutes}m`
    }
    return `${minutes} min`
  } catch (e) {
    return 'Unknown'
  }
}
