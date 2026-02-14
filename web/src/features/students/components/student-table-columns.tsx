import {
  Delete02Icon,
  MoreHorizontalIcon,
  PencilEdit01Icon,
} from '@hugeicons/core-free-icons'
import { HugeiconsIcon } from '@hugeicons/react'
import type { ColumnDef } from '@tanstack/react-table'
import type { StudentResponse } from '@/lib/api/types.gen'
import { Button } from '@/components/ui/button'
import {
  DropdownMenu,
  DropdownMenuContent,
  DropdownMenuItem,
  DropdownMenuTrigger,
} from '@/components/ui/dropdown-menu'
import { Avatar, AvatarFallback, AvatarImage } from '@/components/ui/avatar'
import { Badge } from '@/components/ui/badge'

interface GetStudentColumnsProps {
  onEdit: (student: StudentResponse) => void
  onDelete: (id: string) => void
}

export const getStudentColumns = ({
  onEdit,
  onDelete,
}: GetStudentColumnsProps): Array<ColumnDef<StudentResponse>> => [
  {
    accessorKey: 'name_english',
    header: 'Student Name',
    cell: ({ row }) => {
      const student = row.original
      return (
        <div className="flex items-center gap-3">
          <Avatar className="size-8">
            <AvatarImage
              src={
                student.photo_url ||
                undefined ||
                `https://api.dicebear.com/7.x/avataaars/svg?seed=${student.email}`
              }
              alt={student.name_english}
            />
            <AvatarFallback>
              {student.name_english
                .split(' ')
                .map((n) => n[0])
                .join('')
                .toUpperCase()}
            </AvatarFallback>
          </Avatar>
          <div className="flex flex-col">
            <span className="font-medium">{student.name_english}</span>
            <span className="text-xs text-muted-foreground">
              {student.admission_number}
            </span>
          </div>
        </div>
      )
    },
  },
  {
    accessorKey: 'email',
    header: 'Email',
    cell: ({ row }) => row.getValue('email') || '-',
  },
  {
    accessorKey: 'status',
    header: 'Status',
    cell: ({ row }) => {
      const status = row.original.status
      return (
        <Badge
          variant="secondary"
          className={
            status === 'Active'
              ? 'bg-green-500/10 text-green-600 border-green-500/20'
              : 'bg-orange-500/10 text-orange-600 border-orange-500/20'
          }
        >
          {status}
        </Badge>
      )
    },
  },
  {
    accessorKey: 'phone',
    header: 'Phone',
  },
  {
    accessorKey: 'gender',
    header: 'Gender',
  },
  {
    id: 'actions',
    cell: ({ row }) => {
      const student = row.original
      return (
        <DropdownMenu>
          <DropdownMenuTrigger
            render={
              <Button variant="ghost" size="icon" className="size-8">
                <HugeiconsIcon icon={MoreHorizontalIcon} className="size-4" />
              </Button>
            }
          />
          <DropdownMenuContent align="end" className="w-40">
            <DropdownMenuItem onClick={() => onEdit(student)}>
              <HugeiconsIcon icon={PencilEdit01Icon} className="mr-2 size-4" />
              Edit
            </DropdownMenuItem>
            <DropdownMenuItem
              className="text-destructive focus:text-destructive"
              onClick={() => onDelete(student.id)}
            >
              <HugeiconsIcon icon={Delete02Icon} className="mr-2 size-4" />
              Delete
            </DropdownMenuItem>
          </DropdownMenuContent>
        </DropdownMenu>
      )
    },
  },
]
