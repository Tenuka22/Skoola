'use client'

import * as React from 'react'
import { createFileRoute } from '@tanstack/react-router'
import {
  keepPreviousData,
  useMutation,
  useQuery,
  useQueryClient,
} from '@tanstack/react-query'
import { HugeiconsIcon } from '@hugeicons/react'
import {
  ArrowLeft01Icon,
  ArrowRight01Icon,
  Download02Icon,
  FilterIcon,
  PlusSignIcon,
  RefreshIcon,
  Search01Icon,
  Upload02Icon,
} from '@hugeicons/core-free-icons'
import { toast } from 'sonner'
import type { StudentResponse } from '@/features/students/types'
import {
  deleteStudents4D5Cba944Bd069Fdf2A0246F5Bac2855Mutation,
  getStudents9Cfb76Aa83C6A83D99Db1D6755C24Ee1Options,
  postStudents9Cfb76Aa83C6A83D99Db1D6755C24Ee1Mutation,
  putStudents4D5Cba944Bd069Fdf2A0246F5Bac2855Mutation,
} from '@/features/students/api'
import { StudentCard } from '@/features/students/components/student-card'
import { StudentModals } from '@/features/students/components/student-modals'
import { Button } from '@/components/ui/button'

import {
  DropdownMenu,
  DropdownMenuContent,
  DropdownMenuLabel,
  DropdownMenuRadioGroup,
  DropdownMenuRadioItem,
  DropdownMenuSeparator,
  DropdownMenuTrigger,
} from '@/components/ui/dropdown-menu'
import { authClient } from '@/lib/clients'

export const Route = createFileRoute('/admin/students')({
  component: StudentsPage,
})

