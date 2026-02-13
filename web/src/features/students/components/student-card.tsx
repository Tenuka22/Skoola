import { HugeiconsIcon } from '@hugeicons/react'
import {
  AiPhoneIcon,
  Calendar03Icon,
  Delete02Icon,
  MailIcon,
  MoreHorizontalIcon,
  PencilEdit01Icon,
  User03Icon,
} from '@hugeicons/core-free-icons'
import type { StudentResponse } from '@/features/students/types'
import { Avatar, AvatarFallback, AvatarImage } from '@/components/ui/avatar'
import { Card, CardContent } from '@/components/ui/card'
import {
  DropdownMenu,
  DropdownMenuContent,
  DropdownMenuItem,
  DropdownMenuSeparator,
  DropdownMenuTrigger,
} from '@/components/ui/dropdown-menu'
import { Button } from '@/components/ui/button'

interface StudentCardProps {
  student: StudentResponse
  onEdit?: (student: StudentResponse) => void
  onDelete?: (student: StudentResponse) => void
}

export function StudentCard({ student, onEdit, onDelete }: StudentCardProps) {
  const initials = student.name_english
    .split(' ')
    .map((n) => n[0])
    .join('')
    .toUpperCase()

  const dob = new Date(student.dob).toLocaleDateString('en-GB', {
    day: 'numeric',
    month: 'short',
    year: 'numeric',
  })

  const isActive = student.status === 'Active'

  return (
    <Card className="group relative flex flex-col p-6 rounded-[2.5rem] border-none bg-background shadow-xl shadow-muted/50 hover:shadow-2xl transition-all duration-300 hover:-translate-y-1 ring-1 ring-border overflow-hidden">
      <div className="absolute top-4 right-4 z-10 opacity-0 group-hover:opacity-100 transition-opacity">
        <DropdownMenu>
          <DropdownMenuTrigger
            render={
              <Button
                variant="ghost"
                size="icon"
                className="h-10 w-10 rounded-2xl bg-background/80 backdrop-blur-sm shadow-sm ring-1 ring-border"
              >
                <HugeiconsIcon icon={MoreHorizontalIcon} className="w-5 h-5" />
              </Button>
            }
          />
          <DropdownMenuContent align="end" className="w-48 rounded-2xl p-2">
            <DropdownMenuItem
              onClick={() => onEdit?.(student)}
              className="rounded-xl h-10"
            >
              <HugeiconsIcon icon={PencilEdit01Icon} className="w-4 h-4 mr-2" />{' '}
              Edit Student
            </DropdownMenuItem>
            <DropdownMenuSeparator />
            <DropdownMenuItem
              onClick={() => onDelete?.(student)}
              className="rounded-xl h-10 text-destructive focus:bg-destructive/10 focus:text-destructive"
            >
              <HugeiconsIcon icon={Delete02Icon} className="w-4 h-4 mr-2" />{' '}
              Remove Student
            </DropdownMenuItem>
          </DropdownMenuContent>
        </DropdownMenu>
      </div>

      <CardContent className="flex flex-col items-center p-0">
        <div className="relative mb-6">
          <div className="absolute inset-0 bg-primary/10 rounded-full scale-110 blur-xl group-hover:scale-125 transition-transform duration-500" />
          <Avatar className="w-28 h-28 border-4 border-background ring-1 ring-border shadow-inner">
            <AvatarImage
              src={student.photo_url || ''}
              alt={student.name_english}
              className="object-cover"
            />
            <AvatarFallback className="text-2xl font-black bg-muted text-muted-foreground">
              {initials}
            </AvatarFallback>
          </Avatar>
          {isActive && (
            <span className="absolute bottom-2 right-2 w-6 h-6 bg-green-500 rounded-full border-4 border-background shadow-lg" />
          )}
        </div>

        <div className="text-center space-y-1 mb-6">
          <h3 className="text-xl font-black tracking-tight">
            {student.name_english}
          </h3>
          <p className="text-sm font-bold text-primary/80 uppercase tracking-widest px-3 py-1 bg-primary/10 rounded-full inline-block">
            {student.status}
          </p>
        </div>

        <div className="grid grid-cols-1 w-full gap-3">
          <div className="flex items-center justify-between p-3 rounded-2xl bg-muted/30 group-hover:bg-muted/50 transition-colors">
            <div className="flex items-center text-sm font-medium text-muted-foreground">
              <HugeiconsIcon icon={User03Icon} className="w-4 h-4 mr-2" />{' '}
              Admission No.
            </div>
            <span className="text-sm font-bold">
              {student.admission_number}
            </span>
          </div>
          <div className="flex items-center justify-between p-3 rounded-2xl bg-muted/30 group-hover:bg-muted/50 transition-colors">
            <div className="flex items-center text-sm font-medium text-muted-foreground">
              <HugeiconsIcon icon={User03Icon} className="w-4 h-4 mr-2" />{' '}
              Gender
            </div>
            <span className="text-sm font-bold">{student.gender}</span>
          </div>
          <div className="flex items-center justify-between p-3 rounded-2xl bg-muted/30 group-hover:bg-muted/50 transition-colors">
            <div className="flex items-center text-sm font-medium text-muted-foreground">
              <HugeiconsIcon icon={Calendar03Icon} className="w-4 h-4 mr-2" />{' '}
              D.O.B
            </div>
            <span className="text-sm font-bold">{dob}</span>
          </div>
        </div>

        <div className="flex flex-col w-full mt-6 space-y-3">
          <div className="flex items-center px-4 py-3 rounded-2xl bg-background ring-1 ring-border shadow-sm group-hover:ring-primary/20 transition-all">
            <div className="p-2 rounded-xl bg-muted/50 mr-3">
              <HugeiconsIcon
                icon={MailIcon}
                className="w-4 h-4 text-muted-foreground"
              />
            </div>
            <span className="text-xs font-bold truncate flex-1">
              {student.email}
            </span>
          </div>
          <div className="flex items-center px-4 py-3 rounded-2xl bg-background ring-1 ring-border shadow-sm group-hover:ring-primary/20 transition-all">
            <div className="p-2 rounded-xl bg-muted/50 mr-3">
              <HugeiconsIcon
                icon={AiPhoneIcon}
                className="w-4 h-4 text-muted-foreground"
              />
            </div>
            <span className="text-xs font-bold">{student.phone}</span>
          </div>
        </div>
      </CardContent>
    </Card>
  )
}
