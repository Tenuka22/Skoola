import { HugeiconsIcon } from '@hugeicons/react'
import { Delete02Icon, Edit04Icon } from '@hugeicons/core-free-icons'
import type { ColumnDef } from '@tanstack/react-table'
import type { Permission, PermissionSeverity } from '@/lib/api/types.gen'
import { Badge } from '@/components/ui/badge'
import { Button } from '@/components/ui/button'

interface GetPermissionColumnsProps {
  onEdit: (permission: Permission) => void
  onDelete: (permissionId: number) => void
}

const getSeverityColor = (severity: PermissionSeverity) => {
  switch (severity) {
    case 'Low':
      return 'text-green-500 bg-green-500/10 border-green-500/20'
    case 'Medium':
      return 'text-blue-500 bg-blue-500/10 border-blue-500/20'
    case 'High':
      return 'text-orange-500 bg-orange-500/10 border-orange-500/20'
    case 'Severe':
      return 'text-red-500 bg-red-500/10 border-red-500/20'
    default:
      return 'text-muted-foreground bg-muted border-border'
  }
}

export const getPermissionColumns = ({
  onEdit,
  onDelete,
}: GetPermissionColumnsProps): Array<ColumnDef<Permission>> => [
  {
    accessorKey: 'name',
    header: 'Name',
    cell: ({ row }) => <div className="font-medium">{row.original.name}</div>,
  },
  {
    accessorKey: 'description',
    header: 'Description',
  },
  {
    accessorKey: 'safety_level',
    header: 'Safety Level',
    cell: ({ row }) => (
      <Badge
        className={getSeverityColor(row.original.safety_level)}
        variant="outline"
      >
        {row.original.safety_level}
      </Badge>
    ),
  },
  {
    accessorKey: 'is_admin_only',
    header: 'Admin Only',
    cell: ({ row }) => (row.original.is_admin_only ? 'Yes' : 'No'),
  },
  {
    id: 'actions',
    header: 'Actions',
    cell: ({ row }) => (
      <div className="flex gap-2">
        <Button
          variant="outline"
          size="icon"
          onClick={() => onEdit(row.original)}
        >
          <HugeiconsIcon icon={Edit04Icon} className="size-4" />
        </Button>
        <Button
          variant="destructive"
          size="icon"
          onClick={() => onDelete(row.original.id)}
        >
          <HugeiconsIcon icon={Delete02Icon} className="size-4" />
        </Button>
      </div>
    ),
  },
]
