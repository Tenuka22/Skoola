import { format } from 'date-fns'
import { HugeiconsIcon } from '@hugeicons/react'
import {
  Calendar01Icon,
  Delete02Icon,
  Mail01Icon,
  MoreVerticalIcon,
  PencilEdit01Icon,
} from '@hugeicons/core-free-icons'
import type { StudentResponse } from '@/lib/api/types.gen'
import { Avatar, AvatarFallback, AvatarImage } from '@/components/ui/avatar'
import { Badge } from '@/components/ui/badge'
import { Button } from '@/components/ui/button'
import { Card, CardContent, CardHeader } from '@/components/ui/card'
import {
  DropdownMenu,
  DropdownMenuContent,
  DropdownMenuItem,
  DropdownMenuSeparator,
  DropdownMenuTrigger,
} from '@/components/ui/dropdown-menu'
import { Skeleton } from '@/components/ui/skeleton'

interface StudentBoardViewProps {
  students: Array<StudentResponse> | undefined
  isLoading?: boolean
  onEdit: (student: StudentResponse) => void
  onDelete: (id: string) => void
}

export function StudentBoardView({
  students,
  isLoading,
  onEdit,
  onDelete,
}: StudentBoardViewProps) {
  if (isLoading) {
    return (
      <div className="grid grid-cols-1 gap-4 sm:grid-cols-2 lg:grid-cols-3 xl:grid-cols-4">
        {Array.from({ length: 8 }).map((_, i) => (
          <Card key={i} className="overflow-hidden border-border/60 shadow-sm">
            <CardHeader className="flex flex-row items-center gap-4 p-4">
              <Skeleton className="h-10 w-10 rounded-full" />
              <div className="flex flex-col gap-2">
                <Skeleton className="h-4 w-32" />
                <Skeleton className="h-3 w-24" />
              </div>
            </CardHeader>
            <CardContent className="p-4 pt-0">
              <div className="space-y-2">
                <Skeleton className="h-3 w-full" />
                <Skeleton className="h-3 w-2/3" />
              </div>
            </CardContent>
          </Card>
        ))}
      </div>
    )
  }

  if (!students?.length) {
    return (
      <div className="flex h-64 flex-col items-center justify-center rounded-xl border border-dashed bg-muted/10">
        <p className="text-muted-foreground">No students found</p>
      </div>
    )
  }

  return (
    <div className="grid grid-cols-1 gap-4 sm:grid-cols-2 lg:grid-cols-3 xl:grid-cols-4">
      {students.map((student) => {
        const initials = student.name_english
          .split(' ')
          .map((n) => n[0])
          .join('')
          .toUpperCase()

        return (
          <Card
            key={student.id}
            className="overflow-hidden border-border/60 shadow-none"
          >
            <CardHeader className="flex flex-row items-start justify-between p-4">
              <div className="flex items-center gap-3">
                <Avatar className="h-10 w-10 border border-border/50">
                  <AvatarImage
                    src={
                      student.photo_url ||
                      `https://api.dicebear.com/7.x/avataaars/svg?seed=${student.id}`
                    }
                  />
                  <AvatarFallback className="bg-primary/10 text-primary text-xs font-bold">
                    {initials}
                  </AvatarFallback>
                </Avatar>
                <div>
                  <h3 className="font-semibold leading-none tracking-tight">
                    {student.name_english}
                  </h3>
                  <p className="text-xs text-muted-foreground mt-1">
                    {student.status}
                  </p>
                </div>
              </div>
              <DropdownMenu>
                <DropdownMenuTrigger
                  render={
                    <Button
                      variant="ghost"
                      size="icon"
                      className="h-8 w-8 -mr-2 text-muted-foreground"
                    >
                      <HugeiconsIcon
                        icon={MoreVerticalIcon}
                        className="size-4"
                      />
                    </Button>
                  }
                />
                <DropdownMenuContent align="end">
                  <DropdownMenuItem onClick={() => onEdit(student)}>
                    <HugeiconsIcon
                      icon={PencilEdit01Icon}
                      className="mr-2 size-4 opacity-70"
                    />
                    Edit
                  </DropdownMenuItem>
                  <DropdownMenuSeparator />
                  <DropdownMenuItem
                    onClick={() => onDelete(student.id)}
                    className="text-destructive focus:text-destructive"
                  >
                    <HugeiconsIcon
                      icon={Delete02Icon}
                      className="mr-2 size-4 opacity-70"
                    />
                    Delete
                  </DropdownMenuItem>
                </DropdownMenuContent>
              </DropdownMenu>
            </CardHeader>

            <CardContent className="p-4 pt-0 space-y-3">
              {student.email && (
                <div className="flex items-center gap-2 text-xs text-muted-foreground">
                  <HugeiconsIcon icon={Mail01Icon} className="size-3.5" />
                  <span className="truncate">{student.email}</span>
                </div>
              )}
              <div className="flex items-center gap-2 text-xs text-muted-foreground">
                <HugeiconsIcon icon={Calendar01Icon} className="size-3.5" />
                <span>
                  Joined {format(new Date(student.created_at), 'MMM d, yyyy')}
                </span>
              </div>
            </CardContent>

            <Badge
              variant="outline"
              className={`m-4 mt-0 border-0 bg-transparent px-0 font-medium ${student.status === 'Active' ? 'text-green-500' : 'text-amber-500'}`}
            >
              <span
                className={`mr-1.5 inline-block h-1.5 w-1.5 rounded-full ${student.status === 'Active' ? 'bg-green-500' : 'bg-amber-500'}`}
              />
              {student.status}
            </Badge>
          </Card>
        )
      })}
    </div>
  )
}
