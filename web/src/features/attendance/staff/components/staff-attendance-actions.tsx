import type { AttendanceStatus } from '@/lib/api/types.gen' // Changed from StaffAttendanceStatus
import { Button } from '@/components/ui/button'
import { HStack } from '@/components/primitives'

export function StaffAttendanceActions({
  onBulkMark,
  onSave,
  onSyncLeaves,
  isSaving,
  isSyncing,
  isDirty,
}: {
  onBulkMark: (status: AttendanceStatus) => void // Changed from StaffAttendanceStatus
  onSave: () => void
  onSyncLeaves: () => void
  isSaving: boolean
  isSyncing: boolean
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
        <Button
          variant="outline"
          size="sm"
          onClick={onSyncLeaves}
          disabled={isSyncing}
        >
          {isSyncing ? 'Syncing...' : 'Sync Leaves'}
        </Button>
      </HStack>
      <Button onClick={onSave} disabled={!isDirty || isSaving} size="sm">
        {isSaving ? 'Saving...' : 'Save Changes'}
      </Button>
    </HStack>
  )
}
