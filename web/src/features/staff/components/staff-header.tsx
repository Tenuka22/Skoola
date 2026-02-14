import { Badge } from '@/components/ui/badge'

interface StaffHeaderProps {
  totalStaff?: number
}

export function StaffHeader({ totalStaff = 0 }: StaffHeaderProps) {
  return (
    <div className="px-8 py-6 pb-2">
      <div className="mb-1 flex items-center gap-3">
        <h1 className="text-2xl font-semibold tracking-tight">
          Staff Management
        </h1>
        <Badge
          variant="secondary"
          className="rounded-md bg-muted px-2 py-0.5 text-xs font-normal text-muted-foreground hover:bg-muted"
        >
          {totalStaff}
        </Badge>
      </div>
      <p className="text-sm text-muted-foreground">
        Manage your school's staff members, teachers, and administrators.
      </p>
    </div>
  )
}
