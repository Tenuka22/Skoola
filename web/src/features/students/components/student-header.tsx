import { Badge } from '@/components/ui/badge'

interface StudentHeaderProps {
  totalStudents?: number
}

export function StudentHeader({ totalStudents = 0 }: StudentHeaderProps) {
  return (
    <div className="px-8 py-6 pb-2">
      <div className="mb-1 flex items-center gap-3">
        <h1 className="text-2xl font-semibold tracking-tight">
          Student Management
        </h1>
        <Badge
          variant="secondary"
          className="rounded-md bg-muted px-2 py-0.5 text-xs font-normal text-muted-foreground hover:bg-muted"
        >
          {totalStudents}
        </Badge>
      </div>
      <p className="text-sm text-muted-foreground">
        Manage student records, enrollments, and academic profiles.
      </p>
    </div>
  )
}
