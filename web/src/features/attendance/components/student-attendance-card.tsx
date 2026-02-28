import { Card } from '@/components/ui/card'
import { Badge } from '@/components/ui/badge'
import {
    Avatar,
    AvatarFallback,
    AvatarImage,
} from '@/components/ui/avatar'
import { Stack, HStack, Text } from '@/components/primitives'
import { Button } from '@/components/ui/button'
import type { AttendanceStatus } from '@/lib/api/types.gen'

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

    return (
        <Card className="p-3">
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
                                    className={`px-1.5 py-0 text-[10px] border-none ${getStatusColor(
                                        status,
                                    )}`}
                                >
                                    {status}
                                </Badge>
                            )}
                        </HStack>
                    </Stack>
                </HStack>

                {/* Action Toggles */}
                <HStack gap={1} className="w-full mt-2">
                    {(['Present', 'Absent', 'Late', 'Excused'] as AttendanceStatus[]).map(
                        (s) => (
                            <Button
                                key={s}
                                variant={status === s ? 'secondary' : 'ghost'}
                                size="sm"
                                className={`flex-1 h-7 text-xs px-0 ${status === s
                                        ? getStatusColor(s)
                                        : 'text-zinc-400 hover:text-zinc-200'
                                    }`}
                                onClick={() => onStatusChange(s)}
                            >
                                {s.charAt(0)}
                            </Button>
                        ),
                    )}
                </HStack>
            </Stack>
        </Card>
    )
}
