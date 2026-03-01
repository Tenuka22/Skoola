import type { AttendanceStatus } from '@/lib/api/types.gen'
import { Card } from '@/components/ui/card'
import { Badge } from '@/components/ui/badge'
import { Avatar, AvatarFallback, AvatarImage } from '@/components/ui/avatar'
import { HStack, Stack, Text } from '@/components/primitives'
import { Button } from '@/components/ui/button'
import { cn } from '@/lib/utils'

interface StudentAttendanceCardProps {
  student: {
    id: string
    name_english?: string
    admission_number?: string
    profile_picture_url?: string
  }
  status?: AttendanceStatus
  onStatusChange: (status: AttendanceStatus) => void
}

export function StudentAttendanceCard({
  student,
  status,
  onStatusChange,
}: StudentAttendanceCardProps) {
  const getInitials = (name?: string) => {
    if (!name) return '?'
    return name
      .split(' ')
      .slice(0, 2)
      .map((n) => n[0])
      .join('')
      .toUpperCase()
  }

  const getStatusColor = (s?: AttendanceStatus) => {
    switch (s) {
      case 'Present':
        return 'bg-emerald-500/15 text-emerald-500 hover:bg-emerald-500/25'
      case 'Absent':
        return 'bg-red-500/15 text-red-500 hover:bg-red-500/25'
      case 'Late':
        return 'bg-amber-500/15 text-amber-500 hover:bg-amber-500/25'
      case 'Excused':
        return 'bg-blue-500/15 text-blue-500 hover:bg-blue-500/25'
      default:
        return 'bg-zinc-500/15 text-zinc-500 hover:bg-zinc-500/25'
    }
  }

  const statuses: Array<AttendanceStatus> = [
    'Present',
    'Absent',
    'Late',
    'Excused',
  ]

  const handleStatusClick = (e: React.MouseEvent, s: AttendanceStatus) => {
    e.preventDefault()
    e.stopPropagation()
    onStatusChange(s)
  }

  return (
    <Card
      className={cn(
        'p-3 transition-all duration-200 border-2',
        status ? 'border-primary/20 shadow-md' : 'border-transparent',
      )}
    >
      <Stack gap={2}>
        {/* Header Info */}
        <HStack align="start" gap={3}>
          <Avatar className="h-10 w-10 border border-zinc-800">
            <AvatarImage src={student.profile_picture_url || ''} />
            <AvatarFallback className="bg-zinc-800 text-xs text-zinc-400">
              {getInitials(student.name_english)}
            </AvatarFallback>
          </Avatar>
          <Stack gap={1} className="flex-1 overflow-hidden">
            <Text size="sm" className="font-medium truncate">
              {student.name_english || 'Unknown Student'}
            </Text>
            <HStack gap={2} align="center">
              <Text size="xs" muted>
                {student.admission_number || 'No ID'}
              </Text>
              {status && (
                <Badge
                  variant="secondary"
                  className={cn(
                    'px-1.5 py-0 text-[10px] border-none animate-in fade-in zoom-in duration-300',
                    getStatusColor(status),
                  )}
                >
                  {status}
                </Badge>
              )}
            </HStack>
          </Stack>
        </HStack>

        {/* Action Toggles */}
        <HStack gap={1} className="w-full mt-2">
          {statuses.map((s) => (
            <Button
              key={s}
              variant={status === s ? 'secondary' : 'outline'}
              size="sm"
              className={cn(
                'flex-1 h-8 text-xs px-0 font-bold transition-all',
                status === s
                  ? getStatusColor(s)
                  : 'text-muted-foreground hover:text-foreground border-muted',
              )}
              onClick={(e) => handleStatusClick(e, s)}
            >
              {s.charAt(0)}
            </Button>
          ))}
        </HStack>
      </Stack>
    </Card>
  )
}
