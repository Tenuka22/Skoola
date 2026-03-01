import type { ColumnDef } from '@tanstack/react-table'
import type {
  AttendanceStatus,
  EnrichedStudentAttendance,
  StudentResponse,
} from '@/lib/api/types.gen'
import { isAttendanceStatus } from '@/features/attendance/types'
import { Checkbox } from '@/components/ui/checkbox'
import { Avatar, AvatarFallback, AvatarImage } from '@/components/ui/avatar'
import { HStack, Stack, Text } from '@/components/primitives'
import { Badge } from '@/components/ui/badge'
import {
  Select,
  SelectContent,
  SelectItem,
  SelectTrigger,
  SelectValue,
} from '@/components/ui/select'

interface GetStudentAttendanceColumnsProps {
  attendance: Record<string, AttendanceStatus>
  onStatusChange: (studentId: string, status: AttendanceStatus) => void
}

export type LocalEnrichedStudentAttendance = EnrichedStudentAttendance &
  Pick<StudentResponse, 'profile_photo_url' | 'admission_number'>

export const getStudentAttendanceColumns = ({
  attendance,
  onStatusChange,
}: GetStudentAttendanceColumnsProps): Array<
  ColumnDef<LocalEnrichedStudentAttendance & { id: string | number }>
> => {
  return [
    {
      id: 'select',
      header: ({ table }) => (
        <div className="pl-4">
          <Checkbox
            checked={table.getIsAllPageRowsSelected()}
            indeterminate={table.getIsSomeRowsSelected()}
            onCheckedChange={(value) =>
              table.toggleAllPageRowsSelected(!!value)
            }
            aria-label="Select all"
          />
        </div>
      ),
      cell: ({ row }) => (
        <div className="pl-4">
          <Checkbox
            checked={row.getIsSelected()}
            onCheckedChange={(value) => row.toggleSelected(!!value)}
            aria-label="Select row"
          />
        </div>
      ),
      enableSorting: false,
      enableHiding: false,
    },
    {
      accessorKey: 'student_name',
      header: 'Student',
      cell: ({ row }) => {
        const student: LocalEnrichedStudentAttendance = row.original
        return (
          <HStack gap={3}>
            <Avatar className="h-9 w-9">
              <AvatarImage
                src={student.profile_photo_url || undefined}
                alt={student.student_name}
              />
              <AvatarFallback>{student.student_name.charAt(0)}</AvatarFallback>
            </Avatar>
            <Stack gap={1}>
              <Text className="font-medium">{student.student_name}</Text>
              <Text muted size="xs">
                {student.admission_number}
              </Text>
            </Stack>
          </HStack>
        )
      },
    },
    {
      accessorKey: 'medical_alerts',
      header: 'Alerts',
      cell: ({ row }) => {
        const student: LocalEnrichedStudentAttendance = row.original
        return student.medical_alerts ? (
          <Badge variant="destructive">Medical Alert</Badge>
        ) : null
      },
    },
    {
      accessorKey: 'status',
      header: 'Status',
      cell: ({ row }) => {
        const student: LocalEnrichedStudentAttendance = row.original
        return (
          <Select
            value={
              attendance[student.student_id] || student.status || 'Present'
            }
            onValueChange={(value) => {
              if (value && isAttendanceStatus(value))
                onStatusChange(student.student_id, value)
            }}
          >
            <SelectTrigger className="w-[180px]">
              <SelectValue placeholder="Set status" />
            </SelectTrigger>
            <SelectContent>
              <SelectItem value="Present">Present</SelectItem>
              <SelectItem value="Absent">Absent</SelectItem>
              <SelectItem value="Late">Late</SelectItem>
              <SelectItem value="Excused">Excused</SelectItem>
              <SelectItem value="SchoolBusiness">School Business</SelectItem>
            </SelectContent>
          </Select>
        )
      },
    },
  ]
}
