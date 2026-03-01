import type { AttendanceStatus } from '@/lib/api/types.gen' // Changed from StudentAttendanceStatus
import { Button } from '@/components/ui/button'
import { HStack } from '@/components/primitives'

export function StudentAttendanceActions({
  onBulkMark,
  onSave,
  isSaving,
  isDirty,
}: {
  onBulkMark: (status: AttendanceStatus) => void // Changed from StudentAttendanceStatus
  onSave: () => void
  isSaving: boolean
  isDirty: boolean
}) {
  return (
    <HStack justify="between">
      <HStack gap={2}>
        <Button
          variant="outline"
          size="sm"
          onClick={() => onBulkMark('Present')}
        >
          Mark All Present
        </Button>
        <Button
          variant="outline"
          size="sm"
          onClick={() => onBulkMark('Absent')}
        >
          Mark All Absent
        </Button>
      </HStack>
      <Button onClick={onSave} disabled={!isDirty || isSaving} size="sm">
        {isSaving ? 'Saving...' : 'Save Changes'}
      </Button>
    </HStack>
  )
}