function StudentsPage() {
  const [page, setPage] = React.useState(1)
  const [studentStatusFilter, setStudentStatusFilter] =
    React.useState<string>('all')

  const [studentToDelete, setStudentToDelete] =
    React.useState<StudentResponse | null>(null)
  const [studentToEdit, setStudentToEdit] =
    React.useState<StudentResponse | null>(null)
  const [isAddOpen, setIsAddOpen] = React.useState(false)

  const limit = 9 // 3x3 grid
  const queryClient = useQueryClient()

  const offset = (page - 1) * limit

  const { data, isLoading, error, refetch } = useQuery({
    ...getStudents9Cfb76Aa83C6A83D99Db1D6755C24Ee1Options({
      client: authClient,
      query: {
        offset,
        limit,
      },
    }),
    placeholderData: keepPreviousData,
  })

  const deleteMutation = useMutation({
    ...deleteStudents4D5Cba944Bd069Fdf2A0246F5Bac2855Mutation({
      client: authClient,
    }),
    onSuccess: () => {
      toast.success('Student removed')
      queryClient.invalidateQueries({
        queryKey: [{ _id: 'getStudents9Cfb76Aa83C6A83D99Db1D6755C24Ee1' }],
      })
      setStudentToDelete(null)
    },
    onError: (error: any) => {
      toast.error(`Failed to delete student: ${error.message}`)
    },
  })

  const addMutation = useMutation({
    ...postStudents9Cfb76Aa83C6A83D99Db1D6755C24Ee1Mutation({
      client: authClient,
    }),
    onSuccess: () => {
      toast.success('New student registered')
      queryClient.invalidateQueries({
        queryKey: [{ _id: 'getStudents9Cfb76Aa83C6A83D99Db1D6755C24Ee1' }],
      })
      setIsAddOpen(false)
    },
    onError: (error: any) => {
      toast.error(`Failed to add student: ${error.message}`)
    },
  })

  const updateMutation = useMutation({
    ...putStudents4D5Cba944Bd069Fdf2A0246F5Bac2855Mutation({
      client: authClient,
    }),
    onSuccess: () => {
      toast.success('Records updated')
      queryClient.invalidateQueries({
        queryKey: [{ _id: 'getStudents9Cfb76Aa83C6A83D99Db1D6755C24Ee1' }],
      })
      setStudentToEdit(null)
    },
    onError: (error: any) => {
      toast.error(`Failed to update student: ${error.message}`)
    },
  })

  const students = data?.students || []
  const totalStudents = data?.total_students || 0
  const totalPages = totalStudents > 0 ? Math.ceil(totalStudents / limit) : 0

  if (error) {
    return (
      <div className="p-8 text-center">
        <p className="text-red-500 mb-4">
          Error loading students: {error.message}
        </p>
        <Button onClick={() => refetch()}>Retry</Button>
      </div>
    )
  }

  return (
    <div className="p-8 space-y-8">
      <div className="flex flex-col md:flex-row justify-between items-start md:items-center gap-4">
        <div>
          <h1 className="text-3xl font-black tracking-tight">
            {totalStudents} Students
          </h1>
          <p className="text-muted-foreground font-medium">
            Manage your educational institution's student records.
          </p>
        </div>
        <div className="flex flex-wrap gap-2">
          <Button variant="outline" className="rounded-xl h-11">
            <HugeiconsIcon icon={Upload02Icon} className="w-4 h-4 mr-2" />{' '}
            Import
          </Button>
          <Button variant="outline" className="rounded-xl h-11">
            <HugeiconsIcon icon={Download02Icon} className="w-4 h-4 mr-2" />{' '}
            Export
          </Button>
          <Button
            className="rounded-xl h-11 bg-primary text-primary-foreground shadow-lg shadow-primary/20"
            onClick={() => setIsAddOpen(true)}
          >
            <HugeiconsIcon icon={PlusSignIcon} className="w-4 h-4 mr-2" /> Add
            Student
          </Button>
        </div>
      </div>

      <div className="flex flex-wrap items-center gap-2">
        <DropdownMenu>
          <DropdownMenuTrigger
            render={
              <Button
                variant="outline"
                className="h-11 rounded-xl border-none bg-muted/50 ring-1 ring-border"
              >
                <HugeiconsIcon icon={FilterIcon} className="w-4 h-4 mr-2" />
                {studentStatusFilter === 'all'
                  ? 'All Status'
                  : studentStatusFilter}
              </Button>
            }
          />
          <DropdownMenuContent align="end" className="w-56 rounded-xl p-2">
            <DropdownMenuLabel>Student Status</DropdownMenuLabel>
            <DropdownMenuSeparator />
            <DropdownMenuRadioGroup
              value={studentStatusFilter}
              onValueChange={setStudentStatusFilter}
            >
              <DropdownMenuRadioItem value="all">
                All Status
              </DropdownMenuRadioItem>
              <DropdownMenuRadioItem value="Active">
                Active
              </DropdownMenuRadioItem>
              <DropdownMenuRadioItem value="Suspended">
                Suspended
              </DropdownMenuRadioItem>
              <DropdownMenuRadioItem value="Graduated">
                Graduated
              </DropdownMenuRadioItem>
              <DropdownMenuRadioItem value="Transferred">
                Transferred
              </DropdownMenuRadioItem>
              <DropdownMenuRadioItem value="Withdrawn">
                Withdrawn
              </DropdownMenuRadioItem>
            </DropdownMenuRadioGroup>
          </DropdownMenuContent>
        </DropdownMenu>

        <Button
          variant="outline"
          size="icon"
          className="h-11 w-11 rounded-xl border-none bg-muted/50 ring-1 ring-border shadow-sm active:scale-95 transition-transform"
          onClick={() => refetch()}
        >
          <HugeiconsIcon icon={RefreshIcon} className="w-4 h-4" />
        </Button>
      </div>

      {isLoading ? (
        <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6">
          {[...Array(6)].map((_, i) => (
            <div
              key={i}
              className="h-64 rounded-3xl bg-muted/50 animate-pulse"
            />
          ))}
        </div>
      ) : students.length > 0 ? (
        <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6">
          {students.map((student: StudentResponse) => (
            <StudentCard
              key={student.id}
              student={student}
              onEdit={(s) => setStudentToEdit(s)}
              onDelete={(s) => setStudentToDelete(s)}
            />
          ))}
        </div>
      ) : (
        <div className="py-20 text-center bg-muted/20 rounded-[2.5rem] border-2 border-dashed border-border">
          <div className="mx-auto mb-4 flex size-20 items-center justify-center rounded-3xl bg-muted text-muted-foreground">
            <HugeiconsIcon icon={Search01Icon} className="size-10" />
          </div>
          <h3 className="text-xl font-bold">No students found</h3>
          <p className="text-muted-foreground">
            Try adjusting your search or filters.
          </p>
          <Button
            variant="link"
            onClick={() => {
              setStudentStatusFilter('all')
            }}
          >
            Clear all filters
          </Button>
        </div>
      )}

      <div className="flex flex-col sm:flex-row justify-between items-center gap-4 pt-4">
        <p className="text-sm font-medium text-muted-foreground">
          Showing{' '}
          <span className="text-foreground">
            {students.length > 0 ? (page - 1) * limit + 1 : 0}
          </span>{' '}
          to{' '}
          <span className="text-foreground">
            {Math.min(page * limit, totalStudents)}
          </span>{' '}
          of <span className="text-foreground">{totalStudents}</span> entries
        </p>
        <div className="flex items-center gap-2">
          <Button
            variant="outline"
            size="sm"
            className="rounded-xl h-10 px-4 font-bold border-none bg-muted/50 ring-1 ring-border disabled:opacity-30"
            onClick={() => setPage((p) => Math.max(1, p - 1))}
            disabled={page === 1}
          >
            <HugeiconsIcon icon={ArrowLeft01Icon} className="w-4 h-4 mr-2" />{' '}
            Previous
          </Button>
          <div className="flex items-center gap-1">
            {totalPages > 0 &&
              [...Array(totalPages)].map((_, i) => {
                const p = i + 1
                if (
                  totalPages > 5 &&
                  Math.abs(p - page) > 2 &&
                  p !== 1 &&
                  p !== totalPages
                ) {
                  if (p === 2 || p === totalPages - 1)
                    return <span key={p}>...</span>
                  return null
                }
                return (
                  <Button
                    key={p}
                    variant={page === p ? 'default' : 'outline'}
                    size="sm"
                    className={`w-10 h-10 rounded-xl font-bold transition-all ${page === p ? 'shadow-lg shadow-primary/20' : 'border-none bg-muted/50 ring-1 ring-border'}`}
                    onClick={() => setPage(p)}
                  >
                    {p}
                  </Button>
                )
              })}
          </div>
          <Button
            variant="outline"
            size="sm"
            className="rounded-xl h-10 px-4 font-bold border-none bg-muted/50 ring-1 ring-border disabled:opacity-30"
            onClick={() => setPage((p) => p + 1)}
            disabled={page >= totalPages}
          >
            Next{' '}
            <HugeiconsIcon icon={ArrowRight01Icon} className="w-4 h-4 ml-2" />
          </Button>
        </div>
      </div>

      <StudentModals
        studentToDelete={studentToDelete}
        setStudentToDelete={setStudentToDelete}
        onDeleteConfirm={(id) =>
          deleteMutation.mutate({ path: { student_id: id } })
        }
        isAddOpen={isAddOpen}
        setIsAddOpen={setIsAddOpen}
        onAddConfirm={(values) => addMutation.mutate({ body: values })}
        isAdding={addMutation.isPending}
        studentToEdit={studentToEdit}
        setStudentToEdit={setStudentToEdit}
        onEditConfirm={(values) =>
          studentToEdit &&
          updateMutation.mutate({
            path: { student_id: studentToEdit.id },
            body: values,
          })
        }
        isEditing={updateMutation.isPending}
      />
    </div>
  )
}
